// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

/// Errors that can occur when working with the config file
#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum Error {
    #[error("Config error: {0}")]
    ConfigError(#[from] configura::errors::ConfigError),

    #[error("Description cannot be empty")]
    EmptyDescription,

    #[error("Invalid port: {0}")]
    InvalidPort(u16),

    #[error("Port already exists: {0}")]
    PortAlreadyExists(u16),

    #[error("Port does not exist: {0}")]
    PortDoesNotExist(u16),
}
