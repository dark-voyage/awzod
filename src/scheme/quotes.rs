use crate::scheme::{Colorful, Markdown};
use colored::Colorize;
use serde::{Deserialize, Serialize};
use std::fmt::Display;

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
        format!(
            "{yb}{}{yb} - {}",
            self.content.red(),
            self.author.yellow(),
            yb = "\"".red()
        )
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

#[cfg(test)]
mod test {
    use super::Colorful;
    use super::*;

    #[test]
    fn test_quote() {
        let quote = Quote::new("Shakhzod Kudratov".to_string(), "Hello World!".to_string());
        assert_eq!(quote.to_string(), "\"Hello World!\" - Shakhzod Kudratov");
        assert_eq!(
            quote.to_colorful_string(),
            format!(
                "{yb}{}{yb} - {}",
                "Hello World!".red(),
                "Shakhzod Kudratov".yellow(),
                yb = "\"".red()
            )
        );
        assert_eq!(
            quote.to_blockquote(),
            "<blockquote><strong>Hello World!</strong> - <i>Shakhzod Kudratov</i></blockquote>"
        );
        assert_eq!(
            quote.to_paragraph(),
            "<p><strong>Hello World!</strong> - <i>Shakhzod Kudratov</i></p>"
        );
    }
}
