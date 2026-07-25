// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

mod v1;

use crate::server::{api::v1::ApiDocV1, utils::Service};
use axum::{
    Router,
    body::Body,
    http::{Response, StatusCode, Uri, header},
    response::IntoResponse,
};
use std::sync::Arc;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

#[derive(rust_embed::RustEmbed)]
#[folder = "dist"]
struct Assets;

pub fn routes(_service: &Service) -> Router<Arc<Service>> {
    let r = Router::new().nest("/api/v1", v1::routes()).merge(
        SwaggerUi::new("/api/v1/docs").url("/api-docs/v1/openapi.json", ApiDocV1::openapi()),
    );

    if cfg!(debug_assertions) {
        use std::path::PathBuf;
        use tower_http::services::{ServeDir, ServeFile};
        let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("dist");
        let index_path = path.join("index.html");
        let serve_dir = ServeDir::new(path).not_found_service(ServeFile::new(index_path));

        r.fallback_service(serve_dir)
    } else {
        r.fallback(static_asset_handler)
    }
}

async fn static_asset_handler(uri: Uri) -> impl IntoResponse {
    let path = uri.path().trim_start_matches('/');

    if let Some(file) = Assets::get(path) {
        let mime_type = mime_guess::from_path(path).first_or_octet_stream();

        return Response::builder()
            .status(StatusCode::OK)
            .header(header::CONTENT_TYPE, mime_type.as_ref())
            .body(Body::from(file.data))
            .unwrap();
    }

    if let Some(index) = Assets::get("index.html") {
        return Response::builder()
            .status(StatusCode::OK)
            .header(header::CONTENT_TYPE, "text/html")
            .body(Body::from(index.data))
            .unwrap();
    }

    Response::builder()
        .status(StatusCode::NOT_FOUND)
        .body(Body::from("Build directory missing or empty"))
        .unwrap()
}
