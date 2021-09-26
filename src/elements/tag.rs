use tui::style::Color;

// Tag used to represent a category of tiles
#[derive(Debug)]
pub struct Tag {
    name: String,
    color: Color,
}

impl Tag {
    // Constructor
    pub fn new(name: String, color: Color) -> Tag {
        Tag {name, color}
    }

    // Getters and setters
    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    pub fn set_color(&mut self, color: Color) {
        self.color = color;
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub fn get_color(&self) -> Color {
        self.color
    }
}
