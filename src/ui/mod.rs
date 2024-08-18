use std::io::{stdin, stdout, Write};

use crate::hex::{board::{Color, Suit, Tile}, game::{Game, GameMove, Garden}};

pub fn run() {
    let mut game = Game::new();

    loop {
        println!("Enter moves, type 'help' for help or type 'exit' to quit): ");
        println!("");

        draw_game(&game);
        println!("");

        stdout().flush().expect("Failed to flush");

        let mut input = String::new();

        stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let input = input.trim();

        if input == "exit" {
            println!("Exiting...");
            break;
        }

        println!("You entered: {}", input);
    }
}

fn parse_move(move_text: &str) -> Option<GameMove> {

    None
}

fn draw_game(game: &Game) {
    let hex_one = render_hex(&game.board.tiles[0], 0);
    let garden = render_garden(&game.garden);

    println!("{}", hex_one);
    println!("");
    println!("{}", garden);
}

fn render_hex(tile: &Option<Tile>, index: usize) -> String {
    let top = " ___ ";
    let tile_row = match tile {
        Some(t) => format!("/ {} \\", render_tile(t)),
        None => "/   \\".to_owned(),
    };
    let index_row = if index < 10 {
        format!("| {} |", index)
    } else {
        format!("|{} |", index)
    };
    let filler_row = "\\   /";
    let bottom = " \u{203E}\u{203E}\u{203E} ";

    format!(
        "{}\n{}\n{}\n{}\n{}\n",
        top,
        tile_row,
        index_row,
        filler_row,
        bottom,
    )
}

fn render_garden(garden: &Garden) -> String {
    let border = "-".repeat(11);
    format!(
        "{}\n| {} {} {} {} |\n{}",
        border,
        render_tile(&garden.tiles[0]),
        render_tile(&garden.tiles[1]),
        render_tile(&garden.tiles[2]),
        render_tile(&garden.tiles[3]),
        border,
    )
}

fn render_tile(tile: &Tile) -> String {
    let suit = render_suit(&tile.suit);

    match &tile.color {
        Color::Red => suit.red(),
        Color::Pink => suit.pink(),
        Color::Purple => suit.purple(),
        Color::Yellow => suit.yellow(),
    }
}

fn render_suit(suit: &Suit) -> String {
    match suit {
        Suit::Club => "\u{2663}".to_owned(),
        Suit::Diamond => "\u{2666}".to_owned(),
        Suit::Heart => "\u{2665}".to_owned(),
        Suit::Spade => "\u{2660}".to_owned(),
    }
}

const RED: &str = "\x1b[31m";
const PINK: &str = "\x1b[95m";
const PURPLE: &str = "\x1b[35m";
const YELLOW: &str = "\x1b[33m";
const COLOR_RESET: &str = "\x1b[0m";

trait ColorizeString {
    fn red(&self) -> String;
    fn pink(&self) -> String;
    fn purple(&self) -> String;
    fn yellow(&self) -> String;
}

impl ColorizeString for str {
    fn red(&self) -> String {
        format!("{}{}{}", RED, self, COLOR_RESET)
    }

    fn pink(&self) -> String {
        format!("{}{}{}", PINK, self, COLOR_RESET)
    }

    fn purple(&self) -> String {
        format!("{}{}{}", PURPLE, self, COLOR_RESET)
    }

    fn yellow(&self) -> String {
        format!("{}{}{}", YELLOW, self, COLOR_RESET)
    }
}
