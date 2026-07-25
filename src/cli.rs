// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::{
    App, AppArgs,
    config::{Data, error::Error},
    utils::{InquireExt, validate_port, validate_text},
};
use anyhow::{Context, Result};
use inquire::{Confirm, CustomType, Select, Text};
use std::fmt::Display;
use tabela::{Alignment, Cell, CellStyle, Row, Table};

#[derive(Debug)]
struct PortEntry {
    port: u16,
    description: String,
}

impl Display for PortEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.description.trim().is_empty() {
            write!(f, "{}", self.port)
        } else {
            write!(f, "{} ({})", self.port, self.description)
        }
    }
}

pub fn handle_cli(args: App, config: &mut Data) -> Result<()> {
    match args.command {
        AppArgs::Add {
            port,
            description,
            confirm,
        } => {
            add_port(config, port, description, confirm).context("Failed to add port")?;
        }
        AppArgs::List { reverse } => list_ports(config, reverse).context("Failed to list ports")?,
        AppArgs::Remove { port, confirm } => {
            remove_port(config, port, confirm).context("Failed to remove port")?;
        }
        AppArgs::Serve { .. } => unreachable!("Serve command must be run at the root"),
    }

    Ok(())
}

fn add_port(
    config: &mut Data,
    given_port: Option<u16>,
    given_description: Option<String>,
    confirm: bool,
) -> Result<()> {
    let Some(entry) =
        prompt_port(config, given_port, given_description).context("Failed to prompt a port")?
    else {
        return Ok(());
    };

    let port = entry.port;
    let description = entry.description;

    if confirm
        || Confirm::new("Do you want to save this port?")
            .with_default(false)
            .with_help_message(&format!("{port} ({description})"))
            .prompt()
            .unwrap_or(false)
    {
        config
            .add_port(port, description.clone())
            .context("Failed to add port to config file")?;
        println!("Added port {port} with description {description}");
    }

    Ok(())
}

fn list_ports(config: &Data, reverse: bool) -> Result<()> {
    if config.ports.is_empty() {
        println!("No ports found");
        return Ok(());
    }

    let mut ports: Vec<PortEntry> = config
        .ports
        .iter()
        .map(|(k, v)| PortEntry {
            port: *k,
            description: v.clone(),
        })
        .collect();

    if reverse {
        ports.reverse();
    }

    // tabela needs &[&[T]]
    let ports_ref: Vec<&PortEntry> = ports.iter().collect();
    let table = Table::new(&ports_ref)
        .with_header(&["Port", "Description"], None, Some(CellStyle::Bold), None)
        .with_separator("  ");

    println!("{}", table.format().context("Failed to format port list")?);
    Ok(())
}

fn remove_port(config: &mut Data, given_port: Option<u16>, confirm: bool) -> Result<()> {
    if config.ports.is_empty() {
        println!("No ports found");
        return Ok(());
    }

    let ports: Vec<PortEntry> = config
        .ports
        .iter()
        .map(|(k, v)| PortEntry {
            port: *k,
            description: v.clone(),
        })
        .collect();
    let entry = if let Some(p) = given_port {
        PortEntry {
            port: p,
            description: String::new(),
        }
    } else {
        let Some(p) = Select::new("Choose a port to remove", ports).prompt_ext()? else {
            return Ok(());
        };

        p
    };

    if !config.ports.contains_key(&entry.port) {
        return Err(Error::PortDoesNotExist(entry.port).into());
    }

    if confirm
        || Confirm::new("Do you want to save this port?")
            .with_default(false)
            .with_help_message(&format!("{entry}"))
            .prompt()
            .unwrap_or(false)
    {
        config
            .remove_port(entry.port)
            .context("Failed to remove port from config file")?;
        println!("Removed port {entry}");
    }

    Ok(())
}

// UTILS

fn prompt_port(
    config: &Data,
    default_port: Option<u16>,
    default_desc: Option<String>,
) -> Result<Option<PortEntry>> {
    let port = if let Some(p) = default_port {
        p
    } else {
        let Some(p) = CustomType::<u16>::new("Port number:")
            .with_validator(validate_port)
            .prompt_ext()?
        else {
            return Ok(None);
        };

        p
    };

    if config.ports.contains_key(&port) {
        return Err(Error::PortAlreadyExists(port).into());
    }

    let desc = if let Some(d) = default_desc {
        d
    } else {
        let Some(d) = Text::new("Description:")
            .with_validator(validate_text)
            .prompt_ext()?
        else {
            return Ok(None);
        };

        d
    };

    Ok(Some(PortEntry {
        port,
        description: desc,
    }))
}

impl Row for &PortEntry {
    fn as_row(&self) -> Vec<Cell> {
        vec![
            Cell::new(self.port).with_alignment(Alignment::Center),
            Cell::new(&self.description),
        ]
    }
}
