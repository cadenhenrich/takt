use super::tag::Tag;

// Tile used to represent a single task/process
pub struct Tile {
    title: String,
    desc: String,
    tag: Option<Tag>,
}

impl Tile {
    // Constructor
    fn new(title: String, desc: String, tag: Option<Tag>) -> Tile {
        Tile {title, desc, tag}
    }

    // Getters and setters
    fn get_title(&self) -> &String {
        &self.title
    }

    fn get_desc(&self) -> &String {
        &self.desc
    }

    fn get_tag(&self) -> &Option<Tag> {
        &self.tag
    }

    fn set_title(&mut self, title: String) {
        self.title = title;
    }

    fn set_desc(&mut self, desc: String) {
        self.desc = desc;
    }

    fn set_tag(&mut self, tag: Tag) {
        self.tag = Some(tag);
    }

    // Differentiate between some and none
    fn remove_tag(&mut self) {
        self.tag = None;
    }
}
