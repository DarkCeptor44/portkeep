// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#![forbid(unsafe_code)]
#![warn(clippy::pedantic)]

use clap::{Parser, Subcommand};

pub const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Debug, Parser)]
#[command(version, about, long_about = None, propagate_version = true)]
pub struct App {
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    #[command(about = "Serve portkeep")]
    Serve,
}

fn main() {
    let args = App::parse();

    match args.command {
        Command::Serve => todo!(),
    }
}
