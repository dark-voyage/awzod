use crate::scheme::Content;
use std::fs;
use std::io::{Seek, SeekFrom, Write};
use std::path::PathBuf;
use std::process::exit;
use std::str::FromStr;

static BINARY: &str = include_str!("../database.json");

pub struct Database {
    path: Option<PathBuf>,
    pub content: Content,
}

impl FromStr for Database {
    type Err = ();

    fn from_str(data: &str) -> Result<Database, ()> {
        Ok(Self {
            path: None,
            content: Database::parse(data.to_string()),
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
            content: Database::parse(BINARY.to_string()),
        }
    }

    pub fn from_file(path: PathBuf) -> Self {
        let content = fs::read_to_string(path.clone());

        let read = match content {
            Ok(c) => c,
            Err(e) => {
                eprintln!("Couldn't read database file!\n{}", e);
                exit(1)
            }
        };

        Self {
            path: Some(path),
            content: Database::parse(read),
        }
    }

    pub fn save(&self, pretty: bool) -> std::io::Result<()> {
        let mut file = fs::OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(self.path.as_ref().unwrap().as_path())
            .unwrap();

        file.seek(SeekFrom::Start(0)).unwrap();
        file.write_all(self.content.to_string(pretty).as_bytes())
    }

    pub fn parse(data: String) -> Content {
        match serde_json::from_str::<Content>(data.as_str()) {
            Ok(c) => c,
            Err(e) => {
                eprintln!("Couldn't parse database file!\n{}", e);
                exit(1)
            }
        }
    }
}
