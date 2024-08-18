use super::tile::Tile;

const Q: usize = 5;
const R: usize = 6;

// odd-q rectangular flat-top
pub struct HexGrid {
    tiles: [Slot; Q*R],
}

impl HexGrid {
    pub fn new() -> HexGrid {
        HexGrid {
            tiles: [
                // col 0
                Slot::Empty,
                Slot::Filled(None),
                Slot::Filled(None),
                Slot::Filled(None),
                Slot::Filled(None),
                Slot::Empty,

                // col 1
                Slot::Filled(None),
                Slot::Filled(None),
                Slot::Filled(None),
                Slot::Filled(None),
                Slot::Filled(None),
                Slot::Empty,

                // col 2
                Slot::Filled(None),
                Slot::Filled(None),
                Slot::Filled(None),
                Slot::Filled(None),
                Slot::Filled(None),
                Slot::Filled(None),

                // col 3
                Slot::Filled(None),
                Slot::Filled(None),
                Slot::Filled(None),
                Slot::Filled(None),
                Slot::Filled(None),
                Slot::Empty,

                // col 4
                Slot::Empty,
                Slot::Filled(None),
                Slot::Filled(None),
                Slot::Filled(None),
                Slot::Filled(None),
                Slot::Empty,
            ],
        }
    }

    pub fn place(&mut self, tile: Tile, q: usize, r: usize) {
        self.tiles[Self::to_index(q, r)] = Slot::Filled(Some(tile));
    }

    pub fn hex_at(&self, q: usize, r: usize) -> &Slot {
        &self.tiles[Self::to_index(q, r)]
    }

    pub fn item_at(&self, q: usize, r: usize) -> &Option<Tile> {
        match self.hex_at(q, r) {
            Slot::Empty => &None,
            Slot::Filled(tile) => tile,
        }
    }

    fn to_index(q: usize, r: usize) -> usize {
        (q * R) + r
    }
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum Slot {
    Empty,
    Filled(Option<Tile>),
}

#[cfg(test)]
mod tests {
    use crate::hex::tile::{Color, Suit};

    use super::*;

    #[test]
    fn test_new_grid() {
        let grid = HexGrid::new();

        assert_eq!(grid.hex_at(0, 0), &Slot::Empty);
        assert_eq!(grid.hex_at(0, 1), &Slot::Filled(None));
    }

    #[test]
    fn test_place_tile() {
        let mut grid = HexGrid::new();

        let tile = Tile::new(Color::Red, Suit::Heart);
        grid.place(tile, 0, 1);

        assert_eq!(grid.hex_at(0, 1), &Slot::Filled(Some(tile)));
        assert_eq!(grid.item_at(0, 1), &Some(tile));
    }
}