// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use serde::Serialize;
use utoipa::ToSchema;

#[derive(Serialize, ToSchema)]
pub struct HealthResponse<'a> {
    /// Server status
    pub status: &'a str,

    /// Server version
    pub version: &'a str,

    /// Server time
    pub server_time: &'a str,
}
