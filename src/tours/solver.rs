use super::dice::Dice;
use super::types::*;
use std::io::{self, IsTerminal, Write};

const SIZE: usize = 8;

pub struct Solver {
    board: [[bool; SIZE]; SIZE],
    not_visited: u8,
    dice: Dice,
    current_pos: Point,
    rolls: Vec<Direction>,
}

impl Solver {
    pub fn new() -> Solver {
        let mut board = [[false; SIZE]; SIZE];
        board[0][0] = true;

        Solver {
            board,
            not_visited: (SIZE as u8) * (SIZE as u8) - 1,
            dice: Dice::new(),
            current_pos: Point { row: 0, col: 0 },
            rolls: Vec::new(),
        }
    }

    pub fn solve(&mut self) -> bool {
        self.print();

        // we just found a solution!
        if self.not_visited == 0 && self.current_pos.row == 0 && self.current_pos.col == SIZE - 1 {
            return true;
        }

        if !self.is_end_reachable() {
            return false;
        }

        let can_roll_up = self.can_roll_up(&self.current_pos);
        let can_roll_down = self.can_roll_down(&self.current_pos);
        let can_roll_left = self.can_roll_left(&self.current_pos);
        let can_roll_right = self.can_roll_right(&self.current_pos);

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
            if self.not_visited == 0 {
                return true;
            }

            return false;
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
                if clone.solve() {
                    return true;
                }
            }

            if can_roll_down {
                let mut clone = self.clone();
                clone.roll_down();
                if clone.solve() {
                    return true;
                }
            }

            if can_roll_left {
                let mut clone = self.clone();
                clone.roll_left();
                if clone.solve() {
                    return true;
                }
            }

            if can_roll_up {
                let mut clone = self.clone();
                clone.roll_up();
                if clone.solve() {
                    return true;
                }
            }

            return false;
        }
    }

    fn is_end_reachable(&self) -> bool {
        let p = &self.current_pos;

        if p.row == 0 && p.col == 6 {
            return true;
        }

        if p.row == 1 && p.col == SIZE - 1 {
            return true;
        }

        let mut examined = [[false; SIZE]; SIZE];
        let mut reachable = self.are_cells_reachable(
            &mut examined,
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
                    examined = [[false; SIZE]; SIZE];
                    reachable = self.are_cells_reachable(&mut examined, 0, SIZE - 1, row, col);
                    if !reachable {
                        return false;
                    }
                }
            }
        }

        true
    }

    fn are_cells_reachable(
        &self,
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

        let visited = self.board[start_row][start_col];
        if visited {
            return false;
        }

        // top
        if start_row != 0
            && self.are_cells_reachable(examined, start_row - 1, start_col, end_row, end_col)
        {
            return true;
        }

        // bottom
        if start_row != SIZE - 1
            && self.are_cells_reachable(examined, start_row + 1, start_col, end_row, end_col)
        {
            return true;
        }

        // left
        if start_col != 0
            && self.are_cells_reachable(examined, start_row, start_col - 1, end_row, end_col)
        {
            return true;
        }

        // right
        if start_col != SIZE - 1
            && self.are_cells_reachable(examined, start_row, start_col + 1, end_row, end_col)
        {
            return true;
        }

        return false;
    }

    fn clone(&self) -> Solver {
        let clone = Solver {
            board: self.board.clone(),
            rolls: self.rolls.clone(),
            dice: self.dice,
            current_pos: self.current_pos,
            not_visited: self.not_visited,
        };
        clone
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

    fn can_roll_up(&self, p: &Point) -> bool {
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

    fn can_roll_down(&self, p: &Point) -> bool {
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

    fn can_roll_right(&self, p: &Point) -> bool {
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

    fn can_roll_left(&self, p: &Point) -> bool {
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

    pub fn print(&self) {
        const UP: u8 = 1;
        const RIGHT: u8 = 2;
        const DOWN: u8 = 4;
        const LEFT: u8 = 8;

        // For each cell, the sides through which the path enters or leaves it
        let mut links = [[0u8; SIZE]; SIZE];

        let mut p = Point { row: 0, col: 0 };
        for roll in &self.rolls {
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

        // Cells sit on even columns; odd columns hold the horizontal connectors
        let mut output = String::new();
        for row in 0..SIZE {
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

        // Redraw in place when writing to a terminal
        let mut stdout = io::stdout();
        if stdout.is_terminal() {
            // move cursor to the top-left corner
            print!("\x1B[H");
        }
        print!("{}", output);
        stdout.flush().unwrap();
    }
}
