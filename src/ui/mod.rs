use std::io::{stdin, stdout, Write};

use crate::hex::game::{Game, GameMove};

pub fn run() {
    let mut game = Game::new();

    loop {
        print!("Enter moves in format '<board_index> <garden_index> (or type 'exit' to quit): ");
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

fn draw_game() {

}