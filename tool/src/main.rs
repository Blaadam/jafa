// TODO: remove once the commands use the helpers
#![allow(dead_code)]

mod cli;
mod commands;
mod exec;
mod log;
mod project;
mod prompt;
mod scaffold;
mod workspace_file;

use anyhow::{Result, bail};
use clap::Parser;
use cli::{Cli, Command};

fn run(cli: Cli) -> Result<()> {
    match cli.command {
        Command::Build(args) => commands::build::run(&args),
        Command::Init { target, yes } => commands::init::run(&target, yes),
        Command::New { name, target } => commands::new::run(&name, &target),
        Command::Open(args) => commands::start::run(&args),
        Command::Pull(args) => commands::pull::run(&args),
        Command::Serve(args) => commands::serve::run(&args),
        Command::Sourcemap(args) => commands::sourcemap::run(&args),
        Command::Dev(args) => commands::dev::run(&args),
        Command::Deploy(_) => bail!("this command hasn't been rebuilt in Rust yet"),
    }
}

fn main() {
    let cli = Cli::parse();

    if let Err(err) = run(cli) {
        log::error(format!("{err:#}"));
        std::process::exit(1);
    }
}
