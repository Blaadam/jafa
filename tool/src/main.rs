// TODO: remove once the commands use the helpers
#![allow(dead_code)]

mod cli;
mod commands;
mod exec;
mod log;
mod project;

use anyhow::{Result, bail};
use clap::Parser;
use cli::{Cli, Command};

fn run(cli: Cli) -> Result<()> {
    match cli.command {
        Command::Build(args) => commands::build::run(&args),
        Command::Sourcemap(args) => commands::sourcemap::run(&args),
        Command::Init { .. }
        | Command::New { .. }
        | Command::Serve(_)
        | Command::Open(_)
        | Command::Dev(_)
        | Command::Deploy(_)
        | Command::Pull(_) => bail!("this command hasn't been rebuilt in Rust yet"),
    }
}

fn main() {
    let cli = Cli::parse();

    if let Err(err) = run(cli) {
        log::error(format!("{err:#}"));
        std::process::exit(1);
    }
}
