use super::dialogue::Dialogue;
use super::quotes::Quote;
use crate::scheme::Colorful;
use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Serialize, Deserialize)]
pub enum Category {
    Quote(Quote),
    Dialogue(Dialogue),
}

impl Display for Category {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Category::Quote(q) => write!(f, "{}", q.to_string()),
            Category::Dialogue(d) => write!(f, "{}", d.to_string()),
        }
    }
}

impl Colorful for Category {
    fn to_colorful_string(&self) -> String {
        match self {
            Category::Quote(q) => q.to_colorful_string(),
            Category::Dialogue(d) => d.to_colorful_string(),
        }
    }
}
