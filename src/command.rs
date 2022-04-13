use crate::{
    orange, red,
    vm::{Comparison, Math, Opcode},
    AstCommand, Error, Token,
};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Command {
    pub color: Stone,
    pub dir: Dir,
    pub number: Option<EitherNumber>,
    pub side_effect: bool,
}

impl Command {
    pub fn empty() -> Command {
        Command {
            color: Stone::__,
            dir: Dir::Left,
            number: None,
            side_effect: false,
        }
    }

    pub fn magnitude(&self) -> usize {
        self.number.map(|number| number.magnitude()).unwrap_or(1)
    }

    pub fn change_magnitude(self, new_magnitude: usize) -> Command {
        match self.color {
            Stone::Red => Command {
                color: Stone::Red,
                number: Some(EitherNumber::Red(new_magnitude.into())),
                ..self
            },
            Stone::Orange => Command {
                color: Stone::Orange,
                number: Some(EitherNumber::Orange(new_magnitude.into())),
                ..self
            },
            Stone::Yellow | Stone::Blue | Stone::Green | Stone::Purple => self,
            Stone::__ => unreachable!(),
        }
    }

    pub fn get_opcode(&self) -> Option<Opcode> {
        use Dir::*;
        use Stone::*;

        Some(match (self.color, self.dir, self.number) {
            // constants
            (Red, dir, red!(One)) => Opcode::PushNumber(match dir {
                Up => 0,
                Down => 1,
                Left => 2,
                Right => 3,
            }),
            (Red, dir, red!(Two)) => Opcode::PushNumber(match dir {
                Up => 4,
                Down => 5,
                Left => 6,
                Right => 7,
            }),
            (Red, dir, red!(Three)) => match dir {
                Up => Opcode::PushNumber(8),
                Down => Opcode::PushNumber(9),
                Left => Opcode::PushBool(true),
                Right => Opcode::PushBool(false),
            },

            // arrays
            (Orange, Up, orange!(One)) => Opcode::StartArray,
            (Orange, Down, orange!(One)) => Opcode::EndArray,
            (Orange, Left, orange!(One)) => Opcode::PushArray,
            (Orange, Right, orange!(One)) => Opcode::NthArray,

            // comparisons (feat. idiotic lhs/rhs semantics)
            (Orange, Right, orange!(Two)) => Opcode::Quine,
            (Orange, dir, orange!(Two)) => Opcode::Comparison(match dir {
                Up => Comparison::Equal,
                Down => Comparison::LessThan,
                Left => Comparison::GreaterThan,
                _ => unreachable!(),
            }),

            // math (feat. idiotic lhs/rhs semantics)
            (Yellow, dir, None) => Opcode::Math(match dir {
                Up => Math::Multiply,
                Down => Math::Add,
                Left => Math::Subtract,
                Right => Math::Divide,
            }),

            // stack operations
            (Green, Up, None) => Opcode::Roll,
            (Green, Down, None) => Opcode::Dup,
            (Green, Left, None) => Opcode::Drop,
            (Green, Right, None) => Opcode::Not,

            // i/o
            (Blue, Up, None) => Opcode::Print,
            (Blue, Down, None) => Opcode::Input,
            (Blue, Left, None) => Opcode::Printc,
            (Blue, Right, None) => Opcode::Swap,

            // purple instructions need to be manually patched in the compiler
            (Purple, _, None) => None?,

            (_, _, None) | (_, _, Some(_)) => {
                unreachable!("invalid command asked for opcode {self:?}")
            }
        })
    }
}

impl From<AstCommand> for Command {
    fn from(value: AstCommand) -> Self {
        Command {
            color: value.color,
            dir: value.dir,
            number: value.number.map(|number| number.number),
            side_effect: false,
        }
    }
}

impl std::fmt::Display for Command {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} {}{}{}",
            match self.color {
                Stone::__ => "__",
                Stone::Red => "red",
                Stone::Orange => "orange",
                Stone::Yellow => "yellow",
                Stone::Green => "green",
                Stone::Blue => "blue",
                Stone::Purple => "purple",
            },
            match self.dir {
                Dir::Up => "up",
                Dir::Down => "down",
                Dir::Left => "left",
                Dir::Right => "right",
            },
            match self.number {
                red!(One) => " one",
                red!(Two) => " two",
                red!(Three) => " three",
                orange!(One) => " one",
                orange!(Two) => " two",
                None => "",
            },
            if self.side_effect {
                " (side effect)"
            } else {
                ""
            },
        )?;
        Ok(())
    }
}

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub enum Stone {
    __,
    Red,
    Orange,
    Yellow,
    Green,
    Blue,
    Purple,
}

impl Stone {
    pub fn has_number(&self) -> bool {
        match self {
            Stone::Red | Stone::Orange => true,
            _ => false,
        }
    }

    pub fn number_one(&self) -> Option<EitherNumber> {
        match self {
            Stone::Red => red!(One),
            Stone::Orange => orange!(One),
            _ => None,
        }
    }
}

impl TryFrom<Token> for Stone {
    type Error = Error;
    fn try_from(value: Token) -> Result<Stone, Self::Error> {
        match value {
            Token::Red => Ok(Stone::Red),
            Token::Orange => Ok(Stone::Orange),
            Token::Yellow => Ok(Stone::Yellow),
            Token::Green => Ok(Stone::Green),
            Token::Blue => Ok(Stone::Blue),
            Token::Purple => Ok(Stone::Purple),
            _ => Err(Error::ExpectedColor { got: value }),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Dir {
    Left,
    Right,
    Up,
    Down,
}

impl TryFrom<Token> for Dir {
    type Error = Error;
    fn try_from(value: Token) -> Result<Self, Self::Error> {
        match value {
            Token::Left => Ok(Dir::Left),
            Token::Right => Ok(Dir::Right),
            Token::Up => Ok(Dir::Up),
            Token::Down => Ok(Dir::Down),
            _ => Err(Error::ExpectedDir { got: value }),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum EitherNumber {
    Red(RedNumber),
    Orange(OrangeNumber),
}

impl EitherNumber {
    pub fn magnitude(&self) -> usize {
        match self {
            EitherNumber::Red(number) => number.magnitude(),
            EitherNumber::Orange(number) => number.magnitude(),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RedNumber {
    One,
    Two,
    Three,
}

impl RedNumber {
    pub fn magnitude(&self) -> usize {
        match self {
            RedNumber::One => 1,
            RedNumber::Two => 2,
            RedNumber::Three => 3,
        }
    }
}

impl From<usize> for RedNumber {
    fn from(i: usize) -> Self {
        match i {
            1 => RedNumber::One,
            2 => RedNumber::Two,
            3 => RedNumber::Three,
            _ => unreachable!(),
        }
    }
}

impl TryFrom<Token> for RedNumber {
    type Error = Error;
    fn try_from(value: Token) -> Result<Self, Self::Error> {
        match value {
            Token::One => Ok(RedNumber::One),
            Token::Two => Ok(RedNumber::Two),
            Token::Three => Ok(RedNumber::Three),
            _ => Err(Error::ExpectedRedNumber { got: value }),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OrangeNumber {
    One,
    Two,
}

impl OrangeNumber {
    pub fn magnitude(&self) -> usize {
        match self {
            OrangeNumber::One => 1,
            OrangeNumber::Two => 2,
        }
    }
}

impl From<usize> for OrangeNumber {
    fn from(i: usize) -> Self {
        match i {
            1 => OrangeNumber::One,
            2 => OrangeNumber::Two,
            _ => unreachable!(),
        }
    }
}

impl TryFrom<Token> for OrangeNumber {
    type Error = Error;
    fn try_from(value: Token) -> Result<Self, Self::Error> {
        match value {
            Token::One => Ok(OrangeNumber::One),
            Token::Two => Ok(OrangeNumber::Two),
            _ => Err(Error::ExpectedOrangeNumber { got: value }),
        }
    }
}
