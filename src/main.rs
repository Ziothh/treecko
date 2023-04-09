use std::{path::PathBuf, slice::Iter};

use clap::{Parser, Subcommand};
use treecko::cli::*;

#[derive(Debug)]
pub enum ProjectVariant {
    Rust,
    TypeScript,
}

type DirData = Vec<std::fs::DirEntry>;

static PROJECT_VARIANT_PARSERS: &[fn(dir_entries: &DirData) -> Option<ProjectVariant>] = &[
    |dir_entries| {
        if dir_entries
            .iter()
            .filter(|x| x.path().is_file())
            .any(|x| x.file_name() == "tsconfig.json")
        {
            return Some(ProjectVariant::TypeScript);
        }

        return None;
    },
    // Rust
    |dir_entries| {
        // dir_entries.any(|x| true);
        // println!("{:#?}", &dir_entries);

        if dir_entries
            .iter()
            .filter(|x| x.path().is_file())
            .any(|x| x.file_name() == "Cargo.toml")
        {
            return Some(ProjectVariant::Rust);
        }

        return None;
    },
];

#[derive(Debug)]
struct Project {
    path: String,
    variants: Vec<ProjectVariant>,
}
// impl Project {
//     fn new() -> Self {
//         return Self {
//             path
//         };
//     }
//
// }
impl TryFrom<&std::path::Path> for Project {
    type Error = String;

    fn try_from(path: &std::path::Path) -> Result<Self, Self::Error> {
        let dir_entries = read_dir(path).map_err(|err| err.to_string())?;

        let variants: Vec<_> = PROJECT_VARIANT_PARSERS
            .iter()
            .filter_map(|x| x(&dir_entries))
            .collect();

        if variants.len() == 0 {
            return Err("Could not find product variant for current directory.".to_owned());
        }

        return Ok(Project {
            path: path.to_string_lossy().to_string(),
            variants,
        });
    }
}

fn main() {
    handle().unwrap();
}

fn handle() -> anyhow::Result<()> {
    let cli = CLI::parse();

    match cli.command.unwrap() {
        Commands::List(args) => {
            let results = handle_dir(std::path::Path::new(&args.path))?;

            // let results = handle_dirs(std::path::Path::new(&args.path), |entry| {
            //     let path = entry.path();

            //     println!("{path:#?} {:#?} {:#?}", path.is_file(), path.is_dir());
            //     if !path.is_file() {
            //         return None;
            //     }

            //     return Some(path);
            // })
            // .unwrap();

            println!("{results:#?}")

            // let a = walkdir::WalkDir::new(path.as_ref());

            // for entry in a {
            //     let entry = entry?;
            //     k

            //     // walkdir::DirEntry

            //     // println!("{:#?}", entry.path())
            // }

            // println!("{path}");
        }
        _ => {
            unreachable!("not supported or implemented")
        }
    }

    Ok(())
}

/** Recursively checks every directory if it has a project. */
fn handle_dir(path: &std::path::Path) -> anyhow::Result<Vec<Project>> {
    let mut results = Vec::new();
    let dir_entries = read_dir(path)?;

    // Try if this is a project directory
    if let Ok(project) = Project::try_from(path) {
        results.push(project);
    } else {
        // Recursively retry
        for p in dir_entries.iter().map(|x| x.path()).filter(|x| x.is_dir()) {
            println!("{:#?}", &p);
            if let Ok(mut r) = handle_dir(&p) {
                results.append(&mut r)
            }
        }
    }

    return Ok(results);
}

fn read_dir(path: &std::path::Path) -> std::io::Result<DirData> {
    let dir_items: Vec<_> = std::fs::read_dir(path)?.into_iter().flatten().collect();

    return Ok(dir_items);
}

// fn handle_dirs<R>(
//     path: &std::path::Path,
//     callback: impl Fn(&std::fs::DirEntry) -> Option<R>,
// ) -> std::io::Result<Vec<R>> {
//     let mut results: Vec<R> = vec![];

//     if path.is_file() {
//         return Ok(results);
//     }

//     let dir_items: Vec<_> = std::fs::read_dir(path)?
//         .into_iter()
//         .flatten()
//         .collect();

//     for entry in dir_items {
//         if let Some(r) = callback(&entry) {
//             results.push(r);
//             continue;
//         }

//         results.append(&mut handle_dirs(&entry.path(), &callback).unwrap());

//         println!("{:#?}", entry.path());
//     }

//     return Ok(results);
// }
