use crate::{
    exec_on,
    internal::{
        shell::ShellError,
        utils::{
            dart::{DartProjectMetadata, DartShell},
            directory_utils::DirectoryUtils,
        },
    },
    progress,
};
use anyhow::Context;
use clap::Args;
use log::info;
use std::{ops::Deref, path::PathBuf};
use thiserror::Error;

use super::InternalCommandOptions;

#[derive(Args, Debug)]
pub struct ForEveryDartProjectOptions {
    /// The command to run on each Dart project.
    #[arg()]
    command: Vec<String>,

    /// The path from which to search for Dart projects.
    #[arg(default_value = ".", short, long)]
    path: PathBuf,

    /// Include Flutter projects when searching for Dart projects to run the command on.
    #[arg(default_value = "true", short, long)]
    include_flutter_projects: bool,

    /// Exit the process immediately if any of the commands run on the Dart projects fail.
    #[arg(default_value = "false", short, long)]
    fail_fast: bool,

    /// Show the output of the command run on each Dart project.
    #[arg(default_value = "false", short, long)]
    show_output: bool,
}

#[derive(Error, Debug)]
pub enum ForEveryDartProjectError {
    /// An error that occurred when trying to execute a command in one or more Dart projects.
    #[error("error while executing command '{command}' for one or more projects: {errors:?}")]
    CommandExecutionError {
        command: String,
        errors: Vec<(String, ShellError)>,
    },
}

pub fn for_every_dart_project(
    InternalCommandOptions {
        shell,
        base_args: _,
        options,
    }: InternalCommandOptions<ForEveryDartProjectOptions>,
) -> anyhow::Result<()> {
    let command = options.command.join(" ");
    let path = options.path.clone();
    let mut dir_utils = DirectoryUtils::new();

    dir_utils
        .pushd(&path)
        .context("trying to navigate to provided path")?;

    let dart_shell = DartShell::new(shell);

    let mut projects = progress!("Finding Dart projects", {
        dart_shell.find_dart_projects(None).context(format!(
            "trying to find Dart projects in path '{}'",
            path.display()
        ))
    })?;

    if !options.include_flutter_projects {
        projects.retain(|project| !project.is_flutter_project);
        info!("Found {} Dart (non-Flutter) projects", projects.len());
    } else {
        info!("Found {} Dart and Flutter projects", projects.len());
    }

    if projects.is_empty() {
        info!("No projects found");
        return Ok(());
    }

    let mut errors: Vec<(&DartProjectMetadata, ShellError)> = vec![];

    for project in projects.iter() {
        dir_utils.pushd(&project.path).context(format!(
            "trying to navigate to project path '{}'",
            project.path.display()
        ))?;

        let result = progress!(
            format!(
                "Running command in '{}' ('{}')",
                project.name,
                project.path.display()
            )
            .as_str(),
            exec_on!(shell, "bash", "-c", &command)
        );

        if let Err(error) = &result {
            let error = error.deref().clone();
            if options.fail_fast {
                return Err(error).context(format!(
                    "trying to run command '{}' on project '{}'",
                    command, project.name
                ));
            } else {
                errors.push((project, error.clone()));
            }
        }

        if options.show_output {
            let output = &result.map_or_else(
                |err| match err {
                    crate::internal::shell::ShellError::HostProcessExecutionFailure {
                        command: _,
                        args: _,
                        status: _,
                        stdout,
                        stderr,
                    } => format!("{}\n---\n{}", stdout, stderr),
                    crate::internal::shell::ShellError::ShellStartFailure {
                        command: _,
                        args: _,
                        error,
                    } => error.to_string(),
                },
                |output| output.stdout,
            );

            info!("Output:\n{}\n---", output);
        }
    }

    if !errors.is_empty() {
        return Err(ForEveryDartProjectError::CommandExecutionError {
            command,
            errors: errors
                .into_iter()
                .map(|(project, error)| (project.name.clone(), error))
                .collect(),
        }
        .into());
    }

    dir_utils
        .popd()
        .context("trying to navigate back to original path")?;

    Ok(())
}
