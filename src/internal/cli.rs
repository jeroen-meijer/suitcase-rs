use crate::internal;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
#[clap(propagate_version = true)]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    /// Open the current directory's Git repository in the default browser.
    ///
    /// Optionally provide a path to open a specific directory's repository
    /// other than the current directory.
    #[clap(name = "gho")]
    GitHubOpen(internal::commands::gho::GitHubOpenOptions),
}
