use tui::style::Color;

// Tag used to represent a category of tiles
pub struct Tag {
    name: String, // The name of the tag
    color: Color, // The color of the tag
}

impl Tag {
    // Creates a new tag with the given name and color.
    fn new(name: String, color: Color) -> Tag {
        Tag {name, color}
    }

    // Sets the name of the tag
    fn set_name(&mut self, name: String) {
        self.name = name;
    }

    // Sets the color of the tag
    fn set_color(&mut self, color: Color) {
        self.color = color;
    }

    // Returns the tame of the tag
    fn get_name(&self) -> &String {
        &self.name
    }

    // Returns the color of the tag
    fn get_color(&self) -> Color {
        self.color
    }
}
