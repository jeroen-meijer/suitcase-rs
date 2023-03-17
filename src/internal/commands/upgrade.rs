use std::str::FromStr;

use anyhow::Context;
use log::{debug, info};

use crate::{exec_on, internal::constants::PACKAGE_NAME, progress};

use super::InternalCommandOptions;

pub fn upgrade(
    InternalCommandOptions {
        shell,
        base_args: _,
        options: _,
    }: InternalCommandOptions<()>,
) -> anyhow::Result<()> {
    let installed_packages_str = progress!(
        "Getting cargo packages...",
        exec_on!(shell, "cargo", "install", "--list")
    )
    .context("trying to get installed cargo packages")?;

    let installed_packages = installed_packages_str
        .stdout
        .lines()
        .filter(|line| line.chars().next().map_or(false, |c| c.is_alphabetic()))
        .map(CargoPackageMetadata::from_str)
        .collect::<anyhow::Result<Vec<_>>>()?;

    debug!(
        "installed packages: {:?}",
        installed_packages
            .iter()
            .map(|package| package.name.as_str())
            .collect::<Vec<_>>()
    );

    let Some(this_package) = installed_packages
    .iter()
    .find(|package| package.name == PACKAGE_NAME) else {
        return Err(anyhow::anyhow!(
            "could not find {} package in installed packages (all packages: {:?})",
            PACKAGE_NAME,
            installed_packages
                .iter()
                .map(|package| package.name.as_str())
                .collect::<Vec<_>>()
        ));
    };

    debug!(
        "package name: {}, version: {}, path: {:?}",
        this_package.name,
        this_package.version,
        this_package.path.as_deref().unwrap_or("None")
    );

    let output = if let Some(path) = &this_package.path {
        progress!(
            format!("Upgrading suitcase from local path ({})...", path).as_str(),
            exec_on!(shell, "cargo", "install", "--force", "--path", path)
        )
        .context(format!(
            "trying to upgrade suitcase from local path ({})",
            path
        ))?
    } else {
        progress!(
            "Upgrading suitcase from crates.io...",
            exec_on!(shell, "cargo", "install", "--force", PACKAGE_NAME)
        )
        .context("trying to upgrade suitcase from crates.io")?
    };

    // For some reason, cargo prints to stderr instead of stdout when upgrading a package.
    let last_line = output
        .stderr
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .last();

    if let Some(last_line) = last_line {
        let backticks = last_line.matches('`');
        if backticks.count() >= 4 {
            let mut chars = last_line.chars();
            let previous_package_string = chars
                .by_ref()
                .skip_while(|c| *c != '`')
                .skip(1)
                .take_while(|c| *c != '`')
                .collect::<String>();

            let new_package_string = chars
                .skip_while(|c| *c != '`')
                .skip(1)
                .take_while(|c| *c != '`')
                .collect::<String>();

            let previous_package = CargoPackageMetadata::from_str(&previous_package_string)
                .context(format!(
                    "trying to parse previous package string {}",
                    previous_package_string
                ))?;

            let new_package = CargoPackageMetadata::from_str(&new_package_string).context(
                format!("trying to parse new package string {}", new_package_string),
            )?;

            info!(
                "Upgraded {} from v{} to v{}",
                previous_package.name, previous_package.version, new_package.version
            );
        }
    } else {
        info!("Upgraded suitcase successfully (but failed to parse output from cargo)");
    }

    Ok(())
}

struct CargoPackageMetadata {
    name: String,
    version: String,
    path: Option<String>,
}

impl FromStr for CargoPackageMetadata {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split_whitespace();
        let name = parts
            .next()
            .context(format!("trying to get package name for line {}", s))?;
        let version = parts
            .next()
            .map(|version| version.trim_start_matches('v').trim_end_matches(':'))
            .context(format!("trying to get package version for line {}", s))?;
        let path = parts.next().map(|path| {
            path.trim_start_matches('(')
                .trim_end_matches(':')
                .trim_end_matches(')')
        });
        Ok(CargoPackageMetadata {
            name: name.to_string(),
            version: version.to_string(),
            path: path.map(|path| path.to_string()),
        })
    }
}
