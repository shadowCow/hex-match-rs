#[derive(Debug, PartialEq, Eq)]
pub struct Board {
    pub tiles: [Option<Tile>; 24],
    edges: [Vec<usize>; 24],
}

impl Board {
    pub fn new() -> Board {
        let edges: [Vec<usize>; 24] = [
            vec![1, 5, 6],
            vec![0, 2, 6, 7],
            vec![1, 3, 7],
            vec![2, 4, 7, 8],
            vec![3, 8, 9],
            vec![0, 6, 10, 11],
            vec![0, 1, 5, 7, 11, 12],
            vec![1, 2, 3, 6, 8, 12],
            vec![3, 4, 7, 9, 12, 13],
            vec![4, 8, 13, 14],
            vec![5, 11, 15, 16],
            vec![5, 6, 10, 12, 16, 17],
            vec![6, 7, 8, 11, 13, 17],
            vec![8, 9, 12, 14, 17, 18],
            vec![9, 13, 18, 19],
            vec![10, 16, 20],
            vec![10, 11, 15, 17, 20, 21],
            vec![11, 12, 13, 16, 18, 21],
            vec![13, 14, 17, 19, 21, 22],
            vec![14, 18, 22],
            vec![15, 16, 21],
            vec![16, 17, 18, 20, 22, 23],
            vec![18, 19, 21, 23],
            vec![20, 21, 22],
        ];


        Board {
            tiles: [
                // tiles 0, 4, 11, 12, 13, 15, 17, 19 are preset
                Some(Tile::preset_one()),
                None,
                None,
                None,
                Some(Tile::preset_two()),
                None,
                None,
                None,
                None,
                None,
                None,
                Some(Tile::preset_six()),
                Some(Tile::preset_five()),
                Some(Tile::preset_seven()),
                None,
                Some(Tile::preset_three()),
                None,
                Some(Tile::preset_eight()),
                None,
                Some(Tile::preset_four()),
                None,
                None,
                None,
                None,
            ],
            edges,
        }
    }

    pub fn place_tile(&mut self, tile: Tile, space: usize) {
        self.tiles[space] = Some(tile);
    }
}


#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct Tile {
    pub color: Color,
    pub suit: Suit,
}

impl Tile {
    pub fn new(color: Color, suit: Suit) -> Tile {
        Tile { color, suit }
    }

    pub fn preset_one() -> Tile {
        Tile::new(Color::Red, Suit::Spade)
    }

    pub fn preset_two() -> Tile {
        Tile::new(Color::Yellow, Suit::Heart)
    }

    pub fn preset_three() -> Tile {
        Tile::new(Color::Purple, Suit::Diamond)
    }

    pub fn preset_four() -> Tile {
        Tile::new(Color::Pink, Suit::Club)
    }

    pub fn preset_five() -> Tile {
        Tile::new(Color::Red, Suit::Heart)
    }

    pub fn preset_six() -> Tile {
        Tile::new(Color::Purple, Suit::Spade)
    }

    pub fn preset_seven() -> Tile {
        Tile::new(Color::Yellow, Suit::Club)
    }

    pub fn preset_eight() -> Tile {
        Tile::new(Color::Pink, Suit::Diamond)
    }
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum Suit {
    Club,
    Diamond,
    Heart,
    Spade
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum Color {
    Yellow,
    Pink,
    Purple,
    Red,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_board() {
        let board = Board::new();

        assert_eq!(board.tiles[0], Some(Tile::preset_one()));
        assert_eq!(board.tiles[0], Some(Tile::preset_one()));
        assert_eq!(board.tiles[0], Some(Tile::preset_one()));
        assert_eq!(board.tiles[0], Some(Tile::preset_one()));
        assert_eq!(board.tiles[0], Some(Tile::preset_one()));
        assert_eq!(board.tiles[0], Some(Tile::preset_one()));
        assert_eq!(board.tiles[0], Some(Tile::preset_one()));
        assert_eq!(board.tiles[0], Some(Tile::preset_one()));
    }

    #[test]
    fn test_place_tile() {
        let mut board = Board::new();

        let tile = Tile { suit: Suit::Heart, color: Color::Yellow };
        let space = 1;
        board.place_tile(tile, space);

        assert_eq!(board.tiles[space], Some(tile));
    }
}