use std::io::Write;

use crate::commands::prelude::*;
use console::{style, Style};
use dialoguer::{Confirm, FuzzySelect, Select};

#[derive(Debug, Args)]
pub struct GotoCommand {
    pub path: String,

    /// Search projects with a basic select instead of a fuzzy finder.
    #[arg(long)]
    pub no_fuzzy: bool,
    // #[arg(short, long)]
    // pub verbose: bool,
}

impl RunableCommand for GotoCommand {
    fn run(&self) -> Self::Output {
        let mut results = Project::find_recursively(std::path::Path::new(&self.path)).unwrap();
        results.sort_by_cached_key(|project| project.name.to_lowercase());

        let theme = dialoguer::theme::ColorfulTheme::default();
        let prompt = "Choose a project:";

        let Ok(response) = (
          if self.no_fuzzy {
            Select::with_theme(&theme).with_prompt(prompt).items(&results).default(0).interact()
          } else {
            FuzzySelect::with_theme(&theme).with_prompt(prompt).items(&results).interact()
          }
        ) else {
            return ();
        };

        let chosen_project = results.get(response).unwrap();

        let mut stdout = std::io::stdout().lock();
        writeln!(stdout, "{}", &chosen_project.path)
            .map_err(|err| {
                if err.kind() == std::io::ErrorKind::BrokenPipe {
                    return;
                }
                // Ignore any error that may occur while writing to stderr.
                let _ = writeln!(std::io::stderr(), "{}", err);
            })
            .ok();

        stdout.flush().ok();
    }
}
