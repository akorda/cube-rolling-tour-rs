use std::io::{self, IsTerminal, Write};

pub mod dice;
pub mod red_faced_cube_solver;
pub mod reentrant_tour_solver;

pub const SIZE: usize = 8;

#[derive(Clone, PartialEq)]
pub enum Direction {
    Up,
    Right,
    Down,
    Left,
}

#[derive(Copy, Clone)]
pub struct Point {
    pub row: usize,
    pub col: usize,
}

pub struct Solution {
    pub rolls: Vec<Direction>,
}

pub fn print_board(start: Point, rolls: &Vec<Direction>, redraw_in_place: bool) {
    const UP: u8 = 1;
    const RIGHT: u8 = 2;
    const DOWN: u8 = 4;
    const LEFT: u8 = 8;

    // For each cell, the sides through which the path enters or leaves it
    let mut links = [[0u8; SIZE]; SIZE];

    let mut p = Point {
        row: start.row,
        col: start.col,
    };
    for roll in rolls {
        let (next, exit, entry) = match roll {
            Direction::Up => (
                Point {
                    row: p.row - 1,
                    col: p.col,
                },
                UP,
                DOWN,
            ),
            Direction::Down => (
                Point {
                    row: p.row + 1,
                    col: p.col,
                },
                DOWN,
                UP,
            ),
            Direction::Left => (
                Point {
                    row: p.row,
                    col: p.col - 1,
                },
                LEFT,
                RIGHT,
            ),
            Direction::Right => (
                Point {
                    row: p.row,
                    col: p.col + 1,
                },
                RIGHT,
                LEFT,
            ),
        };

        links[p.row][p.col] |= exit;
        links[next.row][next.col] |= entry;
        p = next;
    }

    // A board coordinate wider than one digit is shown by its last digit only,
    // so the row/column labels still fit in a single character.
    fn last_digit(n: usize) -> char {
        char::from_digit((n % 10) as u32, 10).unwrap()
    }

    // Cells sit on even columns; odd columns hold the horizontal connectors
    let mut output = String::new();

    // column header
    output.push_str("  ");
    for col in 0..SIZE {
        output.push(last_digit(col));
        if col < SIZE - 1 {
            output.push(' ');
        }
    }
    output.push('\n');

    for row in 0..SIZE {
        output.push(last_digit(row));
        output.push(' ');
        for col in 0..SIZE {
            output.push(match links[row][col] {
                x if x == UP | DOWN => '│',
                x if x == LEFT | RIGHT => '─',
                x if x == DOWN | RIGHT => '┌',
                x if x == DOWN | LEFT => '┐',
                x if x == UP | RIGHT => '└',
                x if x == UP | LEFT => '┘',
                UP => '╵',
                DOWN => '╷',
                LEFT => '╴',
                RIGHT => '╶',
                _ => ' ',
            });

            if col < SIZE - 1 {
                output.push(if links[row][col] & RIGHT != 0 {
                    '─'
                } else {
                    ' '
                });
            }
        }
        output.push('\n');
    }

    let mut stdout = io::stdout();

    // Redraw in place when writing to a terminal
    if redraw_in_place && stdout.is_terminal() {
        // move cursor to the top-left corner
        print!("\x1B[H");
    }

    print!("{}", output);
    stdout.flush().unwrap();
}
