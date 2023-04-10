use std::io::Write;

use crate::commands::prelude::*;
use console::{style, Style};
use dialoguer::{Confirm, FuzzySelect, Select};

#[derive(Debug, Args)]
pub struct GotoCommand {
    pub path: String,
    // #[arg(short, long)]
    // pub verbose: bool,
}

impl RunableCommand for GotoCommand {
    fn run(&self) -> Self::Output {
        let results = Project::find_recursively(std::path::Path::new(&self.path)).unwrap();

        let Ok(response) = Select::with_theme(&dialoguer::theme::ColorfulTheme::default())
          .items(&results)
          .interact() else {
            return ();
          };

        let chosen_project = results.get(response).unwrap();

        let mut stdout = std::io::stdout().lock();
        writeln!(stdout, "{}", &chosen_project.path).map_err(|err| {
            if err.kind() == std::io::ErrorKind::BrokenPipe {
                return;
            }
            // Ignore any error that may occur while writing to stderr.
            let _ = writeln!(std::io::stderr(), "{}", err);
        }).ok();

        stdout.flush().ok();
    }
}
