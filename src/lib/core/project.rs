use std::fmt::Display;

use console::{Style, style};

#[derive(Debug)]
pub enum ProjectLanguage {
    Rust,
    TypeScript,
}

// impl ToString for ProjectLanguage {
//   fn to_string(&self) -> String {
//     return format!("{self:?}");
//   }
//
// }

impl Display for ProjectLanguage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let styler = Style::new();

        let styler = match self {
            ProjectLanguage::TypeScript => styler.blue(),
            ProjectLanguage::Rust => styler.red(),
        };

        write!(f, "{}", styler.apply_to(format!("{:?}", self)))
    }
}

static PROJECT_VARIANT_PARSERS: &[fn(
    dir_entries: &crate::utils::DirData,
) -> Option<ProjectLanguage>] = &[
    // TypeScript
    |dir_entries| {
        if dir_entries
            .iter()
            .filter(|x| x.path().is_file())
            .any(|x| x.file_name() == "tsconfig.json")
        {
            return Some(ProjectLanguage::TypeScript);
        }

        return None;
    },
    // Rust
    |dir_entries| {
        if dir_entries
            .iter()
            .filter(|x| x.path().is_file())
            .any(|x| x.file_name() == "Cargo.toml")
        {
            return Some(ProjectLanguage::Rust);
        }

        return None;
    },
];

#[derive(Debug)]
pub struct Project {
    pub name: String,
    pub is_monorepo: bool,
    pub path: String,
    pub languages: Vec<ProjectLanguage>,
}
impl Project {
    //     fn new() -> Self {
    //         return Self {
    //             path
    //         };
    //     }
    /** Recursively checks every directory if it has a project. */
    pub fn find_recursively(path: &std::path::Path) -> anyhow::Result<Vec<Project>> {
        let mut results = Vec::new();
        let dir_entries = crate::utils::read_dir(path)?;

        // Try if this is a project directory
        if let Ok(project) = Project::try_from(path) {
            results.push(project);
        } else {
            // Recursively retry
            for p in dir_entries.iter().map(|x| x.path()).filter(|x| x.is_dir()) {
                if let Ok(mut r) = Project::find_recursively(&p) {
                    results.append(&mut r)
                }
            }
        }

        return Ok(results);
    }
}
impl TryFrom<&std::path::Path> for Project {
    type Error = String;

    fn try_from(path: &std::path::Path) -> Result<Self, Self::Error> {
        let dir_entries = crate::utils::read_dir(path).map_err(|err| err.to_string())?;

        let languages: Vec<_> = PROJECT_VARIANT_PARSERS
            .iter()
            .filter_map(|x| x(&dir_entries))
            .collect();

        if languages.len() == 0 {
            return Err("Could not find product variant for current directory.".to_owned());
        }

        let path = path.to_string_lossy().to_string();

        return Ok(Project {
            name: path.split('/').last().unwrap().to_string(),
            is_monorepo: languages.len() > 1,
            path,
            languages,
        });
    }
}

impl Display for Project {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return write!(
            f,
            "{}: [{}]",
            style(&self.name).bold(),
            self.languages
                .iter()
                .map(|x| format!("{}", x))
                .collect::<Vec<_>>()
                .join(", ")
        );
    }
}
