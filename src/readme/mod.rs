use crate::scheme::Content;
use std::io::{Seek, SeekFrom, Write};
use std::path::PathBuf;

static HEADER: &str = include_str!("header.md");
static FOOTER: &str = include_str!("footer.md");

pub struct Readme {
    content: String,
}

impl Default for Readme {
    fn default() -> Self {
        Self::new()
    }
}

impl Readme {
    pub fn new() -> Self {
        let mut start = String::new();
        start.push_str(HEADER);

        Self { content: start }
    }

    pub fn write(content: String, path: Option<PathBuf>) -> std::io::Result<()> {
        let place = match path {
            Some(p) => p,
            None => "readme.md".parse().unwrap(),
        };

        let mut file = std::fs::OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(place)
            .unwrap();

        file.seek(SeekFrom::Start(0)).unwrap();
        file.write_all(content.as_bytes())
    }

    pub fn render(&self, data: Content) {
        let mut content = self.content.clone();

        for category in data.content.iter() {
            content.push_str(category.to_string().as_str())
        }

        content.push_str(FOOTER);

        Readme::write(content, None).unwrap();
    }
}
