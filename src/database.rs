use std::fs;
use std::path::PathBuf;
use std::process::exit;
use std::str::FromStr;
use rand::seq::SliceRandom;
use crate::scheme::Content;

pub type Datafile = Vec<Content>;

pub struct Database {
    path: Option<PathBuf>,
    content: Datafile
}

impl FromStr for Database {
    type Err = ();

    fn from_str(data: &str) -> Result<Database, ()> {
            Ok(Self {
                path: None,
                content: Database::parse(data.to_string())
            })
    }
}

impl Database {
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
        fs::write(self.path.as_ref().unwrap().as_path(), self.to_string(pretty))
    }

    pub fn add(&self) {

    }

    pub fn random(&self) -> &Content {
        match self.content.choose(&mut rand::thread_rng()) {
            Some(c) => c,
            None => {
                eprintln!("Couldn't choose random from database!");
                exit(1)
            }
        }
    }

    pub fn to_string(&self, pretty: bool) -> String {
        let wrap = match pretty {
            true => serde_json::to_string_pretty(&self.content),
            false => serde_json::to_string(&self.content)
        };
        
        match wrap {
            Ok(c) => c,
            Err(e) => {
                eprintln!("Couldn't convert content to JSON!\n{}", e);
                exit(1)
            }
        }
    }

    pub fn parse(data: String) -> Datafile {
        match serde_json::from_str::<Datafile>(data.as_str()) {
            Ok(c) => c,
            Err(e) => {
                eprintln!("Couldn't parse database file!\n{}", e);
                exit(1)
            }
        }
    }
}