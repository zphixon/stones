use color;
use direction;
use number;

#[derive(Debug, Clone, Copy)]
pub struct Statement {
    pub color: color::Color,
    pub direction: direction::Direction,
    pub number: number::Number,
}

impl Statement {
    pub fn new(color: color::Color, direction: direction::Direction, number: number::Number) -> Statement {
        Statement {
            color: color,
            direction: direction,
            number: number
        }
    }
}

