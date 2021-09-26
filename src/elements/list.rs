use super::tile::Tile;

// List of tiles, individual types
#[derive(Debug)]
pub struct List {
    name: String,
    tiles: Vec<Tile>
}

impl List {
    // Constructor
    pub fn new(name: String) -> List {
        List {name, tiles: vec![]}
    }

    // Getters and setters
    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub fn get_tiles(&self) -> &Vec<Tile> {
        &self.tiles
    }

    pub fn get_tile(&mut self, index: usize) -> &mut Tile {
        if index > self.tiles.len() {
            panic!("tile index out of range");
        }
        &mut self.tiles[index]
    }

    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    pub fn push_tile(&mut self, tile: Tile) {
        self.tiles.push(tile);
    }

    pub fn pop_tile(&mut self) -> Option<Tile> {
        self.tiles.pop()
    }

    pub fn remove_tile(&mut self, index: usize) {
        self.tiles.remove(index);
    }
}
