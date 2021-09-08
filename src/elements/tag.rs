use tui::style::Color;

// Tag used to represent a category of tiles
pub struct Tag {
    name: String,
    color: Color,
}

impl Tag {
    // Constructor
    fn new(name: String, color: Color) -> Tag {
        Tag {name, color}
    }

    // Getters and setters
    fn set_name(&mut self, name: String) {
        self.name = name;
    }

    fn set_color(&mut self, color: Color) {
        self.color = color;
    }

    fn get_name(&self) -> &String {
        &self.name
    }

    fn get_color(&self) -> Color {
        self.color
    }
}
