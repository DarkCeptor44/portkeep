// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#![forbid(unsafe_code)]
#![warn(clippy::pedantic)]

mod cli;
mod config;
mod utils;

use crate::{cli::handle_cli, config::Data};
use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use configura::load_config;
use std::process::exit;

pub const NAME: &str = env!("CARGO_BIN_NAME");
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Debug, Parser)]
#[command(version, about, long_about = None, propagate_version = true)]
pub struct App {
    #[command(subcommand)]
    command: AppArgs,
}

#[derive(Debug, Subcommand)]
enum AppArgs {
    #[command(about = "Add a port", aliases = ["a"])]
    Add {
        #[arg(help = "Port number")]
        port: Option<u16>,

        #[arg(help = "Port description")]
        description: Option<String>,
    },

    #[command(about = "List ports", aliases = ["ls"])]
    List {
        #[arg(short, long, help = "Reverse order")]
        reverse: bool,
    },

    #[command(about = "Serve portkeep")]
    Serve,
}

fn main() {
    if let Err(e) = main_impl() {
        eprintln!("{NAME}: {e:?}");
        exit(1);
    }
}

fn main_impl() -> Result<()> {
    let args = App::parse();
    let mut config: Data = load_config().context("Failed to load config file")?;

    #[allow(clippy::match_wildcard_for_single_variants)]
    match args.command {
        AppArgs::Serve => todo!(),
        _ => handle_cli(args, &mut config)?,
    }

    Ok(())
}
