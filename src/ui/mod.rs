use std::io::{stdin, stdout, Write};

use crate::hex::{game::{Game, GameMove, Garden}, grid::{Coords, HexGrid, Slot}, tile::{Color, Suit, Tile}};

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
    // let hex_one = render_hex(&game.grid.item_at(0, 1), 0);
    draw_hex_grid(&game.grid);

    let garden = render_garden(&game.garden);

    // println!("{}", hex_one);
    println!("");
    println!("{}", garden);
}

fn render_hex(tile: &Option<Tile>, index: usize) -> String {
    let top = "  _____  ";
    let body_top = " /     \\ ";
    let tile_row = match tile {
        Some(t) => format!("/   {}   \\", render_tile(t)),
        None => "/       \\".to_owned(),
    };
    let index_row = if index < 10 {
        format!("\\   {}   /", index)
    } else {
        format!("\\  {}   /", index)
    };
    let bottom = " \\_____/ ";

    format!(
        "{}\n{}\n{}\n{}\n{}\n",
        top,
        body_top,
        tile_row,
        index_row,
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

fn draw_hex_grid(grid: &HexGrid) {
    // full hex has 5 lines
    // a non-top hex has 4 lines

    // every hex will render 4 lines.
    // each hex is 9 characters.

    // we will render in 'half-rows'.
    // so we render 2 lines
    // then render 2 lines

    // this means the only special handling we need
    // is to render the 'top border'

    let top_border = render_top_border(grid);
    println!("{}", top_border);

    for r in 0..grid.r() {
        let top_half = render_row_top_half(grid, r);
        let bottom_half = render_row_bottom_half(grid, r);

        println!("{}\n{}", top_half, bottom_half);
    }
}

fn render_top_border(grid: &HexGrid) -> String {
    let mut border = String::new();
    let blank_even = " ".repeat(5);
    let blank_odd = " ".repeat(9);
    let filled = format!("{}", "_".repeat(5));

    border.push_str("  ");
    for q in 0..grid.q() {
        let hex_top = if HexGrid::is_even_col(q) {
            match grid.hex_at(q, 0) {
                Slot::Empty => &blank_even,
                _ => &filled,
            }
        } else {
            &blank_odd
        };

        border.push_str(hex_top);
    }
    border.push_str("  ");
    
    border
}
    
fn render_row_top_half(grid: &HexGrid, r: usize) -> String {
    // if the hex has a neighbor to NW
    // do not render top left border

    // hex always renders its right side

    let mut line_one = String::new();
    let mut line_two = String::new();

    for q in 0..grid.q() {
        if HexGrid::is_even_col(q) {
            line_one.push_str(&render_hex_line_one(grid, q, r));
            line_two.push_str(&render_hex_line_two(grid, q, r));
        } else {
            if r == 0 {
                line_one.push_str(&render_odd_top_line_three(grid, q));
                line_two.push_str(&render_odd_top_line_four(grid, q));
            } else {
                line_one.push_str(&render_hex_line_three(grid, q, r-1));
                line_two.push_str(&render_hex_line_four(grid, q, r-1));
            }
        }
    }

    format!("{}\n{}", line_one, line_two)
}

fn render_row_bottom_half(grid: &HexGrid, r: usize) -> String {
    // if the hex has a neighbor to SW
    // do not render bottom left border

    // hex always renders its bottom

    let mut line_one = String::new();
    let mut line_two = String::new();

    for q in 0..grid.q() {
        if HexGrid::is_even_col(q) {
            // if r >= grid.r() - 1 {
            //     line_one.push_str(&" ".repeat(9));
            //     line_two.push_str(&" ".repeat(9));
            // }

            line_one.push_str(&render_hex_line_three(grid, q, r));
            line_two.push_str(&render_hex_line_four(grid, q, r));
        } else {
            line_one.push_str(&render_hex_line_one(grid, q, r));
            line_two.push_str(&render_hex_line_two(grid, q, r));
        }
    }

    format!("{}\n{}", line_one, line_two)
}

fn render_hex_line_one(grid: &HexGrid, q: usize, r: usize) -> String {
    // hex draws everything but left-hand border
    // special case, column 0, must draw its left-hand border

    let mut line = String::new();
    let slot = grid.hex_at(q, r);
    if q == 0 {
        let left_border = match slot {
            Slot::Empty => "  ",
            _ => " /",
        };
        line.push_str(left_border);
    }

    let inside = " ".repeat(5);
    line.push_str(&inside);

    let ne_neighbor = HexGrid::coords_ne(q, r)
        .map(|(n_q, n_r)| grid.hex_at(n_q, n_r));
    let right_border = match slot {
        Slot::Empty => match ne_neighbor {
            Some(Slot::Filled(_)) => "\\",
            _ => " ",
        },
        _ => "\\",
    };
    line.push_str(right_border);

    let right_padding = if q == grid.q() - 1 {
        " "
    } else {
        ""
    };
    line.push_str(right_padding);

    line
}

fn render_hex_line_two(grid: &HexGrid, q: usize, r: usize) -> String {
    // hex draws everything but left-hand border
    // special case, column 0, must draw its left-hand border

    let mut line = String::new();
    let slot = grid.hex_at(q, r);
    if q == 0 {
        let left_border = match slot {
            Slot::Empty => " ",
            _ => "/",
        };
        line.push_str(left_border);
    }

    let inside = match slot {
        Slot::Empty => " ".repeat(7),
        _ => format!("  {},{}  ", q, r),
    };
    line.push_str(&inside);

    let ne_neighbor = HexGrid::coords_ne(q, r)
        .map(|(n_q, n_r)| grid.hex_at(n_q, n_r));
    let right_border = match slot {
        Slot::Empty => match ne_neighbor {
            Some(Slot::Filled(_)) => "\\",
            _ => " ",
        },
        _ => "\\",
    };
    line.push_str(right_border);

    let right_padding = if q == grid.q() - 1 {
        " "
    } else {
        ""
    };
    line.push_str(right_padding);

    line
}

fn render_hex_line_three(grid: &HexGrid, q: usize, r: usize) -> String {
    let mut line = String::new();
    let slot = grid.hex_at(q, r);
    if q == 0 {
        let left_border = match slot {
            Slot::Empty => " ",
            _ => "\\",
        };
        line.push_str(left_border);
    }

    let inside = match slot {
        Slot::Empty => " ".repeat(7),
        Slot::Filled(tile) => match tile {
            Some(t) => format!("   {}   ", render_tile(t)),
            None => " ".repeat(7),
        }
    };
    line.push_str(&inside);

    let se_neighbor = HexGrid::coords_se(q, r)
        .map(|(n_q, n_r)| grid.hex_at(n_q, n_r));
    let right_border = match slot {
        Slot::Empty => match se_neighbor {
            Some(Slot::Filled(_)) => "/",
            _ => " ",
        },
        _ => "/",
    };
    line.push_str(&right_border);

    let right_padding = if q == grid.q() - 1 {
        " "
    } else {
        ""
    };
    line.push_str(right_padding);

    line
}

fn render_hex_line_four(grid: &HexGrid, q: usize, r: usize) -> String {
    let mut line = String::new();
    let slot = grid.hex_at(q, r);
    if q == 0 {
        let left_border = match slot {
            Slot::Empty => "  ",
            _ => " \\",
        };
        line.push_str(left_border);
    }

    let s_neighbor = HexGrid::coords_s(q, r);
    let inside = match slot {
        Slot::Empty if s_neighbor.is_none() => " ".repeat(5),
        _ => "_".repeat(5)
    };
    line.push_str(&inside);

    let se_neighbor = HexGrid::coords_se(q, r)
        .map(|(n_q, n_r)| grid.hex_at(n_q, n_r));
    let right_border = match slot {
        Slot::Empty => match se_neighbor {
            Some(Slot::Filled(_)) => "/",
            _ => " ",
        },
        _ => "/",
    };
    line.push_str(&right_border);

    let right_padding = if q == grid.q() - 1 {
        " "
    } else {
        ""
    };
    line.push_str(right_padding);

    line
}

fn render_odd_top_line_three(grid: &HexGrid, q: usize) -> String {
    let inside = " ".repeat(7);

    let sw_neighbor = grid.hex_at(q+1, 0);
    let right = match sw_neighbor {
        Slot::Empty => " ",
        _ => "/",
    };

    format!("{}{}", inside, right)
}

fn render_odd_top_line_four(grid: &HexGrid, q: usize) -> String {
    let s_neighbor = grid.hex_at(q, 0);
    let inside = match s_neighbor {
        Slot::Empty => " ".repeat(5),
        _ => "_".repeat(5),
    };

    let sw_neighbor = grid.hex_at(q+1, 0);
    let right = match sw_neighbor {
        Slot::Empty => " ",
        _ => "/",
    };

    format!("{}{}", inside, right)
}

fn border_bottom() -> String {
    "_".repeat(5)
}

fn line_one_interior() -> String {
    " ".repeat(5)
}

fn line_two_interior(coords: &Option<Coords>) -> String {
    match coords {
        Some((q, r)) => format!("  {},{}  ", q, r),
        None => " ".repeat(7),
    }
}

fn line_three_interior(tile: &Option<Tile>) -> String {
    match tile {
        Some(t) => format!("   {}   ", render_tile(&t)),
        None => " ".repeat(7),
    }
}



/*
hex's widest point is 9 characters.

hex's narrowest point is 7 characters.

when rendering a single hex,
one of its lines,
i don't include outside spaces.

 /     \

                _____
               /     \
         _____/       \_____
        /     \       /     \
  _____/       \_____/       \_____  
 /     \       /     \       /     \
/       \_____/       \_____/       \
\       /     \       /     \       /
 \_____/       \_____/       \_____/

*/

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_render_top_border() {
        let game = Game::new();

        let expected = format!(
            "  {}{}{}{}{}  ",
            " ".repeat(5),
            " ".repeat(9),
            "_".repeat(5),
            " ".repeat(9),
            " ".repeat(5),
        );
        
        assert_eq!(render_top_border(&game.grid), expected);
    }

    #[test]
    fn test_render_row_top_half() {
        let game = Game::new();

        let expected = format!(
            "  {}{}{}{}{} ",
            " ".repeat(6),
            " ".repeat(9),
            "_".repeat(5),
            " ".repeat(9),
            " ".repeat(5),
        );
        
        assert_eq!(render_row_top_half(&game.grid, 0), expected);
    }
}