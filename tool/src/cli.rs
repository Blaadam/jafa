use clap::{Args, Parser, Subcommand};

#[derive(Parser)]
#[command(name = "jafa", version, about = "A CLI tool for Roblox repository management")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Args)]
pub struct ProjectArgs {
    /// Project name, not needed with --all
    pub name: Option<String>,

    /// Workspace directory [default: .]
    pub target: Option<String>,

    /// Run on every project in the workspace
    #[arg(short, long)]
    pub all: bool,
}

#[derive(Args)]
pub struct SingleArgs {
    /// Project name
    pub name: String,

    /// Workspace directory
    #[arg(default_value = ".")]
    pub target: String,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, clap::ValueEnum)]
pub enum ObscureLevel {
    /// Strip comments and collapse whitespace, keep names as-is
    Minify,
    /// Minify and rename local variables and function parameters
    Rename,
    /// Rename, plus strip type annotations and fold/drop dead code
    Full,
}

#[derive(Args)]
pub struct ObscureArgs {
    #[command(flatten)]
    pub project: ProjectArgs,

    /// How aggressively to obfuscate the build
    #[arg(short, long, value_enum, default_value_t = ObscureLevel::Rename)]
    pub level: ObscureLevel,
}

#[derive(Args)]
pub struct DeployArgs {
    #[command(flatten)]
    pub project: ProjectArgs,

    /// Mantle environment to deploy
    #[arg(short, long, default_value = "dev")]
    pub environment: String,

    /// Roblox group ID to deploy under, instead of your account
    #[arg(short, long = "groupid")]
    pub group_id: Option<u64>,

    /// Payments mode: owner, personal or group
    #[arg(short, long)]
    pub payments: Option<String>,

    /// Name of the deployed place
    #[arg(short = 'n', long = "name")]
    pub place_name: Option<String>,

    /// Obfuscate scripts before deploying, at this level, instead of a plain build
    #[arg(short, long, value_enum)]
    pub level: Option<ObscureLevel>,
}

#[derive(Subcommand)]
pub enum Command {
    /// Create a new workspace
    Init {
        /// Directory to create the workspace in
        #[arg(default_value = ".")]
        target: String,

        /// Skip the questions and use the defaults
        #[arg(short, long)]
        yes: bool,

        /// Also create a shared `common` project
        #[arg(long)]
        common: bool,

        /// Name of the first project
        #[arg(short, long)]
        name: Option<String>,
    },
    /// Create a new project in a workspace
    New {
        /// Project name
        name: String,

        /// Workspace directory
        #[arg(default_value = ".")]
        target: String,
    },
    /// Build a project with Rojo
    Build(ProjectArgs),
    /// Build a project with its scripts obfuscated, to make a leaked copy harder to read
    Obscure(ObscureArgs),
    /// Generate a project's sourcemap.json
    Sourcemap(ProjectArgs),
    /// Serve a project for live sync with Roblox Studio
    Serve(SingleArgs),
    /// Open a built project in Roblox Studio
    Open(ProjectArgs),
    /// Build, open and serve a project
    Dev(SingleArgs),
    /// Build a project and deploy it with Mantle
    Deploy(DeployArgs),
    /// Pull published Roblox assets into a project
    Pull(ProjectArgs),
    /// Create a Mantle config for one experience made of several projects
    NewUniverse {
        /// Universe name
        name: String,

        /// Projects to include, the first is the start place [default: all but common]
        projects: Vec<String>,

        /// Environment to create the config for
        #[arg(short, long, default_value = "dev")]
        environment: String,

        /// Workspace directory
        #[arg(short, long, default_value = ".")]
        target: String,
    },
    /// Build a universe's projects and deploy it with Mantle
    DeployUniverse {
        /// Universe name
        name: String,

        /// Mantle environment to deploy
        #[arg(short, long, default_value = "dev")]
        environment: String,

        /// Workspace directory
        #[arg(short, long, default_value = ".")]
        target: String,

        /// Obfuscate scripts before deploying, at this level, instead of a plain build
        #[arg(short, long, value_enum)]
        level: Option<ObscureLevel>,
    },
}
