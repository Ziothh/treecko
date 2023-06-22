use std::io::Write;

use crate::commands::prelude::*;

#[derive(Debug, Args)]
pub struct ShellCommand {
}

const SHELL_SCRIPT: &str = include_str!("./script.zsh");

impl RunableCommand for ShellCommand {
    fn run(&self) -> Self::Output {
        let mut stdout = std::io::stdout().lock();
        writeln!(stdout, "{}", SHELL_SCRIPT).unwrap();
        stdout.flush().unwrap();
    }
}
