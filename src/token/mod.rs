
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
    pub fn to_stone(self) -> Color {
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

    pub fn to_direction(self) -> Color {
        match d {
            direction::Direction::Up => Token::Up,
            direction::Direction::Down => Token::Down,
            direction::Direction::Left => Token::Left,
            direction::Direction::Right => Token::Right,
            direction::Direction::None => panic!("from_direction recieved Direction::None"),
        }
    }

    pub fn to_number(self) -> Number {}

    pub fn is_color(self) -> bool {
        match self {
            Token::Red => true,
            Token::Orange => true,
            Token::Yellow => true,
            Token::Green => true,
            Token::Blue => true,
            Token::Purple => true,
            _ => false
        }
    }
    pub fn is_direction(self) -> bool {
        match self {
            Token::Up => true,
            Token::Down => true,
            Token::Left => true,
            Token::Right => true,
            _ => false
        }
    }
    pub fn is_number(self) -> bool {
        match self {
            Token::One => true,
            Token::Two => true,
            Token::Three => true,
            _ => false
        }
    }
}

