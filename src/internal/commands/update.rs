use anyhow::Context;
use log::debug;

use crate::{exec_on, internal::constants::PACKAGE_NAME, progress};

use super::InternalCommandOptions;

pub fn update(
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
        .lines()
        .filter(|line| line.chars().next().map_or(false, |c| c.is_alphabetic()))
        .map(|line| {
            let mut parts = line.split_whitespace();
            let name = parts
                .next()
                .context(format!("trying to get package name for line {}", line))?;
            let version = parts
                .next()
                .map(|version| version.trim_start_matches('v').trim_end_matches(':'))
                .context(format!("trying to get package version for line {}", line))?;
            let path = parts.next().map(|path| {
                path.trim_start_matches('(')
                    .trim_end_matches(':')
                    .trim_end_matches(')')
            });
            Ok(InstalledCargoPackage {
                name: name.to_string(),
                version: version.to_string(),
                path: path.map(|path| path.to_string()),
            })
        })
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

    if let Some(path) = &this_package.path {
        progress!(
            format!("Updating suitcase from local path ({})...", path).as_str(),
            exec_on!(shell, "cargo", "install", "--force", "--path", path)
        )
        .context(format!(
            "trying to update suitcase from local path ({})",
            path
        ))?;
    } else {
        progress!(
            "Updating suitcase from crates.io...",
            exec_on!(shell, "cargo", "install", "--force", PACKAGE_NAME)
        )
        .context("trying to update suitcase from crates.io")?;
    }

    Ok(())
}

struct InstalledCargoPackage {
    name: String,
    #[allow(dead_code)]
    version: String,
    path: Option<String>,
}
