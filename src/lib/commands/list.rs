use console::style;

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

        println!("Found {} projects:", results.len());
        results
            .iter()
            .enumerate()
            .for_each(|(i, project)| println!("{} {project}", style(format!("> {i}:")).black()));
    }
}
