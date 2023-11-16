pub mod category;
pub mod dialogue;
pub mod quotes;

use crate::scheme::category::Category;
use colored::*;
use rand::seq::SliceRandom;
use serde::{Deserialize, Serialize};
use std::process::exit;

pub trait Colorful {
    fn to_colorful_string(&self) -> String;
}

pub trait Markdown {
    fn to_blockquote(&self) -> String;
    fn to_paragraph(&self) -> String;
}

#[derive(Serialize, Deserialize)]
pub struct Content {
    pub content: Vec<Category>,
}

impl Default for Content {
    fn default() -> Self {
        Self::new()
    }
}

impl Content {
    pub fn new() -> Self {
        Self {
            content: Vec::new(),
        }
    }

    pub fn random(&self) -> &Category {
        match self.content.choose(&mut rand::thread_rng()) {
            Some(c) => c,
            None => {
                eprintln!("{}", "Couldn't choose random from database!".red());
                exit(1)
            }
        }
    }

    pub fn add(&mut self, category: Category) {
        self.content.push(category);
    }

    pub fn merge(&mut self, content: Vec<Category>) {
        for category in content {
            self.add(category);
        }
    }

    pub fn parse(data: String) -> Content {
        match serde_json::from_str::<Vec<Category>>(data.as_str()) {
            Ok(c) => Content { content: c },
            Err(e) => {
                eprintln!("{}\n{}", "Couldn't parse database file!".red(), e);
                exit(1)
            }
        }
    }

    pub fn to_string(&self, pretty: bool) -> String {
        let wrap = match pretty {
            true => serde_json::to_string_pretty(&self.content),
            false => serde_json::to_string(&self.content),
        };

        match wrap {
            Ok(c) => c,
            Err(e) => {
                eprintln!("{}\n{}", "Couldn't convert content to JSON!".red(), e);
                exit(1)
            }
        }
    }
}
