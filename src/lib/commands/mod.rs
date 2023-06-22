use clap::Subcommand;

use crate::commands::prelude::RunableCommand;

use self::{goto::GotoCommand, list::ListCommand, shell::ShellCommand};

pub mod prelude;

pub mod goto;
pub mod list;
pub mod shell;

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// does testing things
    // Test {
    //     /// lists test values
    //     #[arg(short, long)]
    //     list: bool,

    //     #[arg(short, long)]
    //     verbose: bool,
    // },
    List(ListCommand),
    Goto(GotoCommand),
    Shell(ShellCommand),
}

impl Commands {
    pub fn run(&self) -> () {
        match self {
            Self::List(list) => list.run(),
            Self::Goto(goto) => goto.run(),
            Self::Shell(shell) => shell.run(),
        }
    }
}
