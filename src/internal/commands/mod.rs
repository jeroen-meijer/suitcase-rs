pub mod ford;
pub mod gho;
pub mod upgrade;

pub use ford::*;
pub use gho::*;
pub use upgrade::*;

use super::{cli::BaseArgs, shell::Shell};

pub struct InternalCommandOptions<'a, T> {
    pub shell: &'a Shell,
    pub base_args: &'a BaseArgs,
    pub options: &'a T,
}
