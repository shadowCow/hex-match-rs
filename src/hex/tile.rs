
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
