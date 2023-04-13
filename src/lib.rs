pub mod internal;

use anyhow::Context;
use clap::Parser;
use internal::{
    cli::{Cli, Command},
    commands::{self, InternalCommandOptions},
    constants::{PACKAGE_NAME, PACKAGE_VERSION},
    shell,
};
use log::debug;
use std::{env, ffi::OsString, path::PathBuf};

/// Entrypoint for the 'suitcase' CLI.
pub fn run(args: Option<Vec<OsString>>) -> anyhow::Result<()> {
    let args = args.unwrap_or_else(|| env::args_os().collect());
    let cli = Cli::parse_from(&args);
    let shell = &shell::Shell::new();
    let base_args = &cli.base_args;

    if base_args.quiet && base_args.verbose {
        anyhow::bail!("--quiet and --verbose cannot be used together");
    }

    let log_level = match (base_args.quiet, base_args.verbose) {
        (true, _) => log::LevelFilter::Off,
        (_, true) => log::LevelFilter::Trace,
        _ => log::LevelFilter::Info,
    };

    env_logger::builder()
        .filter_level(log_level)
        .format_timestamp(None)
        .format_module_path(false)
        .format_target(false)
        .format_indent(None)
        .init();

    debug!("{} v{}", PACKAGE_NAME, PACKAGE_VERSION);
    debug!("verbose mode enabled");
    debug!("received args: {:?}", args);

    return match &cli.command {
        Command::GitHubOpen(options) => commands::git_hub_open(InternalCommandOptions {
            shell,
            base_args,
            options,
        })
        .context("trying to open git repository"),
        Command::Upgrade(options) => commands::upgrade(InternalCommandOptions {
            shell,
            base_args,
            options,
        })
        .context(format!("trying to upgrade {}", PACKAGE_NAME)),
        Command::ForEveryDartProject(options) => {
            commands::for_every_dart_project(InternalCommandOptions {
                shell,
                base_args,
                options,
            })
            .context("trying to run a command for every Dart project")
        }
        Command::FvmUseForEveryFlutterProject(options) => {
            commands::fvm_use_for_every_flutter_project(InternalCommandOptions {
                shell,
                base_args,
                options,
            })
            .context("trying to set an FVM version for every Flutter project")
        }
    };
}

/// Called by aliases in the `src/bin` directory to run the CLI.
///
/// Every alias in the `src/bin` directory is a shortcut to a subcommand
/// (except for `upgrade` which is a special case).
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
    let mut args = vec![PACKAGE_NAME.into()];
    let mut raw_args = env::args_os();
    let file_path = PathBuf::from(raw_args.next().unwrap());
    let subcommand: String =
        command_name.unwrap_or(file_path.file_stem().unwrap().to_string_lossy().into());
    args.push(subcommand.into());
    args.extend(raw_args);
    run(Some(args))
}
