pub mod error;

use crate::config::error::Error;
use configura::{Config, formats::JsonFormat};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

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
