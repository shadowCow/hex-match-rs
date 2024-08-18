use rand::thread_rng;
use rand::seq::SliceRandom;

use super::board::{Color, Suit};

pub struct Card {
    top: CardSlot,
    bottom: CardSlot,
    points: u8,
}

impl Card {
    pub fn color_color(top: Color, bottom: Color, points: u8) -> Card {
        Card {
            top: CardSlot::Color(top),
            bottom: CardSlot::Color(bottom),
            points,
        }
    }

    pub fn suit_suit(top: Suit, bottom: Suit, points: u8) -> Card {
        Card {
            top: CardSlot::Suit(top),
            bottom: CardSlot::Suit(bottom),
            points,
        }
    }

    pub fn color_suit(top: Color, bottom: Suit, points: u8) -> Card {
        Card {
            top: CardSlot::Color(top),
            bottom: CardSlot::Suit(bottom),
            points,
        }
    }

    pub fn suit_color(top: Suit, bottom: Color, points: u8) -> Card {
        Card {
            top: CardSlot::Suit(top),
            bottom: CardSlot::Color(bottom),
            points,
        }
    }
}

pub enum CardSlot {
    Color(Color),
    Suit(Suit),
}

pub struct CardStack {
    cards: Vec<Card>,
}

impl CardStack {
    pub fn new_easy() -> CardStack {
        let cards: Vec<Card> = vec![
            Card::color_color(Color::Red, Color::Yellow, 1),
            Card::color_color(Color::Red, Color::Yellow, 1),
            Card::color_color(Color::Red, Color::Purple, 1),
            Card::color_color(Color::Red, Color::Purple, 1),
            Card::color_color(Color::Red, Color::Pink, 1),
            Card::color_color(Color::Red, Color::Pink, 1),
            Card::color_color(Color::Red, Color::Red, 2),
            Card::color_color(Color::Red, Color::Red, 2),
            

            Card::color_color(Color::Pink, Color::Yellow, 1),
            Card::color_color(Color::Pink, Color::Yellow, 1),
            Card::color_color(Color::Pink, Color::Purple, 1),
            Card::color_color(Color::Pink, Color::Purple, 1),
            Card::color_color(Color::Pink, Color::Pink, 2),
            Card::color_color(Color::Pink, Color::Pink, 2),

            Card::color_color(Color::Purple, Color::Yellow, 1),
            Card::color_color(Color::Purple, Color::Yellow, 1),
            Card::color_color(Color::Purple, Color::Purple, 2),
            Card::color_color(Color::Purple, Color::Purple, 2),

            Card::color_color(Color::Yellow, Color::Yellow, 2),
            Card::color_color(Color::Yellow, Color::Yellow, 2),
        ];

        Self::new(cards)
    }

    pub fn new_medium() -> CardStack {
        let cards: Vec<Card> = vec![
            Card::color_color(Color::Red, Color::Pink, 2),
            Card::color_color(Color::Red, Color::Pink, 2),
            Card::color_color(Color::Red, Color::Purple, 2),
            Card::color_color(Color::Red, Color::Purple, 2),
            Card::color_color(Color::Red, Color::Yellow, 2),
            Card::color_color(Color::Red, Color::Yellow, 2),
            Card::color_color(Color::Red, Color::Red, 3),
            Card::color_color(Color::Red, Color::Red, 3),

            Card::color_color(Color::Pink, Color::Purple, 2),
            Card::color_color(Color::Pink, Color::Purple, 2),
            Card::color_color(Color::Pink, Color::Yellow, 2),
            Card::color_color(Color::Pink, Color::Yellow, 2),
            Card::color_color(Color::Pink, Color::Pink, 3),
            Card::color_color(Color::Pink, Color::Pink, 3),

            Card::color_color(Color::Purple, Color::Yellow, 2),
            Card::color_color(Color::Purple, Color::Yellow, 2),
            Card::color_color(Color::Purple, Color::Purple, 3),
            Card::color_color(Color::Purple, Color::Purple, 3),

            Card::color_color(Color::Yellow, Color::Yellow, 3),
            Card::color_color(Color::Yellow, Color::Yellow, 3),

            Card::suit_suit(Suit::Club, Suit::Diamond, 2),
            Card::suit_suit(Suit::Club, Suit::Diamond, 2),
            Card::suit_suit(Suit::Club, Suit::Heart, 2),
            Card::suit_suit(Suit::Club, Suit::Heart, 2),
            Card::suit_suit(Suit::Club, Suit::Spade, 2),
            Card::suit_suit(Suit::Club, Suit::Spade, 2),
            Card::suit_suit(Suit::Club, Suit::Club, 3),
            Card::suit_suit(Suit::Club, Suit::Club, 3),

            Card::suit_suit(Suit::Diamond, Suit::Heart, 2),
            Card::suit_suit(Suit::Diamond, Suit::Heart, 2),
            Card::suit_suit(Suit::Diamond, Suit::Spade, 2),
            Card::suit_suit(Suit::Diamond, Suit::Spade, 2),
            Card::suit_suit(Suit::Diamond, Suit::Diamond, 3),
            Card::suit_suit(Suit::Diamond, Suit::Diamond, 3),

            Card::suit_suit(Suit::Heart, Suit::Spade, 2),
            Card::suit_suit(Suit::Heart, Suit::Spade, 2),
            Card::suit_suit(Suit::Heart, Suit::Heart, 3),
            Card::suit_suit(Suit::Heart, Suit::Heart, 3),

            Card::suit_suit(Suit::Spade, Suit::Spade, 3),
            Card::suit_suit(Suit::Spade, Suit::Spade, 3),
        ];

        Self::new(cards)
    }

    pub fn new_hard() -> CardStack {
        let cards: Vec<Card> = vec![
            Card::color_color(Color::Pink, Color::Red, 4),
            Card::color_color(Color::Pink, Color::Red, 4),
            Card::color_color(Color::Pink, Color::Purple, 4),
            Card::color_color(Color::Pink, Color::Purple, 4),
            Card::color_color(Color::Pink, Color::Yellow, 4),
            Card::color_color(Color::Pink, Color::Yellow, 4),
            Card::color_color(Color::Pink, Color::Pink, 5),
            Card::color_color(Color::Pink, Color::Pink, 5),

            Card::color_color(Color::Purple, Color::Red, 4),
            Card::color_color(Color::Purple, Color::Red, 4),
            Card::color_color(Color::Purple, Color::Yellow, 4),
            Card::color_color(Color::Purple, Color::Yellow, 4),
            Card::color_color(Color::Purple, Color::Purple, 5),
            Card::color_color(Color::Purple, Color::Purple, 5),

            Card::color_color(Color::Red, Color::Yellow, 4),
            Card::color_color(Color::Red, Color::Yellow, 4),
            Card::color_color(Color::Red, Color::Red, 5),
            Card::color_color(Color::Red, Color::Red, 5),

            Card::color_color(Color::Yellow, Color::Yellow, 5),
            Card::color_color(Color::Yellow, Color::Yellow, 5),

            Card::suit_suit(Suit::Club, Suit::Diamond, 4),
            Card::suit_suit(Suit::Club, Suit::Diamond, 4),
            Card::suit_suit(Suit::Club, Suit::Heart, 4),
            Card::suit_suit(Suit::Club, Suit::Heart, 4),
            Card::suit_suit(Suit::Club, Suit::Spade, 4),
            Card::suit_suit(Suit::Club, Suit::Spade, 4),
            Card::suit_suit(Suit::Club, Suit::Club, 5),
            Card::suit_suit(Suit::Club, Suit::Club, 5),

            Card::suit_suit(Suit::Diamond, Suit::Heart, 4),
            Card::suit_suit(Suit::Diamond, Suit::Heart, 4),
            Card::suit_suit(Suit::Diamond, Suit::Spade, 4),
            Card::suit_suit(Suit::Diamond, Suit::Spade, 4),
            Card::suit_suit(Suit::Diamond, Suit::Diamond, 5),
            Card::suit_suit(Suit::Diamond, Suit::Diamond, 5),

            Card::suit_suit(Suit::Heart, Suit::Spade, 4),
            Card::suit_suit(Suit::Heart, Suit::Spade, 4),
            Card::suit_suit(Suit::Heart, Suit::Heart, 5),
            Card::suit_suit(Suit::Heart, Suit::Heart, 5),

            Card::suit_suit(Suit::Spade, Suit::Spade, 5),
            Card::suit_suit(Suit::Spade, Suit::Spade, 5),

            Card::color_suit(Color::Red, Suit::Spade, 4),
            Card::color_suit(Color::Red, Suit::Spade, 4),
            Card::color_suit(Color::Pink, Suit::Spade, 4),
            Card::color_suit(Color::Pink, Suit::Spade, 4),
            Card::color_suit(Color::Purple, Suit::Spade, 4),
            Card::color_suit(Color::Purple, Suit::Spade, 4),
            Card::suit_color(Suit::Spade, Color::Yellow, 4),
            Card::suit_color(Suit::Spade, Color::Yellow, 4),

            Card::suit_color(Suit::Club, Color::Red, 4),
            Card::suit_color(Suit::Club, Color::Red, 4),
            Card::suit_color(Suit::Club, Color::Pink, 4),
            Card::suit_color(Suit::Club, Color::Pink, 4),
            Card::suit_color(Suit::Club, Color::Purple, 4),
            Card::suit_color(Suit::Club, Color::Purple, 4),
            Card::suit_color(Suit::Club, Color::Yellow, 4),
            Card::suit_color(Suit::Club, Color::Yellow, 4),

            Card::suit_color(Suit::Diamond, Color::Red, 4),
            Card::suit_color(Suit::Diamond, Color::Red, 4),
            Card::suit_color(Suit::Diamond, Color::Pink, 4),
            Card::suit_color(Suit::Diamond, Color::Pink, 4),
            Card::suit_color(Suit::Diamond, Color::Purple, 4),
            Card::suit_color(Suit::Diamond, Color::Purple, 4),
            Card::suit_color(Suit::Diamond, Color::Yellow, 4),
            Card::suit_color(Suit::Diamond, Color::Yellow, 4),

            Card::suit_color(Suit::Heart, Color::Red, 4),
            Card::suit_color(Suit::Heart, Color::Red, 4),
            Card::suit_color(Suit::Heart, Color::Pink, 4),
            Card::suit_color(Suit::Heart, Color::Pink, 4),
            Card::suit_color(Suit::Heart, Color::Purple, 4),
            Card::suit_color(Suit::Heart, Color::Purple, 4),
            Card::suit_color(Suit::Heart, Color::Yellow, 4),
            Card::suit_color(Suit::Heart, Color::Yellow, 4),
        ];

        Self::new(cards)
    }

    fn new(mut cards: Vec<Card>) -> CardStack {
        let mut rng = thread_rng();
        cards.shuffle(&mut rng);

        CardStack { cards }
    }

    pub fn draw(&mut self) -> Option<Card> {
        self.cards.pop()
    }
}
