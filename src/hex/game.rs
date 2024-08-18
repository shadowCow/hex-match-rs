use rand::thread_rng;
use rand::seq::SliceRandom;

use super::{board::{Board, Color, Suit, Tile}, card::CardStack};

pub struct Game {
    board: Board,
    tile_bag: TileBag,
    garden: Garden,
    easy_stack: CardStack,
    medium_stack: CardStack,
    hard_stack: CardStack,
}

impl Game {
    pub fn new() -> Game {
        let mut tile_bag = TileBag::new();
        let g1 = tile_bag.draw();
        let g2 = tile_bag.draw();
        let g3 = tile_bag.draw();
        let g4 = tile_bag.draw();

        let garden = Garden::new(g1, g2, g3, g4);

        let board = Board::new();

        Game {
            board,
            tile_bag,
            garden,
            easy_stack: CardStack::new_easy(),
            medium_stack: CardStack::new_medium(),
            hard_stack: CardStack::new_hard(),
        }
    }

    pub fn make_move(&mut self, m: GameMove) {

    }
}

pub enum GameMove {
    PlaceTile { tile: Tile, index: usize },
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
    tiles: [Tile; 4],
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
    }
}