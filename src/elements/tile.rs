use super::tag::Tag;

// Tile used to represent a single task/process
pub struct Tile {
    title: String,
    desc: String,
    tag: Option<Tag>,
}

impl Tile {
    fn new(title: String, desc: String, tag: Option<Tag>) -> Tile {
        Tile {title, desc, tag}
    }
}
