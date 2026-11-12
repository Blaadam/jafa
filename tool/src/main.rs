mod cli;
mod code_workspace;
mod commands;
mod exec;
mod log;
mod mantle;
mod project;
mod prompt;
mod scaffold;
mod workspace_file;

use anyhow::Result;
use clap::Parser;
use cli::{Cli, Command};

fn run(cli: Cli) -> Result<()> {
    match cli.command {
        Command::Build(args) => commands::build::run(&args),
        Command::Obscure(args) => commands::obscure::run(&args),
        Command::Init { target, yes, common, name } => {
            commands::init::run(&target, yes, common, name)
        }
        Command::New { name, target } => commands::new::run(&name, &target),
        Command::Open(args) => commands::start::run(&args),
        Command::Pull(args) => commands::pull::run(&args),
        Command::Serve(args) => commands::serve::run(&args),
        Command::Sourcemap(args) => commands::sourcemap::run(&args),
        Command::Dev(args) => commands::dev::run(&args),
        Command::Deploy(args) => commands::deploy::run(&args),
        Command::NewUniverse { name, projects, environment, target } => {
            commands::universe::new(&name, &projects, &environment, &target)
        }
        Command::DeployUniverse { name, environment, target, level } => {
            commands::universe::deploy(&name, &environment, &target, level)
        }
    }
}

fn main() {
    let cli = Cli::parse();

    if let Err(err) = run(cli) {
        log::error(format!("{err:#}"));
        std::process::exit(1);
    }
}
