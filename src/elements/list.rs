use super::tile::Tile;

// List of tiles, individual types
pub struct List {
    name: String,
    tiles: Vec<Tile>
}

impl List {
    // Constructor
    fn new(name: String) -> List {
        List {name, tiles: vec![]}
    }

    // Getters and setters
    fn get_name(&self) -> &String {
        &self.name
    }

    fn get_tiles(&self) -> &Vec<Tile> {
        &self.tiles
    }

    fn get_tile(&self, index: usize) -> &Tile {
        &self.tiles.get(index).unwrap()
    }

    fn set_name(&mut self, name: String) {
        self.name = name;
    }

    fn push_tile(&mut self, tile: Tile) {
        self.tiles.push(tile);
    }

    fn pop_tile(&mut self) {
        self.tiles.pop();
    }

    fn remove_tile(&mut self, index: usize) {
        self.tiles.remove(index);
    }
}
