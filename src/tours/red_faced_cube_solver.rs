use super::dice::Dice;
use super::{Direction, Point, SIZE, Solution};

#[derive(Clone)]
pub struct RedFacedCubeSolver {
    board: [[bool; SIZE]; SIZE],
    not_visited: u8,
    dice: Dice,
    current_pos: Point,
    rolls: Vec<Direction>,
}

impl RedFacedCubeSolver {
    pub fn new() -> Self {
        let mut board = [[false; SIZE]; SIZE];
        board[0][0] = true;

        RedFacedCubeSolver {
            board,
            not_visited: (SIZE as u8) * (SIZE as u8) - 1,
            dice: Dice::new(),
            current_pos: Point { row: 0, col: 0 },
            rolls: Vec::new(),
        }
    }

    pub fn solve(&mut self) -> Option<Solution> {
        super::print_board(Point { row: 0, col: 0 }, &self.rolls, true);

        // we just found a solution!
        if self.not_visited == 0 && self.current_pos.row == 0 && self.current_pos.col == SIZE - 1 {
            return Some(Solution {
                rolls: self.rolls.clone(),
            });
        }

        if !self.is_end_reachable() {
            return None;
        }

        let can_roll_up = self.can_roll_up();
        let can_roll_down = self.can_roll_down();
        let can_roll_left = self.can_roll_left();
        let can_roll_right = self.can_roll_right();

        let mut no_rolls = 0;
        if can_roll_up {
            no_rolls = no_rolls + 1;
        }
        if can_roll_down {
            no_rolls = no_rolls + 1;
        }
        if can_roll_left {
            no_rolls = no_rolls + 1;
        }
        if can_roll_right {
            no_rolls = no_rolls + 1;
        }

        if no_rolls == 0 {
            return None;
        } else if no_rolls == 1 {
            // no need to clone a solution. Just continue with this one
            if can_roll_right {
                self.roll_right();
            } else if can_roll_down {
                self.roll_down();
            } else if can_roll_left {
                self.roll_left();
            } else {
                self.roll_up();
            }

            return self.solve();
        } else {
            if can_roll_right {
                let mut clone = self.clone();
                clone.roll_right();
                let solution = clone.solve();
                if solution.is_some() {
                    return solution;
                }
            }

            if can_roll_down {
                let mut clone = self.clone();
                clone.roll_down();
                let solution = clone.solve();
                if solution.is_some() {
                    return solution;
                }
            }

            if can_roll_left {
                let mut clone = self.clone();
                clone.roll_left();
                let solution = clone.solve();
                if solution.is_some() {
                    return solution;
                }
            }

            if can_roll_up {
                let mut clone = self.clone();
                clone.roll_up();
                let solution = clone.solve();
                if solution.is_some() {
                    return solution;
                }
            }

            return None;
        }
    }

    fn is_end_reachable(&self) -> bool {
        let p = &self.current_pos;

        if p.row == 0 && p.col == SIZE - 2 {
            return true;
        }

        if p.row == 1 && p.col == SIZE - 1 {
            return true;
        }

        let mut reachable = are_cells_reachable(
            &self.board,
            0,
            SIZE - 1,
            self.current_pos.row,
            self.current_pos.col,
        );
        if !reachable {
            return false;
        }

        for row in 0..SIZE {
            for col in 0..SIZE {
                if row == 0 && col == SIZE - 1 {
                    continue;
                }

                let visited = self.board[row][col];
                if !visited {
                    reachable = are_cells_reachable(&self.board, 0, SIZE - 1, row, col);
                    if !reachable {
                        return false;
                    }
                }
            }
        }

        true
    }

    fn roll_up(&mut self) {
        self.current_pos.row -= 1;
        self.dice = self.dice.roll_up();
        self.rolls.push(Direction::Up);
        self.board[self.current_pos.row][self.current_pos.col] = true;
        self.not_visited -= 1;
    }

    fn roll_down(&mut self) {
        self.current_pos.row += 1;
        self.dice = self.dice.roll_down();
        self.rolls.push(Direction::Down);
        self.board[self.current_pos.row][self.current_pos.col] = true;
        self.not_visited -= 1;
    }

    fn roll_left(&mut self) {
        self.current_pos.col -= 1;
        self.dice = self.dice.roll_left();
        self.rolls.push(Direction::Left);
        self.board[self.current_pos.row][self.current_pos.col] = true;
        self.not_visited -= 1;
    }

    fn roll_right(&mut self) {
        self.current_pos.col += 1;
        self.dice = self.dice.roll_right();
        self.rolls.push(Direction::Right);
        self.board[self.current_pos.row][self.current_pos.col] = true;
        self.not_visited -= 1;
    }

    fn can_roll_up(&self) -> bool {
        let p = &self.current_pos;

        // do not go beyond the upper limit
        if p.row == 0 {
            return false;
        }

        let next = Point {
            row: p.row - 1,
            col: p.col,
        };

        // we can't visit an already visited cell
        if self.board[next.row][next.col] {
            return false;
        }

        let next_number = self.dice.roll(Direction::Up).number();

        // if this is the last cell
        if next.row == 0 && next.col == SIZE - 1 {
            // if there are still not visited cell
            if self.not_visited != 1 {
                return false;
            }

            if next_number != 1 {
                return false;
            }
        }

        next_number != 1
    }

    fn can_roll_down(&self) -> bool {
        let p = &self.current_pos;

        // do not go beyond the bottom limit
        if p.row == SIZE - 1 {
            return false;
        }

        let next = Point {
            row: p.row + 1,
            col: p.col,
        };

        // we can't visit an already visited cell
        if self.board[next.row][next.col] {
            return false;
        }

        let next_number = self.dice.roll(Direction::Down).number();

        // since we move down, next cell cannot be the last cell
        next_number != 1
    }

    fn can_roll_right(&self) -> bool {
        let p = &self.current_pos;

        // do not go beyond the upper limit
        if p.col == SIZE - 1 {
            return false;
        }

        let next = Point {
            row: p.row,
            col: p.col + 1,
        };

        // we can't visit an already visited cell
        if self.board[next.row][next.col] {
            return false;
        }

        let next_number = self.dice.roll(Direction::Right).number();

        // if this is the last cell
        if next.row == 0 && next.col == SIZE - 1 {
            // if there are still not visited cell
            if self.not_visited != 1 {
                return false;
            }

            if next_number == 1 {
                return true;
            }
        }

        next_number != 1
    }

    fn can_roll_left(&self) -> bool {
        let p = &self.current_pos;

        // do not go beyond the bottom limit
        if p.col == 0 {
            return false;
        }

        let next = Point {
            row: p.row,
            col: p.col - 1,
        };

        // we can't visit an already visited cell
        if self.board[next.row][next.col] {
            return false;
        }

        let next_number = self.dice.roll(Direction::Left).number();
        // since we move left, next cell cannot be the last cell
        next_number != 1
    }
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
