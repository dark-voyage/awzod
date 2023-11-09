use super::dialogue::Dialogue;
use super::quotes::Quote;
use crate::scheme::{Colorful, Markdown};
use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Category {
    Quote(Quote),
    Dialogue(Dialogue),
}

impl Display for Category {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Category::Quote(q) => write!(f, "{}", q),
            Category::Dialogue(d) => write!(f, "{}", d),
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

impl Markdown for Category {
    fn to_blockquote(&self) -> String {
        match self {
            Category::Quote(q) => q.to_blockquote(),
            Category::Dialogue(d) => d.to_blockquote(),
        }
    }

    fn to_paragraph(&self) -> String {
        match self {
            Category::Quote(q) => q.to_paragraph(),
            Category::Dialogue(d) => d.to_paragraph(),
        }
    }
}
