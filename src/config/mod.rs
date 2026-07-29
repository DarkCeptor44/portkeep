// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

pub mod error;

use crate::config::error::Error;
use configura::{Config, formats::JsonFormat};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use tabela::{Alignment, Cell, Color, Row};

type Result<T> = std::result::Result<T, Error>;

const CONFIG_NAME: &str = env!("CARGO_PKG_NAME");

#[derive(Debug, Serialize, Deserialize, PartialEq, Default)]
pub struct Data {
    pub ports: BTreeMap<u16, String>,
}

impl Config for Data {
    type FormatType = JsonFormat;
    type FormatContext = ();

    fn config_path_and_filename(_home_dir: &std::path::Path) -> (Option<std::path::PathBuf>, &str) {
        (None, CONFIG_NAME)
    }
}

impl Data {
    /// Add a port with a description to the config file
    ///
    /// ## Arguments
    ///
    /// * `port` - The port number to add (1-65535)
    /// * `description` - The description of the port
    ///
    /// ## Errors
    ///
    /// * [`Error::InvalidPort`] - If the port is invalid (is 0)
    /// * [`Error::EmptyDescription`] - If the description is empty
    /// * [`Error::PortAlreadyExists`] - If the port already exists
    /// * [`Error::ConfigError`] - If there is an error with saving the config file
    pub fn add_port(&mut self, port: u16, description: String) -> Result<()> {
        if port == 0 {
            return Err(Error::InvalidPort(port));
        }

        if description.trim().is_empty() {
            return Err(Error::EmptyDescription);
        }

        if self.ports.contains_key(&port) {
            return Err(Error::PortAlreadyExists(port));
        }

        self.ports.insert(port, description);
        self.save()?;

        Ok(())
    }

    /// Edit a port in the config file
    ///
    /// ## Arguments
    ///
    /// * `port` - The port number to edit (1-65535)
    /// * `description` - The new description of the port
    ///
    /// ## Errors
    ///
    /// * [`Error::InvalidPort`] - If the port is invalid (is 0)
    /// * [`Error::EmptyDescription`] - If the description is empty
    /// * [`Error::PortDoesNotExist`] - If the port does not exist
    /// * [`Error::ConfigError`] - If there is an error with saving the config file
    pub fn edit_port(&mut self, port: u16, description: String) -> Result<()> {
        if port == 0 {
            return Err(Error::InvalidPort(port));
        }

        if description.trim().is_empty() {
            return Err(Error::EmptyDescription);
        }

        if !self.ports.contains_key(&port) {
            return Err(Error::PortDoesNotExist(port));
        }

        self.ports.insert(port, description);
        self.save()?;

        Ok(())
    }

    /// Remove a port from the config file
    ///
    /// ## Arguments
    ///
    /// * `port` - The port number to remove (1-65535)
    ///
    /// ## Errors
    ///
    /// * [`Error::InvalidPort`] - If the port is invalid (is 0)
    /// * [`Error::PortDoesNotExist`] - If the port does not exist
    /// * [`Error::ConfigError`] - If there is an error with saving the config file
    pub fn remove_port(&mut self, port: u16) -> Result<()> {
        if port == 0 {
            return Err(Error::InvalidPort(port));
        }

        if !self.ports.contains_key(&port) {
            return Err(Error::PortDoesNotExist(port));
        }

        self.ports.remove(&port);
        self.save()?;

        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PortDetailResponse {
    pub port: u16,
    pub description: Option<String>,
    pub status: PortStatus,
}

impl Row for &PortDetailResponse {
    fn as_row(&self) -> Vec<Cell> {
        vec![
            Cell::new(self.port)
                .with_alignment(Alignment::Center)
                .with_color(Color::Cyan),
            Cell::new(self.description.as_deref().unwrap_or("")),
            Cell::new(self.status.is_listening)
                .with_alignment(Alignment::Center)
                .with_color(if self.status.is_listening {
                    Color::Green
                } else {
                    Color::Red
                }),
            Cell::new(self.status.pid.map_or(-1, i64::from))
                .with_alignment(Alignment::Center)
                .with_color(Color::Cyan),
            Cell::new(self.status.process_name.as_deref().unwrap_or("")),
        ]
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PortStatus {
    pub is_listening: bool,
    pub pid: Option<u32>,
    pub process_name: Option<String>,
}
