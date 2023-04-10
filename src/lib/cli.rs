use clap::Parser;

use crate::commands::Commands;

#[derive(Debug, Parser)]
#[clap(
    name = "Treecko",
    author, 
    version, 
    about, 
    long_about = None
)]
pub struct CLI {
    // /// Optional name to operate on
    // name: Option<String>,

    // /// Sets a custom config file
    // #[arg(short, long, value_name = "FILE")]
    // config: Option<PathBuf>,

    // /// Turn debugging information on
    // #[arg(short, long, action = clap::ArgAction::Count)]
    // debug: u8,
    #[clap(subcommand)]
    pub command: Commands,
    // pub command: Option<Commands>,
}

impl CLI {
  pub fn run(&self) {
    self.command.run();
  }
}
