use crate::{exec_on, internal::shell::ShellError};
use anyhow::Context;
use clap::Args;
use std::path::PathBuf;
use thiserror::Error;

use super::InternalCommandOptions;

#[derive(Args)]
pub struct GitHubOpenOptions {
    /// The path of the project for which to open the Git repository.
    #[arg(default_value = ".")]
    path: PathBuf,
}

#[derive(Error, Debug)]
pub enum GitHubOpenError {
    #[error("path '{path}' does not exist or is not an accessible directory")]
    PathDoesNotExist { path: PathBuf },

    #[error("path '{path}' is not a Git repository")]
    PathNotAGitRepository { path: PathBuf },

    #[error("project at path '{path}' has no remotes configured")]
    NoRemotesConfigured { path: PathBuf },
}

pub fn git_hub_open(
    InternalCommandOptions {
        shell,
        base_args: _,
        options,
    }: InternalCommandOptions<GitHubOpenOptions>,
) -> anyhow::Result<()> {
    let path = options.path.clone();
    let path_str = path.to_str().unwrap();

    if !path.exists() || !path.is_dir() {
        return Err(GitHubOpenError::PathDoesNotExist { path }.into());
    }

    let is_git_dir = exec_on!(shell, "git", "-C", path_str, "rev-parse", "--git-dir")
        .map_or_else(
            |err| match err {
                ShellError::HostProcessExecutionFailure {
                    command: _,
                    args: _,
                    status,
                    stdout: _,
                    stderr: _,
                } if status.code() == Some(128) => Ok(false),
                _ => Err(err),
            },
            |output| Ok(!output.stdout.trim().is_empty()),
        )
        .context("checking whether current dir is a git repository")?;

    if !is_git_dir {
        return Err(GitHubOpenError::PathNotAGitRepository { path }.into());
    }

    let remote_branches = exec_on!(shell, "git", "-C", path_str, "branch", "-r")
        .map(|output| output.stdout.trim().to_string())?;

    if remote_branches.is_empty() {
        return Err(GitHubOpenError::NoRemotesConfigured { path }.into());
    }

    let remote_url = exec_on!(
        shell,
        "git",
        "-C",
        path_str,
        "config",
        "--get",
        "remote.origin.url"
    )
    .map(|output| output.stdout.trim().to_string())
    .context("trying to fetch the remote url")?;

    exec_on!(shell, "open", remote_url)?;

    Ok(())
}
