// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

mod types;

use crate::{
    VERSION,
    server::{api::v1::types::HealthResponse, utils::Service},
};
use axum::{Json, Router, http::StatusCode, response::IntoResponse, routing::get};
use chrono::Local;
use log::debug;
use std::sync::Arc;
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    info(title = "PortKeep API", version = "1.0.0"),
    paths(health),
    components(schemas(HealthResponse))
)]
pub struct ApiDocV1;

pub fn routes() -> Router<Arc<Service>> {
    Router::new().route("/health", get(health))
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
