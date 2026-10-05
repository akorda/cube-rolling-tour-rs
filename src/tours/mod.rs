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

fn are_cells_reachable(
    board: &[[bool; SIZE]; SIZE],
    start_row: usize,
    start_col: usize,
    end_row: usize,
    end_col: usize,
) -> bool {
    let mut examined = [[false; SIZE]; SIZE];
    are_cells_reachable_int(board, &mut examined, start_row, start_col, end_row, end_col)
}

fn are_cells_reachable_int(
    board: &[[bool; SIZE]; SIZE],
    examined: &mut [[bool; SIZE]; SIZE],
    start_row: usize,
    start_col: usize,
    end_row: usize,
    end_col: usize,
) -> bool {
    if start_row == end_row && start_col == end_col {
        return true;
    }

    if examined[start_row][start_col] {
        return false;
    }

    examined[start_row][start_col] = true;

    let visited = board[start_row][start_col];
    if visited {
        return false;
    }

    // top
    if start_row != 0
        && are_cells_reachable_int(board, examined, start_row - 1, start_col, end_row, end_col)
    {
        return true;
    }

    // bottom
    if start_row != SIZE - 1
        && are_cells_reachable_int(board, examined, start_row + 1, start_col, end_row, end_col)
    {
        return true;
    }

    // left
    if start_col != 0
        && are_cells_reachable_int(board, examined, start_row, start_col - 1, end_row, end_col)
    {
        return true;
    }

    // right
    if start_col != SIZE - 1
        && are_cells_reachable_int(board, examined, start_row, start_col + 1, end_row, end_col)
    {
        return true;
    }

    return false;
}

#[test]
fn test_are_cells_reachable_1() {
    let board_def = r#"
00001000
00010000
00100000
00100000
11100000
00000000
00000000
00000000
"#;
    let mut board = [[false; SIZE]; SIZE];
    parse_board(board_def, &mut board);

    // col #0
    let mut reachable = are_cells_reachable(&board, 0, 0, 0, 7);
    assert_eq!(reachable, false);

    reachable = are_cells_reachable(&board, 1, 0, 0, 7);
    assert_eq!(reachable, false);

    reachable = are_cells_reachable(&board, 2, 0, 0, 7);
    assert_eq!(reachable, false);

    reachable = are_cells_reachable(&board, 3, 0, 0, 7);
    assert_eq!(reachable, false);

    reachable = are_cells_reachable(&board, 5, 0, 0, 7);
    assert_eq!(reachable, true);

    reachable = are_cells_reachable(&board, 6, 0, 0, 7);
    assert_eq!(reachable, true);

    reachable = are_cells_reachable(&board, 7, 0, 0, 7);
    assert_eq!(reachable, true);

    // col #1
    reachable = are_cells_reachable(&board, 0, 1, 0, 7);
    assert_eq!(reachable, false);

    reachable = are_cells_reachable(&board, 1, 1, 0, 7);
    assert_eq!(reachable, false);

    reachable = are_cells_reachable(&board, 2, 1, 0, 7);
    assert_eq!(reachable, false);

    reachable = are_cells_reachable(&board, 3, 1, 0, 7);
    assert_eq!(reachable, false);

    // col #2
    reachable = are_cells_reachable(&board, 0, 2, 0, 7);
    assert_eq!(reachable, false);

    reachable = are_cells_reachable(&board, 1, 2, 0, 7);
    assert_eq!(reachable, false);

    // col #3
    reachable = are_cells_reachable(&board, 0, 3, 0, 7);
    assert_eq!(reachable, false);
}

#[test]
fn test_are_cells_reachable_2() {
    let board_def = r#"
00001000
00010000
00100000
00100000
00100000
00010000
00001000
00001100
"#;
    let mut board = [[false; SIZE]; SIZE];
    parse_board(board_def, &mut board);

    // col #0
    let mut reachable = are_cells_reachable(&board, 0, 0, 0, 7);
    assert_eq!(reachable, false);

    reachable = are_cells_reachable(&board, 1, 0, 0, 7);
    assert_eq!(reachable, false);

    reachable = are_cells_reachable(&board, 2, 0, 0, 7);
    assert_eq!(reachable, false);

    reachable = are_cells_reachable(&board, 3, 0, 0, 7);
    assert_eq!(reachable, false);

    reachable = are_cells_reachable(&board, 5, 0, 0, 7);
    assert_eq!(reachable, false);

    reachable = are_cells_reachable(&board, 6, 0, 0, 7);
    assert_eq!(reachable, false);

    reachable = are_cells_reachable(&board, 7, 0, 0, 7);
    assert_eq!(reachable, false);

    // col #1
    reachable = are_cells_reachable(&board, 0, 1, 0, 7);
    assert_eq!(reachable, false);

    reachable = are_cells_reachable(&board, 1, 1, 0, 7);
    assert_eq!(reachable, false);

    reachable = are_cells_reachable(&board, 2, 1, 0, 7);
    assert_eq!(reachable, false);

    reachable = are_cells_reachable(&board, 3, 1, 0, 7);
    assert_eq!(reachable, false);

    // col #2
    reachable = are_cells_reachable(&board, 0, 2, 0, 7);
    assert_eq!(reachable, false);

    reachable = are_cells_reachable(&board, 1, 2, 0, 7);
    assert_eq!(reachable, false);

    // col #3
    reachable = are_cells_reachable(&board, 0, 3, 0, 7);
    assert_eq!(reachable, false);
}

#[cfg(test)]
fn parse_board(board_def: &str, board: &mut [[bool; SIZE]; SIZE]) {
    let rows: Vec<&str> = board_def.trim().split('\n').collect();
    for r in 0..8 {
        let row = rows[r];
        let chars: Vec<char> = row.chars().collect();
        for c in 0..8 {
            board[r][c] = chars[c] == '1';
        }
    }
}
