use crate::{exec_on, internal::shell::ShellError};
use anyhow::Context;
use clap::Args;
use std::path::Path;
use thiserror::Error;

use super::InternalCommandOptions;

#[derive(Args)]
pub struct GitHubOpenOptions {
    /// The path of the project for which to open the Git repository.
    #[arg(default_value = ".")]
    path: String,
}

#[derive(Error, Debug)]
pub enum GitHubOpenError {
    #[error("path '{path}' does not exist or is not an accessible directory")]
    PathDoesNotExist { path: String },

    #[error("path '{path}' is not a Git repository")]
    PathNotAGitRepository { path: String },

    #[error("project at path '{path}' has no remotes configured")]
    NoRemotesConfigured { path: String },
}

pub fn git_hub_open(
    InternalCommandOptions {
        shell,
        base_args: _,
        options,
    }: InternalCommandOptions<GitHubOpenOptions>,
) -> anyhow::Result<()> {
    let path = Path::new(&options.path);
    let path_str = path.to_string_lossy().to_string();

    if !path.exists() || !path.is_dir() {
        return Err(GitHubOpenError::PathDoesNotExist {
            path: options.path.clone(),
        }
        .into());
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
            |output| Ok(!output.trim().is_empty()),
        )
        .context("checking whether current dir is a git repository")?;

    if !is_git_dir {
        return Err(GitHubOpenError::PathNotAGitRepository { path: path_str }.into());
    }

    let remote_branches = exec_on!(shell, "git", "-C", path_str, "branch", "-r")
        .map(|output| output.trim().to_string())?;

    if remote_branches.is_empty() {
        return Err(GitHubOpenError::NoRemotesConfigured { path: path_str }.into());
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
    .map(|output| output.trim().to_string())
    .context("trying to fetch the remote url")?;

    exec_on!(shell, "open", remote_url)?;

    Ok(())
}
