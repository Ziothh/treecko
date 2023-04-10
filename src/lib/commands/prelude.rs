pub use clap::{Args, Parser, Subcommand};
pub use crate::core::project::{Project, ProjectLanguage};

pub trait RunableCommand {
    type Output = ();
    fn run(&self) -> Self::Output;
}
