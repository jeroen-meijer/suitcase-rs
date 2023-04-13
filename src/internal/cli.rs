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
    ///
    /// Command name is short for "github open"
    #[clap(name = "gho")]
    GitHubOpen(internal::commands::gho::GitHubOpenOptions),

    /// Upgrade suitcase to the latest version from cargo or from a local path.
    ///
    /// This command will download and install the latest version of suitcase
    /// from crates.io, or from a local path if it was installed from source.
    #[clap(name = "upgrade")]
    Upgrade(internal::commands::upgrade::UpgradeOptions),

    /// Run the given command for every Dart project in the current directory
    /// and its subdirectories recursively.
    ///
    /// Command name is short for "for dart"
    #[clap(name = "ford")]
    ForEveryDartProject(internal::commands::ford::ForEveryDartProjectOptions),

    /// Set the FVM version for every Flutter or Dart project in the current directory
    /// and its subdirectories recursively.
    ///
    /// Command name is short for "flutter use all"
    #[clap(name = "fua")]
    FvmUseForEveryFlutterProject(internal::commands::fua::FvmUseForEveryFlutterProjectOptions),
}

/// Base arguments that are shared across all subcommands.
#[derive(Clone, Args)]
pub struct BaseArgs {
    /// Print verbose output. Cannot be used in combination with `--quiet`.
    #[clap(short, long, global = true)]
    pub verbose: bool,

    /// Print no output. Cannot be used in combination with `--verbose`.
    #[clap(short, long, global = true)]
    pub quiet: bool,
}
