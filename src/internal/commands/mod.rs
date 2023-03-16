pub mod gho;
pub mod update;

pub use gho::*;
pub use update::*;

use super::{cli::BaseArgs, shell::Shell};

pub struct InternalCommandOptions<'a, T> {
    pub shell: &'a Shell,
    pub base_args: &'a BaseArgs,
    pub options: &'a T,
}
