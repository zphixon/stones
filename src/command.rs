use crate::{
    orange, red,
    vm::{Comparison, Math, OpColor, Opcode},
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
            Stone::X => unreachable!(),
        }
    }

    pub fn to_opcode(&self) -> Opcode {
        use Dir::*;
        use Opcode::*;
        use Stone::*;

        match (self.color, self.dir, self.number) {
            // constants
            (Red, dir, red!(One)) => PushNumber(match dir {
                Up => 0,
                Down => 1,
                Left => 2,
                Right => 3,
            }),
            (Red, dir, red!(Two)) => PushNumber(match dir {
                Up => 4,
                Down => 5,
                Left => 6,
                Right => 7,
            }),
            (Red, dir, red!(Three)) => match dir {
                Up => PushNumber(8),
                Down => PushNumber(9),
                Left => PushBool(true),
                Right => PushBool(false),
            },

            // arrays
            (Orange, Up, orange!(One)) => StartArray,
            (Orange, Down, orange!(One)) => EndArray,
            (Orange, Left, orange!(One)) => PushArray,
            (Orange, Right, orange!(One)) => NthArray,

            // comparisons (feat. idiotic lhs/rhs semantics)
            (Orange, Right, orange!(Two)) => Quine,
            (Orange, dir, orange!(Two)) => Comparison(match dir {
                Up => crate::vm::Comparison::Equal,
                Down => crate::vm::Comparison::LessThan,
                Left => crate::vm::Comparison::GreaterThan,
                _ => unreachable!(),
            }),

            // math (feat. idiotic lhs/rhs semantics)
            (Yellow, dir, None) => Math(match dir {
                Up => crate::vm::Math::Multiply,
                Down => crate::vm::Math::Add,
                Left => crate::vm::Math::Subtract,
                Right => crate::vm::Math::Divide,
            }),

            // stack operations
            (Green, Up, None) => Roll,
            (Green, Down, None) => Dup,
            (Green, Left, None) => Drop,
            (Green, Right, None) => Not,

            // i/o
            (Blue, Up, None) => Print,
            (Blue, Down, None) => Input,
            (Blue, Left, None) => Printc,
            (Blue, Right, None) => Swap,

            // control flow
            (Purple, dir, None) => match dir {
                Up => Roll,
                Down => Roll,
                Left => Roll,
                Right => Roll,
            },

            (_, _, Some(_)) => todo!(),
            (_, _, None) => unreachable!(),
        }
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

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub enum Stone {
    X,
    Red,
    Orange,
    Yellow,
    Green,
    Blue,
    Purple,
}

impl Stone {
    pub fn to_op(&self) -> OpColor {
        match self {
            Stone::Red => OpColor::Red(RedNumber::One),
            Stone::Orange => OpColor::Orange(OrangeNumber::One),
            Stone::Yellow => OpColor::Yellow,
            Stone::Blue => OpColor::Blue,
            Stone::Green => OpColor::Green,

            // cannot get purple, it is the heaviest
            // cannot get X, it has no op
            _ => unreachable!(),
        }
    }

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