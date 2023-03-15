pub mod internal;

use anyhow::Context;
use clap::Parser;
use internal::{
    cli::{Cli, Command},
    commands, shell,
};
use std::{env, ffi::OsString, path::PathBuf};

/// Entrypoint for the 'suitcase' CLI.
pub fn run(args: Option<Vec<OsString>>) -> anyhow::Result<()> {
    let cli = if let Some(args) = args {
        Cli::parse_from(args)
    } else {
        Cli::parse()
    };

    let shell = shell::Shell::new();

    return match &cli.command {
        Command::GitHubOpen(options) => {
            commands::git_hub_open(&shell, &options).context("trying to open git repository")
        }
    };
}

/// Called by aliases in the `src/bin` directory to run the CLI.
///
/// Every alias in the `src/bin` directory is a shortcut to a subcommand.
/// This function will parse the file path of the alias and use it to
/// extract the subcommand name. It will then pass the subcommand name
/// as an argument to the `run` function, as well as any other arguments
/// passed to the alias.
///
/// * `command_name` - Aliases may provide a custom command name override.
///   This is useful for aliases that are named differently than the
///   subcommand they represent, or for making multiple aliases for the
///   same subcommand. When `None`, the alias file name is used.
pub fn run_from_alias(command_name: Option<String>) -> anyhow::Result<()> {
    let mut args = vec![env!("CARGO_PKG_NAME").into()];
    let mut raw_args = env::args_os();
    let file_path = PathBuf::from(raw_args.next().unwrap());
    let subcommand: String =
        command_name.unwrap_or(file_path.file_stem().unwrap().to_string_lossy().into());
    args.push(subcommand.into());
    args.extend(raw_args);
    run(Some(args))
}
