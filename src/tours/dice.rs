use crate::tours::Direction;

#[derive(Copy, Clone)]
pub struct Dice {
    top: u8,
    bottom: u8,
    north: u8,
    south: u8,
    east: u8,
    west: u8,
}

impl Dice {
    pub fn new() -> Dice {
        Dice {
            top: 1,
            bottom: 6,
            north: 2,
            south: 5,
            east: 3,
            west: 4,
        }
    }

    pub fn number(&self) -> u8 {
        self.top
    }

    pub fn roll_up(&self) -> Dice {
        Dice {
            top: self.south,
            bottom: self.north,
            north: self.top,
            south: self.bottom,
            east: self.east,
            west: self.west,
        }
    }

    pub fn roll_down(&self) -> Dice {
        Dice {
            top: self.north,
            bottom: self.south,
            north: self.bottom,
            south: self.top,
            east: self.east,
            west: self.west,
        }
    }

    pub fn roll_right(&self) -> Dice {
        Dice {
            top: self.west,
            bottom: self.east,
            north: self.north,
            south: self.south,
            east: self.top,
            west: self.bottom,
        }
    }

    pub fn roll_left(&self) -> Dice {
        Dice {
            top: self.east,
            bottom: self.west,
            north: self.north,
            south: self.south,
            east: self.bottom,
            west: self.top,
        }
    }

    pub fn roll(&self, dir: Direction) -> Dice {
        match dir {
            Direction::Up => self.roll_up(),
            Direction::Down => self.roll_down(),
            Direction::Right => self.roll_right(),
            Direction::Left => self.roll_left(),
        }
    }
}
