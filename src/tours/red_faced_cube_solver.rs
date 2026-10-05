use super::dice::Dice;
use super::{Direction, Point, SIZE, Solution, are_cells_reachable};

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
            no_rolls += 1;
        }
        if can_roll_down {
            no_rolls += 1;
        }
        if can_roll_left {
            no_rolls += 1;
        }
        if can_roll_right {
            no_rolls += 1;
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

        let reachable = are_cells_reachable(&self.board, 0, SIZE - 1, p.row, p.col);
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
                    let reachable = are_cells_reachable(&self.board, 0, SIZE - 1, row, col);
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
