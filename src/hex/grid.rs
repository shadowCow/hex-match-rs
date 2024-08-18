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

    pub fn r(&self) -> usize {
        R
    }

    pub fn q(&self) -> usize {
        Q
    }

    pub fn place(&mut self, tile: Tile, q: usize, r: usize) {
        self.tiles[Self::to_index(q, r)] = Slot::Filled(Some(tile));
    }

    pub fn hex_at(&self, q: usize, r: usize) -> &Slot {
        let index = Self::to_index(q, r);
        if index >= self.tiles.len() {
            &Slot::Empty
        } else {
            &self.tiles[index]
        }
    }

    pub fn item_at(&self, q: usize, r: usize) -> &Option<Tile> {
        match self.hex_at(q, r) {
            Slot::Empty => &None,
            Slot::Filled(tile) => tile,
        }
    }

    pub fn has_neighbor_n(&self, q: usize, r: usize) -> bool {
        Self::coords_n(q, r).is_some()
    }

    pub fn has_neighbor_nw(&self, q: usize, r: usize) -> bool {
        Self::coords_nw(q, r).is_some()
    }

    pub fn has_neighbor_ne(&self, q: usize, r: usize) -> bool {
        Self::coords_ne(q, r).is_some()
    }

    pub fn has_neighbor_sw(&self, q: usize, r: usize) -> bool {
        Self::coords_sw(q, r).is_some()
    }

    pub fn has_neighbor_se(&self, q: usize, r: usize) -> bool {
        Self::coords_se(q, r).is_some()
    }

    pub fn has_neighbor_s(&self, q: usize, r: usize) -> bool {
        Self::coords_s(q, r).is_some()
    }

    fn to_index(q: usize, r: usize) -> usize {
        (q * R) + r
    }

    pub fn is_even_col(q: usize) -> bool {
        q % 2 == 0
    }

    pub fn is_odd_col(q: usize) -> bool {
        !Self::is_even_col(q)
    }

    pub fn coords_n(q: usize, r: usize) -> Option<Coords> {
        if r < 1 {
            None
        } else {
            Some((q, r-1))
        }
    }

    pub fn coords_nw(q: usize, r: usize) -> Option<Coords> {
        match (q, r) {
            (q, _) if q < 1 => None,
            (q, r) if Self::is_even_col(q) && r < 1 => None,
            (q, r) if Self::is_even_col(q) => Some((q-1, r+1)),
            (q, r) => Some((q-1, r)),
        }
    }

    pub fn coords_ne(q: usize, r: usize) -> Option<Coords> {
        match (q, r) {
            (q, _) if q >= Q-1 => None,
            (q, r) if Self::is_even_col(q) && r < 1 => None,
            (q, r) if Self::is_even_col(q) => Some((q+1, r+1)),
            (q, r) => Some((q+1, r)),
        }
    }

    pub fn coords_sw(q: usize, r: usize) -> Option<Coords> {
        match (q, r) {
            (q, _) if q < 1 => None,
            (q, r) if Self::is_odd_col(q) && r >= R-1 => None,
            (q, r) if Self::is_even_col(q) => Some((q-1, r)),
            (q, r) => Some((q-1, r+1)),
        }
    }

    pub fn coords_se(q: usize, r: usize) -> Option<Coords> {
        match (q, r) {
            (q, _) if q >= Q-1 => None,
            (q, r) if Self::is_odd_col(q) && r >= R-1 => None,
            (q, r) if Self::is_even_col(q) => Some((q+1, r)),
            (q, r) => Some((q+1, r+1)),
        }
    }

    pub fn coords_s(q: usize, r: usize) -> Option<Coords> {
        if r >= R-1 {
            None
        } else {
            Some((q, r+1))
        }
    }
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum Slot {
    Empty,
    Filled(Option<Tile>),
}

pub type Coords = (usize, usize);

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