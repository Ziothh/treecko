use clap::Subcommand;

use crate::commands::prelude::RunableCommand;

use self::{list::ListCommand, goto::GotoCommand};

pub mod prelude;

pub mod list;
pub mod goto;

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
}

impl Commands {
    pub fn run(&self) -> () {
        match self {
            Self::List(list) => list.run(),
            Self::Goto(goto) => goto.run(),
        }
    }
}
