// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::config::Data;
use anyhow::{Result, anyhow};
use log::LevelFilter;
use parking_lot::RwLock;
use simplelog::{ColorChoice, CombinedLogger, ConfigBuilder, TermLogger, TerminalMode};

/// Service struct to distribute the configuration type across handlers
#[derive(Debug)]
pub struct Service {
    pub config: RwLock<Data>,
}

/// Arguments needed for the server
#[derive(Debug)]
pub struct ServerArgs {
    pub host: String,
    pub port: u16,
    pub debug: bool,
}

/// Initialize the logger
///
/// ## Arguments
///
/// * `debug` - Whether to enable debug logging
///
/// ## Errors
///
/// Returns an error if the logger could not be initialized
pub fn init_logger(debug: bool) -> Result<()> {
    let filter = if debug {
        LevelFilter::Debug
    } else {
        LevelFilter::Info
    };

    let config = ConfigBuilder::new()
        .set_time_offset_to_local()
        .expect("could not determine local time offset")
        .set_time_format_rfc3339()
        .build();

    CombinedLogger::init(vec![TermLogger::new(
        filter,
        config,
        TerminalMode::Mixed,
        ColorChoice::Auto,
    )])
    .map_err(|e| anyhow!(e))
}
