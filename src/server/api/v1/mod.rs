// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

mod types;

use crate::{
    VERSION,
    scanner::scan_ports,
    server::{
        api::v1::types::{HealthResponse, PortResponse},
        utils::Service,
    },
};
use axum::{Json, Router, extract::State, http::StatusCode, response::IntoResponse, routing::get};
use chrono::Local;
use log::debug;
use std::sync::Arc;
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    info(title = "PortKeep API", version = "1.0.0"),
    paths(all_ports, health),
    components(schemas(HealthResponse, PortResponse))
)]
pub struct ApiDocV1;

pub fn routes() -> Router<Arc<Service>> {
    Router::new()
        .route("/health", get(health))
        .route("/ports", get(all_ports))
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
