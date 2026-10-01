#[derive(Clone)]
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
