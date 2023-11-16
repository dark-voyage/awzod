use crate::scheme::Content;
use colored::Colorize;
use std::fs;
use std::io::Write;
use std::path::PathBuf;
use std::process::exit;
use std::str::FromStr;

static BINARY: &str = include_str!("../cache.json");

pub struct Database {
    path: Option<PathBuf>,
    pub content: Content,
}

impl FromStr for Database {
    type Err = ();

    fn from_str(data: &str) -> Result<Database, ()> {
        Ok(Self {
            path: None,
            content: Content::parse(data.to_string()),
        })
    }
}

impl Database {
    pub fn from_path_or_binary(path: Option<PathBuf>) -> Self {
        match path {
            Some(c) => Database::from_file(c),
            None => Database::from_binary(),
        }
    }

    pub fn from_binary() -> Self {
        Self {
            path: None,
            content: Content::parse(BINARY.to_string()),
        }
    }

    pub fn from_file(path: PathBuf) -> Self {
        let content = fs::read_to_string(path.clone());

        let read = match content {
            Ok(c) => c,
            Err(e) => {
                eprintln!("{}\n{}", "Couldn't read database file!".red(), e);
                exit(1)
            }
        };

        Self {
            path: Some(path),
            content: Content::parse(read),
        }
    }

    pub fn from_file_or_new(path: PathBuf) -> Self {
        let content = fs::read_to_string(path.clone());

        match content {
            Ok(c) => Self {
                path: Some(path),
                content: Content::parse(c),
            },
            Err(_) => Self {
                path: Some(path),
                content: Content::new(),
            },
        }
    }

    pub fn save(&self, pretty: bool) -> std::io::Result<()> {
        if self.path.as_ref().unwrap().exists() {
            fs::remove_file(self.path.as_ref().unwrap().as_path()).unwrap();
        }

        let mut file = fs::OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(self.path.as_ref().unwrap().as_path())
            .unwrap();

        file.write_all(self.content.to_string(pretty).as_bytes())
    }
}
