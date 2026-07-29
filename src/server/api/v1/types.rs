// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::config::PortDetailResponse;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Deserialize, ToSchema, Debug)]
pub struct AddPortRequest {
    /// Port number
    pub port: u16,

    /// Port description
    pub description: String,
}

#[derive(Serialize, ToSchema)]
pub struct HealthResponse<'a> {
    /// Server status
    pub status: &'a str,

    /// Server version
    pub version: &'a str,

    /// Server time
    pub server_time: &'a str,
}

#[derive(Serialize, ToSchema)]
pub struct PortResponse {
    /// Port number
    pub port: u16,

    /// Port description
    pub description: Option<String>,

    /// Whether the port is listening
    pub is_listening: bool,

    /// PID of the process listening on the port
    pub pid: Option<u32>,

    /// Name of the process listening on the port
    pub process_name: Option<String>,
}

impl From<PortDetailResponse> for PortResponse {
    fn from(p: PortDetailResponse) -> Self {
        Self {
            port: p.port,
            description: p.description,
            is_listening: p.status.is_listening,
            pid: p.status.pid,
            process_name: p.status.process_name,
        }
    }
}
