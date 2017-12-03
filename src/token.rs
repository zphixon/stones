
use color::*;
use direction::*;
use number::*;

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
    pub fn from_stone(c: Color) -> Token {
        match c {
            Color::Red => Token::Red,
            Color::Orange => Token::Orange,
            Color::Yellow => Token::Yellow,
            Color::Green => Token::Green,
            Color::Blue => Token::Blue,
            Color::Purple => Token::Purple,
            _ => Token::Nop,
        }
    }

    pub fn from_direction(d: Direction) -> Token {
        match d {
            Direction::Up => Token::Up,
            Direction::Down => Token::Down,
            Direction::Left => Token::Left,
            Direction::Right => Token::Right,
            _ => Token::Nop,
        }
    }

    pub fn from_number(n: Number) -> Token {
        match n {
            Number::One => Token::One,
            Number::Two => Token::Two,
            Number::Three => Token::Three,
            _ => Token::Nop,
        }
    }

    pub fn to_stone(self) -> Color {
        match self {
            Token::Red => Color::Red,
            Token::Orange => Color::Orange,
            Token::Yellow => Color::Yellow,
            Token::Green => Color::Green,
            Token::Blue => Color::Blue,
            Token::Purple => Color::Purple,
            _ => Color::Invis,
        }
    }

    pub fn to_direction(self) -> Direction {
        match self {
            Token::Up => Direction::Up,
            Token::Down => Direction::Down,
            Token::Left => Direction::Left,
            Token::Right => Direction::Right,
            _ => Direction::None,
        }
    }

    pub fn to_number(self) -> Number {
        match self {
            Token::One => Number::One,
            Token::Two => Number::Two,
            Token::Three => Number::Three,
            _ => Number::None,
        }
    }

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

