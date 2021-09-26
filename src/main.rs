pub mod elements;

use elements::{board::Board, list::List, tile::Tile, tag::Tag};
use tui::style::Color;

fn main() {
    let mut b1 = Board::new("Board 1".to_owned());

    let l1 = List::new("List 1".to_owned());
    b1.push_list(l1);

    let ta1 = Tag::new("Tag 1".to_owned(), Color::Green);
    let ti1 = Tile::new("Tile 1".to_owned(), "Desc 1".to_owned(), Some(ta1));
    b1.get_list(0).push_tile(ti1);

    println!("{:?}", b1);
}
