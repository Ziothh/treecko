use clap::Parser;
use treecko::cli::*;

fn main() {
    let cli = CLI::parse();

    cli.run()
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
