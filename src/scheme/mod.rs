pub mod category;
pub mod dialogue;
pub mod quotes;

use crate::scheme::category::Category;
use rand::seq::SliceRandom;
use serde::{Deserialize, Serialize};
use std::process::exit;

pub trait Colorful {
    fn to_colorful_string(&self) -> String;
}

#[derive(Serialize, Deserialize)]
pub struct Content {
    pub content: Vec<Category>,
}

impl Content {
    pub fn random(&self) -> &Category {
        match self.content.choose(&mut rand::thread_rng()) {
            Some(c) => c,
            None => {
                eprintln!("Couldn't choose random from database!");
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

    pub fn to_string(&self, pretty: bool) -> String {
        let wrap = match pretty {
            true => serde_json::to_string_pretty(&self.content),
            false => serde_json::to_string(&self.content),
        };

        match wrap {
            Ok(c) => c,
            Err(e) => {
                eprintln!("Couldn't convert content to JSON!\n{}", e);
                exit(1)
            }
        }
    }
}
