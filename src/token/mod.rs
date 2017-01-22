
use color;
use direction;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Token {
    Red,
    Orange,
    Yellow,
    Green,
    Blue,
    Purple,
    Up,
    Down,
    Left,
    Right,
    One,
    Two,
    Three,
    Nop
}

impl Token {
    pub fn from_stone(s: color::Color) -> Token {
        match s {
            color::Color::Red => Token::Red,
            color::Color::Orange => Token::Orange,
            color::Color::Yellow => Token::Yellow,
            color::Color::Green => Token::Green,
            color::Color::Blue => Token::Blue,
            color::Color::Purple => Token::Purple,
            color::Color::Invis => Token::Nop,
        }
    }

    pub fn from_direction(d: direction::Direction) -> Token {
        match d {
            direction::Direction::Up => Token::Up,
            direction::Direction::Down => Token::Down,
            direction::Direction::Left => Token::Left,
            direction::Direction::Right => Token::Right,
            direction::Direction::None => panic!("from_direction recieved Direction::None"),
        }
    }
}

