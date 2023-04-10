use console::{style, Style};

use crate::commands::prelude::*;

#[derive(Debug, Args)]
pub struct ListCommand {
    pub path: String,

    #[arg(short, long)]
    pub verbose: bool,
}

impl RunableCommand for ListCommand {
    fn run(&self) -> Self::Output {
        let results = Project::find_recursively(std::path::Path::new(&self.path)).unwrap();

        let amount = results.len();
        let padding = (amount.checked_ilog10().unwrap_or(0) + 1) as usize;

        let black = Style::new().black();

        println!("Found {} projects:", style(amount).red());
        results.iter().enumerate().for_each(|(i, project)| {
            println!(
                "{} {project}{}",
                black.apply_to(format!("> {i:>width$}:", width = &padding)),
                black.apply_to(if self.verbose {
                    format!("\n{}\n", &project.path)
                } else {
                    "".to_owned()
                })
            )
        });
    }
}
