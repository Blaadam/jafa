use clap::{Args, Parser, Subcommand};

#[derive(Parser)]
#[command(name = "jafa", version, about = "A CLI tool for Roblox repository management")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Args)]
pub struct ProjectArgs {
    pub name: Option<String>,

    pub target: Option<String>,

    #[arg(short, long)]
    pub all: bool,
}

#[derive(Args)]
pub struct SingleArgs {
    pub name: String,

    #[arg(default_value = ".")]
    pub target: String,
}

#[derive(Args)]
pub struct DeployArgs {
    #[command(flatten)]
    pub project: ProjectArgs,

    #[arg(short, long, default_value = "dev")]
    pub environment: String,

    #[arg(short, long = "groupid")]
    pub group_id: Option<String>,

    #[arg(short, long)]
    pub payments: Option<String>,

    #[arg(short = 'n', long = "name")]
    pub place_name: Option<String>,
}

#[derive(Subcommand)]
pub enum Command {
    Init {
        #[arg(default_value = ".")]
        target: String,

        #[arg(short, long)]
        yes: bool,

        #[arg(long)]
        common: bool,
    },
    New {
        name: String,
        #[arg(default_value = ".")]
        target: String,
    },
    Build(ProjectArgs),
    Sourcemap(ProjectArgs),
    Serve(SingleArgs),
    Open(ProjectArgs),
    Dev(SingleArgs),
    Deploy(DeployArgs),
    Pull(ProjectArgs),
}
