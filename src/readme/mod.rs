static HEADER: &str = include_str!("header.md");
static FOOTER: &str = include_str!("footer.md");

#[derive(Default)]
pub struct Readme {
    content: String
}



impl Readme {
    pub fn new() -> Readme {
        Readme::default()
    }


}