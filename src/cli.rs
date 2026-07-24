use crate::{
    App, AppArgs,
    config::{Data, error::Error},
    utils::{InquireExt, validate_port, validate_text},
};
use anyhow::{Context, Result};
use inquire::{Confirm, CustomType, Text};
use tabela::{Alignment, Cell, CellStyle, Row, Table};

#[derive(Debug)]
struct PortEntry {
    port: u16,
    description: String,
}

pub fn handle_cli(args: App, config: &mut Data) -> Result<()> {
    match args.command {
        AppArgs::Add { port, description } => {
            add_port(config, port, description).context("Failed to add port")?;
        }
        AppArgs::List { reverse } => list_ports(config, reverse).context("Failed to list ports")?,
        AppArgs::Serve => unreachable!("Serve command must be run at the root"),
    }

    Ok(())
}

fn add_port(
    config: &mut Data,
    given_port: Option<u16>,
    given_description: Option<String>,
) -> Result<()> {
    let Some(entry) =
        prompt_port(config, given_port, given_description).context("Failed to prompt a port")?
    else {
        return Ok(());
    };

    let port = entry.port;
    let description = entry.description;

    if Confirm::new("Do you want to save this port?")
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
