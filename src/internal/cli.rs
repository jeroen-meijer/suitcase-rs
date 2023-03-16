use crate::internal;
use clap::{Args, Parser, Subcommand};

/// A set of personal CLI tools to automate common tasks in software development
/// (including Rust, Dart, and Flutter), written in Rust.
#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
#[clap(propagate_version = true)]
pub struct Cli {
    #[clap(subcommand)]
    /// The command to run.
    pub command: Command,

    /// Base arguments that are shared across all subcommands.
    #[clap(flatten)]
    pub base_args: BaseArgs,
}

/// A single suitcase subcommand to run.
#[derive(Subcommand)]
pub enum Command {
    /// Open the current directory's Git repository in the default browser.
    ///
    /// Optionally provide a path to open a specific directory's repository
    /// other than the current directory.
    #[clap(name = "gho")]
    GitHubOpen(internal::commands::gho::GitHubOpenOptions),

    /// Update suitcase to the latest version from GitHub.
    ///
    /// This command will download and install the latest version of suitcase
    /// from the GitHub repository (https://github.com/jeroen-meijer/suitcase-rs)
    /// using `cargo install`.
    #[clap(name = "update")]
    Update,
}

/// Base arguments that are shared across all subcommands.
#[derive(Clone, Args)]
pub struct BaseArgs {
    /// Print verbose output.
    #[clap(short, long, global = true)]
    pub verbose: bool,
}
