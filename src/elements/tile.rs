use super::tag::Tag;

// Tile used to represent a single task/process
#[derive(Debug)]
pub struct Tile {
    title: String,
    desc: String,
    tag: Option<Tag>,
}

impl Tile {
    // Constructor
    pub fn new(title: String, desc: String, tag: Option<Tag>) -> Tile {
        Tile {title, desc, tag}
    }

    // Getters and setters
    pub fn get_title(&self) -> &String {
        &self.title
    }

    pub fn get_desc(&self) -> &String {
        &self.desc
    }

    pub fn get_tag(&self) -> &Option<Tag> {
        &self.tag
    }

    pub fn set_title(&mut self, title: String) {
        self.title = title;
    }

    pub fn set_desc(&mut self, desc: String) {
        self.desc = desc;
    }

    pub fn set_tag(&mut self, tag: Tag) {
        self.tag = Some(tag);
    }

    // Differentiate between some and none
    pub fn remove_tag(&mut self) {
        self.tag = None;
    }
}
