use std::path::PathBuf;

use anyhow::Context;
use log::debug;

pub struct DirectoryUtils {
    dir_stack: Vec<PathBuf>,
}

impl DirectoryUtils {
    pub fn new() -> Self {
        Self {
            dir_stack: vec![std::env::current_dir().unwrap()],
        }
    }

    /// Returns the current working directory.
    fn get_cwd(&self) -> &PathBuf {
        &self.dir_stack.last().unwrap()
    }

    /// Changes the current working directory to the given path and returns the previous working
    /// directory.
    ///
    /// Equivalent to the command `pushd`.
    pub fn pushd(&mut self, new_path: &PathBuf) -> anyhow::Result<&PathBuf> {
        debug!("changing cwd from {:?} to {:?}", self.get_cwd(), new_path);

        std::env::set_current_dir(new_path).context(format!(
            "trying to change cwd from {:?} to {:?}",
            self.get_cwd(),
            new_path
        ))?;

        self.dir_stack.push(new_path.clone());

        Ok(&self.dir_stack[self.dir_stack.len() - 2])
    }

    /// Changes the current working directory to the previous working directory and returns the
    /// previous working directory.
    ///
    /// Equivalent to the command `popd`.
    pub fn popd(&mut self) -> anyhow::Result<PathBuf> {
        if self.dir_stack.len() == 1 {
            anyhow::bail!("cannot popd from root directory");
        }
        let prev_dir = self.dir_stack.pop().unwrap();
        debug!("changing cwd from {:?} to {:?}", self.get_cwd(), prev_dir);

        std::env::set_current_dir(&prev_dir).context(format!(
            "trying to change cwd from {:?} to {:?}",
            self.get_cwd(),
            prev_dir
        ))?;

        Ok(prev_dir)
    }
}
