
// direction enum, used for parsing directions
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
    None
}

impl Direction {
    pub fn from_str(dir: &str) -> Direction {
        match dir {
            "up" => Direction::Up,
            "down" => Direction::Down,
            "left" => Direction::Left,
            "right" => Direction::Right,
            _ => Direction::None
        }
    }
}

