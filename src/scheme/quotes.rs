use crate::scheme::Colorful;
use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Serialize, Deserialize)]
pub struct Quote {
    author: String,
    content: String,
}

impl Quote {
    pub fn new(author: String, content: String) -> Self {
        Self { author, content }
    }
}

pub trait ToQuote {
    fn to_quote(&self) -> Quote;
}

impl Display for Quote {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "\"{}\" - {}", self.content, self.author)
    }
}

impl Colorful for Quote {
    fn to_colorful_string(&self) -> String {
        format!("\"{}\" - {}", self.content, self.author)
    }
}

impl ToQuote for String {
    fn to_quote(&self) -> Quote {
        Quote {
            author: "Shakhzod Kudratov".to_string(),
            content: self.to_string(),
        }
    }
}
