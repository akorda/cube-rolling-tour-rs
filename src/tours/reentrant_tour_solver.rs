use super::dice::Dice;
use super::{Direction, Point, SIZE, Solution};

#[derive(Clone)]
pub struct ReentrantTourSolver {
    board: [[bool; SIZE]; SIZE],
    not_visited: u8,
    dice: Dice,
    current_pos: Point,
    rolls: Vec<Direction>,
    start_end: Point,
}

impl ReentrantTourSolver {
    pub fn new(start_end: Point) -> Self {
        ReentrantTourSolver {
            board: [[false; SIZE]; SIZE],
            not_visited: (SIZE as u8) * (SIZE as u8),
            dice: Dice::new().roll_left().roll_left(), // the initial face should not be the red
            current_pos: start_end,
            rolls: Vec::new(),
            start_end,
        }
    }

    pub fn solve(&mut self) -> Option<Solution> {
        super::print_board(self.start_end, &self.rolls, true);

        // we just found a solution!
        if self.not_visited == 0
            && self.current_pos.row == self.start_end.row
            && self.current_pos.col == self.start_end.col
        {
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
        for row in 0..SIZE {
            for col in 0..SIZE {
                if row == self.start_end.row && col == self.start_end.col {
                    continue;
                }

                let visited = self.board[row][col];
                if !visited {
                    let reachable = are_cells_reachable(
                        &self.board,
                        self.start_end.row,
                        self.start_end.col,
                        row,
                        col,
                    );
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

        // if this is the last cell
        if next.row == self.start_end.row && next.col == self.start_end.col {
            // if there are still not visited cell
            if self.not_visited != 1 {
                return false;
            }
        }

        let next_number = self.dice.roll(Direction::Up).number();
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

        // if this is the last cell
        if next.row == self.start_end.row && next.col == self.start_end.col {
            // if there are still not visited cell
            if self.not_visited != 1 {
                return false;
            }
        }

        let next_number = self.dice.roll(Direction::Down).number();
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

        // if this is the last cell
        if next.row == self.start_end.row && next.col == self.start_end.col {
            // if there are still not visited cell
            if self.not_visited != 1 {
                return false;
            }
        }

        let next_number = self.dice.roll(Direction::Right).number();
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

        // if this is the last cell
        if next.row == self.start_end.row && next.col == self.start_end.col {
            // if there are still not visited cell
            if self.not_visited != 1 {
                return false;
            }
        }

        let next_number = self.dice.roll(Direction::Left).number();
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
