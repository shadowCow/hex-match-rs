use rand::thread_rng;
use rand::seq::SliceRandom;

use super::{card::CardStack, grid::HexGrid, tile::{Color, Suit, Tile}};

pub struct Game {
    pub grid: HexGrid,
    pub tile_bag: TileBag,
    pub garden: Garden,
    pub easy_stack: CardStack,
    pub medium_stack: CardStack,
    pub hard_stack: CardStack,
}

impl Game {
    pub fn new() -> Game {
        let mut tile_bag = TileBag::new();
        let g1 = tile_bag.draw();
        let g2 = tile_bag.draw();
        let g3 = tile_bag.draw();
        let g4 = tile_bag.draw();

        let garden = Garden::new(g1, g2, g3, g4);

        let mut grid = HexGrid::new();
        grid.place(Tile::preset_one(), 0, 1);
        grid.place(Tile::preset_two(), 4, 1);
        grid.place(Tile::preset_three(), 0, 4);
        grid.place(Tile::preset_four(), 4, 4);
        grid.place(Tile::preset_five(), 2, 2);
        grid.place(Tile::preset_six(), 1, 2);
        grid.place(Tile::preset_seven(), 3, 2);
        grid.place(Tile::preset_eight(), 2, 3);

        Game {
            grid,
            tile_bag,
            garden,
            easy_stack: CardStack::new_easy(),
            medium_stack: CardStack::new_medium(),
            hard_stack: CardStack::new_hard(),
        }
    }

    pub fn make_move(&mut self, m: GameMove) {
        match m {
            GameMove::PlaceTile { tile, q, r } => {
                self.grid.place(tile, q, r);
            },
            _ => {},
        }
    }
}

pub enum GameMove {
    PlaceTile { tile: Tile, q: usize, r: usize },
    PlaceCubes { player: Player, count: u8 },
}

pub enum Player {
    P1,
    P2,
    P3,
    P4,
    P5,
}

pub struct Garden {
    pub tiles: [Tile; 4],
}

impl Garden {
    pub fn new(one: Tile, two: Tile, three: Tile, four: Tile) -> Garden {
        Garden {
            tiles: [
                one,
                two,
                three,
                four,
            ]
        }
    }

    pub fn replenish(&mut self, index: usize, tile: Tile) {
        self.tiles[index] = tile;
    }
}

pub struct TileBag {
    tiles: Vec<Tile>,
}

impl TileBag {
    pub fn new() -> TileBag {
        let mut tiles: Vec<Tile> = vec![
            Tile::new(Color::Purple, Suit::Heart),
            Tile::new(Color::Purple, Suit::Heart),
            Tile::new(Color::Purple, Suit::Diamond),
            Tile::new(Color::Purple, Suit::Spade),
            Tile::new(Color::Purple, Suit::Club),
            Tile::new(Color::Purple, Suit::Club),

            Tile::new(Color::Pink, Suit::Heart),
            Tile::new(Color::Pink, Suit::Heart),
            Tile::new(Color::Pink, Suit::Diamond),
            Tile::new(Color::Pink, Suit::Spade),
            Tile::new(Color::Pink, Suit::Spade),
            Tile::new(Color::Pink, Suit::Club),

            Tile::new(Color::Red, Suit::Heart),
            Tile::new(Color::Red, Suit::Diamond),
            Tile::new(Color::Red, Suit::Diamond),
            Tile::new(Color::Red, Suit::Spade),
            Tile::new(Color::Red, Suit::Club),
            Tile::new(Color::Red, Suit::Club),

            Tile::new(Color::Yellow, Suit::Heart),
            Tile::new(Color::Yellow, Suit::Diamond),
            Tile::new(Color::Yellow, Suit::Diamond),
            Tile::new(Color::Yellow, Suit::Spade),
            Tile::new(Color::Yellow, Suit::Spade),
            Tile::new(Color::Yellow, Suit::Club),
        ];

        let mut rng = thread_rng();
        tiles.shuffle(&mut rng);

        TileBag { tiles }
    }

    pub fn draw(&mut self) -> Tile {
        self.tiles.pop().unwrap()
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_game() {
        let game = Game::new();

        // grid.place(Tile::preset_one(), 0, 1);
        // grid.place(Tile::preset_two(), 4, 1);
        // grid.place(Tile::preset_three(), 0, 4);
        // grid.place(Tile::preset_four(), 4, 4);
        // grid.place(Tile::preset_five(), 2, 2);
        // grid.place(Tile::preset_six(), 1, 2);
        // grid.place(Tile::preset_seven(), 3, 2);
        // grid.place(Tile::preset_eight(), 2, 3);

        assert_eq!(game.grid.item_at(0, 1), &Some(Tile::preset_one()));
        assert_eq!(game.grid.item_at(4, 1), &Some(Tile::preset_two()));
        assert_eq!(game.grid.item_at(0, 4), &Some(Tile::preset_three()));
        assert_eq!(game.grid.item_at(4, 4), &Some(Tile::preset_four()));
        assert_eq!(game.grid.item_at(2, 2), &Some(Tile::preset_five()));
        assert_eq!(game.grid.item_at(1, 2), &Some(Tile::preset_six()));
        assert_eq!(game.grid.item_at(3, 2), &Some(Tile::preset_seven()));
        assert_eq!(game.grid.item_at(2, 3), &Some(Tile::preset_eight()));
    }
}