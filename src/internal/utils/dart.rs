use crate::{exec_on, internal::shell::Shell};
use anyhow::Context;
use log::debug;
use std::{env, path::PathBuf};

const IGNORED_FOLDERS: [&str; 10] = [
    "ios",
    "android",
    "windows",
    "linux",
    "macos",
    ".symlinks",
    ".plugin_symlinks",
    ".dart_tool",
    "build",
    ".fvm",
];

pub struct DartShell<'a> {
    shell: &'a Shell,
}

impl<'a> DartShell<'a> {
    pub fn new(shell: &'a Shell) -> Self {
        Self { shell }
    }

    pub fn find_dart_projects(
        &self,
        path: Option<&PathBuf>,
    ) -> anyhow::Result<Vec<DartProjectMetadata>> {
        let cwd = env::current_dir().context("trying to get current directory")?;
        let path = path.unwrap_or(&cwd);
        debug!("finding Dart projects recursively in path: {:?}", path);

        let path_str = path.to_str().context(format!(
            "trying to convert path '{}' to a string",
            path.display()
        ))?;

        let output = exec_on!(
            self.shell,
            "find",
            path_str,
            "-type",
            "f",
            "-name",
            "pubspec.yaml",
            "-exec",
            "dirname",
            "{}",
            ";"
        )
        .context(format!(
            "trying to find Dart projects in path '{}'",
            path.display()
        ))?;

        let projects = output
            .stdout
            .lines()
            .map(|line| PathBuf::try_from(line.trim()))
            .filter_map(Result::ok)
            .map(|path| path.canonicalize())
            .filter_map(Result::ok)
            .filter(|path| {
                let Some(last_component) = path.components().last() else {
                    return true;
                };

                let Some(dir_name) = last_component.as_os_str().to_str() else {
                    return true;
                };

                !IGNORED_FOLDERS.contains(&dir_name)
            })
            .map(|path| self.get_dart_project_metadata(path))
            .collect::<Result<Vec<_>, _>>()?;

        debug!("found {} Dart projects", projects.len());

        Ok(projects)
    }

    pub fn get_dart_project_metadata(&self, path: PathBuf) -> anyhow::Result<DartProjectMetadata> {
        let name = path
            .file_name()
            .context(format!(
                "trying to get file name from path '{}'",
                path.display()
            ))?
            .to_str()
            .context(format!(
                "trying to convert file name from path '{}' to a string",
                path.display()
            ))?
            .to_string();

        let pubspec_file_path = path.join("pubspec.yaml");

        let pubspec_file = std::fs::File::open(&pubspec_file_path).context(format!(
            "trying to open pubspec.yaml file at path '{}'",
            pubspec_file_path.display()
        ))?;
        let pubspec_yaml: serde_yaml::Value =
            serde_yaml::from_reader(pubspec_file).context(format!(
                "trying to parse pubspec.yaml file at path '{}'",
                pubspec_file_path.display()
            ))?;

        let is_flutter_project = !pubspec_yaml["dependencies"]["flutter"].is_null();

        Ok(DartProjectMetadata {
            path,
            name,
            is_flutter_project,
        })
    }
}

#[derive(Debug)]
pub struct DartProjectMetadata {
    pub path: PathBuf,
    pub name: String,
    pub is_flutter_project: bool,
}
