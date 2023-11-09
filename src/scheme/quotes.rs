use crate::scheme::{Colorful, Markdown};
use serde::{Deserialize, Serialize};
use std::fmt::Display;
use colored::Colorize;

#[derive(Serialize, Deserialize, PartialEq)]
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
        format!("{yb}{}{yb} - {}", self.content.red(), self.author.yellow(), yb = "\"".red())
    }
}

impl Markdown for Quote {
    fn to_blockquote(&self) -> String {
        format!(
            "<blockquote><strong>{}</strong> - <i>{}</i></blockquote>",
            self.content, self.author
        )
    }

    fn to_paragraph(&self) -> String {
        format!(
            "<p><strong>{}</strong> - <i>{}</i></p>",
            self.content, self.author
        )
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
