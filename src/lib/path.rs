pub mod meta {
    pub fn get_home() -> String {
        std::env::var("HOME").unwrap()
    }
}
pub mod tilde {
    use super::meta::get_home;

    pub fn from(path: String) -> String {
        if !path.starts_with("~/") {
            return path;
        }

        path.replacen("~/", &get_home(), 1)
    }

    pub fn into(path: String) -> String {
        let home = &get_home();

        if !path.starts_with(home) {
            return path;
        }

        path.replacen(home, "~/", 1)
    }
}
// let root_dir = env::current_dir().unwrap().join(
//     // Add support for the tilde to point to the home directory
//     shellexpand::tilde(sub_matches
//         .get_one::<String>("ROOT_DIRECTORY")
//         .expect("Could not get the root directory")
//     ).into_owned()
// );

// let root_dir = root_dir.canonicalize().unwrap_or_else(|err| {
//     let kind = err.kind();

//     match kind {
//         std::io::ErrorKind::NotFound => {
//             println!("Path \"{}\" is not a directory", root_dir.display());
//         }
//         _ => {}
//     };

//     exit(1)
// });
