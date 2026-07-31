// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

mod types;
mod utils;

use crate::{
    VERSION,
    scanner::scan_ports,
    server::{
        api::v1::{
            types::{AddPortRequest, HealthResponse, PortResponse},
            utils::validate_port,
        },
        utils::Service,
    },
};
use axum::{
    Json, Router,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{delete, get, post, put},
};
use chrono::Local;
use configura::Config;
use log::debug;
use std::sync::Arc;
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    info(title = "PortKeep API", version = "1.0.0"),
    paths(add_port, all_ports, delete_port, edit_port, health),
    components(schemas(AddPortRequest, HealthResponse, PortResponse))
)]
pub struct ApiDocV1;

pub fn routes() -> Router<Arc<Service>> {
    Router::new()
        .route("/health", get(health))
        .route("/ports", get(all_ports))
        .route("/port/{port}", delete(delete_port))
        .route("/port", post(add_port))
        .route("/port", put(edit_port))
}

/// Add a new port
#[utoipa::path(post, path = "/api/v1/port", request_body(content = AddPortRequest, description = "The port object"), responses(
    (status = 200, description = "Successfully added port", body = String, example = "ok"),
    (status = 400, description = "Port is invalid or description is empty", body = String, example = "Invalid port"),
    (status = 409, description = "Port already exists", body = String, example = "port already exists"),
    (status = 500, description = "Internal server error", body = String, example = "Internal server error"),
))]
pub async fn add_port(
    State(service): State<Arc<Service>>,
    Json(payload): Json<AddPortRequest>,
) -> impl IntoResponse {
    debug!("adding port={payload:?}");

    let (port, desc) = match validate_port(payload) {
        Ok(p) => p,
        Err(e) => return (StatusCode::BAD_REQUEST, e).into_response(),
    };

    if service.config.read().ports.contains_key(&port) {
        return (StatusCode::CONFLICT, "port already exists").into_response();
    }

    match service.config.write().add_port(port, desc) {
        Ok(()) => (StatusCode::OK, "ok".to_string()).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}

/// Get all ports
#[utoipa::path(get, path = "/api/v1/ports", responses(
    (status = 200, description = "List of ports", body = Vec<PortResponse>),
))]
pub async fn all_ports(State(service): State<Arc<Service>>) -> impl IntoResponse {
    debug!("getting all ports");

    let lock = service.config.read();
    let registered = &lock.ports;
    let ports: Vec<PortResponse> = scan_ports(registered)
        .into_iter()
        .map(std::convert::Into::into)
        .collect();

    (StatusCode::OK, Json(ports)).into_response()
}

/// Delete a port
#[utoipa::path(delete, path = "/api/v1/port/{port}", responses(
    (status = 200, description = "Port was deleted", body = String, example = "ok"),
    (status = 400, description = "Port is invalid (not 1-65535)", body = String, example = "invalid port"),
    (status = 404, description = "Port was not found", body = String, example = "port not found"),
    (status = 500, description = "Internal server error", body = String, example = "Internal server error"),
))]
pub async fn delete_port(
    State(service): State<Arc<Service>>,
    Path(port): Path<u16>,
) -> impl IntoResponse {
    debug!("deleting port {port}");

    if port == 0 {
        return (StatusCode::BAD_REQUEST, "invalid port".to_string()).into_response();
    }

    let mut lock = service.config.write();
    match lock.ports.remove(&port) {
        Some(_) => (),
        None => return (StatusCode::NOT_FOUND, "port not found".to_string()).into_response(),
    }

    match lock.save() {
        Ok(()) => (StatusCode::OK, "ok".to_string()).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}

/// Edit a port
#[utoipa::path(put, path = "/api/v1/port", request_body(content = AddPortRequest, description = "The port object"), responses(
    (status = 200, description = "Successfully added port", body = String, example = "ok"),
    (status = 204, description = "No changes were made", body = String, example = "no changes"),
    (status = 400, description = "Port is invalid or description is empty", body = String, example = "Invalid port"),
    (status = 500, description = "Internal server error", body = String, example = "Internal server error"),
))]
pub async fn edit_port(
    State(service): State<Arc<Service>>,
    Json(payload): Json<AddPortRequest>,
) -> impl IntoResponse {
    debug!("editing port={payload:?}");

    let (port, desc) = match validate_port(payload) {
        Ok(p) => p,
        Err(e) => return (StatusCode::BAD_REQUEST, e).into_response(),
    };

    if let Some(current_desc) = service.config.read().ports.get(&port) {
        if current_desc.eq_ignore_ascii_case(&desc) {
            return (StatusCode::NO_CONTENT, "no changes".to_string()).into_response();
        }
    }

    match service.config.write().edit_port(port, desc) {
        Ok(()) => (StatusCode::OK, "ok".to_string()).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}

/// Health check
#[utoipa::path(get, path = "/api/v1/health", responses(
      (status = 200, description = "Health check successful", body = HealthResponse, example = json!({"status":"ok","version":"1.0.0","server_time":"2
  026-03-31T11:34:49.810125500-03:00"})),
  ))]
async fn health() -> impl IntoResponse {
    let now = Local::now().to_rfc3339();
    debug!("health check at {now}");

    (
        StatusCode::OK,
        Json(HealthResponse {
            status: "ok",
            version: VERSION,
            server_time: &now,
        }),
    )
        .into_response()
}
