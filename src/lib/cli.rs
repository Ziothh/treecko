use clap::{Args, Parser, Subcommand};

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
    pub command: Option<Commands>,
}

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
}

#[derive(Debug, Args)]
pub struct ListCommand {
    pub path: String,

    #[arg(short, long)]
    pub verbose: bool,
}
