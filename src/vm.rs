use crate::{field::Stone, Error, Token};

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RedNumber {
    One,
    Two,
    Three,
}

impl RedNumber {
    fn magnitude(&self) -> usize {
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
    fn magnitude(&self) -> usize {
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

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OpColor {
    Red(RedNumber),
    Orange(OrangeNumber),
    Yellow,
    Green,
    Blue,
    Purple,
}

impl OpColor {
    pub fn magnitude(&self) -> usize {
        match self {
            OpColor::Red(step) => step.magnitude(),
            OpColor::Orange(step) => step.magnitude(),
            OpColor::Yellow | OpColor::Blue | OpColor::Green | OpColor::Purple => 1,
        }
    }
}

impl TryFrom<Token> for OpColor {
    type Error = Error;
    fn try_from(value: Token) -> Result<Self, Self::Error> {
        match value {
            Token::Yellow => Ok(OpColor::Yellow),
            Token::Green => Ok(OpColor::Green),
            Token::Blue => Ok(OpColor::Blue),
            Token::Purple => Ok(OpColor::Purple),
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

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Op {
    pub color: OpColor,
    pub dir: Dir,
}

impl std::fmt::Display for Op {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?} {:?}", self.color, self.dir)
    }
}

impl Op {
    pub fn color(&self) -> Stone {
        match self.color {
            OpColor::Red(_) => Stone::Red,
            OpColor::Orange(_) => Stone::Orange,
            OpColor::Yellow => Stone::Yellow,
            OpColor::Blue => Stone::Blue,
            OpColor::Green => Stone::Green,
            OpColor::Purple => Stone::Purple,
        }
    }

    pub fn magnitude(&self) -> usize {
        self.color.magnitude()
    }

    pub fn change_magnitude(self, new_magnitude: usize) -> Op {
        match self.color {
            OpColor::Red(_) => Op {
                color: OpColor::Red(new_magnitude.into()),
                ..self
            },
            OpColor::Orange(_) => Op {
                color: OpColor::Orange(new_magnitude.into()),
                ..self
            },
            OpColor::Yellow | OpColor::Blue | OpColor::Green | OpColor::Purple => self,
        }
    }
}

#[derive(Default, Debug)]
pub struct Vm {
    #[cfg(test)]
    pub history: Vec<Op>,
    stack: Vec<crate::Value>,
}

impl Vm {
    pub fn exec(&mut self, op: Op) {
        use Dir::*;
        use OpColor::*;
        use OrangeNumber as O;
        use RedNumber as R;

        println!("exec {op:?}");

        #[cfg(test)]
        self.history.push(op);

        match (op.color, op.dir) {
            (Red(R::One), Left) => {}
            (Red(R::One), Right) => {}
            (Red(R::One), Up) => {}
            (Red(R::One), Down) => {}

            (Red(R::Two), Left) => {}
            (Red(R::Two), Right) => {}
            (Red(R::Two), Up) => {}
            (Red(R::Two), Down) => {}

            (Red(R::Three), Left) => {}
            (Red(R::Three), Right) => {}
            (Red(R::Three), Up) => {}
            (Red(R::Three), Down) => {}

            (Orange(O::One), Left) => {}
            (Orange(O::One), Right) => {}
            (Orange(O::One), Up) => {}
            (Orange(O::One), Down) => {}

            (Orange(O::Two), Left) => {}
            (Orange(O::Two), Right) => {}
            (Orange(O::Two), Up) => {}
            (Orange(O::Two), Down) => {}

            (Yellow, Left) => {}
            (Yellow, Right) => {}
            (Yellow, Up) => {}
            (Yellow, Down) => {}

            (Green, Left) => {}
            (Green, Right) => {}
            (Green, Up) => {}
            (Green, Down) => {}

            (Blue, Left) => {}
            (Blue, Right) => {}
            (Blue, Up) => {}
            (Blue, Down) => {}

            (Purple, Left) => {}
            (Purple, Right) => {}
            (Purple, Up) => {}
            (Purple, Down) => {}
        }
    }
}
