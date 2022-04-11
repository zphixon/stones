use crate::{
    orange, red,
    vm::{OpColor, Opcode},
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
        todo!();
        //use Dir::*;
        //use Stone::*;

        //match (self.color, self.dir, self.number) {
        //    // constants
        //    (Red(R::One), dir) => self.stack.push(Value::Num(match dir {
        //        Up => 0,
        //        Down => 1,
        //        Left => 2,
        //        Right => 3,
        //    })),
        //    (Red(R::Two), dir) => self.stack.push(Value::Num(match dir {
        //        Up => 4,
        //        Down => 5,
        //        Left => 6,
        //        Right => 7,
        //    })),
        //    (Red(R::Three), dir) => self.stack.push(match dir {
        //        Up => Value::Num(8),
        //        Down => Value::Num(9),
        //        Left => Value::Bool(true),
        //        Right => Value::Bool(false),
        //    }),

        //    // arrays
        //    (Orange(O::One), Up) => self.array_in_progress = Some(Vec::new()),
        //    (Orange(O::One), Down) => {
        //        let arr = self
        //            .array_in_progress
        //            .take()
        //            .ok_or_else(|| my_todo!())
        //            .unwrap();
        //        self.stack.push(Value::Arr(arr));
        //    }
        //    (Orange(O::One), Left) => {
        //        // I didn't remember how useless arrays were in this language lol
        //        let in_progress = self.array_in_progress.take();
        //        if let Some(mut in_progress) = in_progress {
        //            in_progress.push(self.pop()?);
        //        } else {
        //            my_todo!();
        //        }
        //    }
        //    (Orange(O::One), Right) => {
        //        let idx: i64 = self.pop()?.try_into()?;
        //        let maybe_arr = self.peek(0)?;
        //        let arr: &[Value] = maybe_arr.get_slice().ok_or(Error::TypeMismatch {
        //            wanted: "array",
        //            got: maybe_arr.type_name(),
        //        })?;
        //        let dup = arr[idx as usize].clone();
        //        self.stack.push(dup);
        //    }

        //    // comparisons (feat. idiotic lhs/rhs semantics)
        //    (Orange(O::Two), Right) => Err(Error::Quine)?,
        //    (Orange(O::Two), dir) => {
        //        let lhs = self.pop()?;
        //        let rhs = self.pop()?;
        //        self.stack.push(Value::Bool(match dir {
        //            Up => lhs == rhs,
        //            Down => !(lhs < rhs) && lhs != rhs,
        //            Left => !(lhs > rhs) && lhs != rhs,
        //            _ => unreachable!(),
        //        }))
        //    }

        //    // math (feat. idiotic lhs/rhs semantics)
        //    (Yellow, dir) => {
        //        let lhs: i64 = self.pop()?.try_into()?;
        //        let rhs: i64 = self.pop()?.try_into()?;
        //        self.stack.push(Value::Num(match dir {
        //            Up => lhs * rhs,
        //            Down => lhs + rhs,
        //            Left => lhs - rhs,
        //            Right => lhs / rhs,
        //        }));
        //    }

        //    // stack operations
        //    (Green, Up) => {
        //        let d: i64 = self.pop()?.try_into()?;
        //        if d > 0 {
        //            let mut to_roll = Vec::new();
        //            for _ in 0..d + 1 {
        //                to_roll.push(self.pop()?);
        //            }
        //            to_roll.reverse();
        //            let top = to_roll.pop().unwrap();
        //            to_roll.insert(0, top);
        //            for elem in to_roll {
        //                self.stack.push(elem);
        //            }
        //        }
        //    }
        //    (Green, Down) => {
        //        let dup = self.pop()?;
        //        self.stack.push(dup.clone());
        //        self.stack.push(dup);
        //    }
        //    (Green, Left) => {
        //        let _ = self.pop()?;
        //    }
        //    (Green, Right) => {
        //        let bool = self.pop()?.is_truthy();
        //        self.stack.push(Value::Bool(!bool));
        //    }

        //    // i/o
        //    (Blue, Up) => self.pop()?.print_as_num(),
        //    (Blue, Down) => {
        //        let mut line = String::new();
        //        std::io::stdin().read_line(&mut line)?;
        //        let line = line.trim();
        //        if let Ok(num) = line.parse() {
        //            self.stack.push(Value::Num(num));
        //        } else if let Ok(bool) = line.parse() {
        //            self.stack.push(Value::Bool(bool));
        //        } else {
        //            my_todo!();
        //        }
        //    }
        //    (Blue, Left) => self.pop()?.print_as_char(),
        //    (Blue, Right) => {
        //        let a = self.pop()?;
        //        let b = self.pop()?;
        //        self.stack.push(a);
        //        self.stack.push(b);
        //    }

        //    // control flow
        //    (Purple, Up) => my_todo!(),
        //    (Purple, Down) => my_todo!(),
        //    (Purple, Left) => my_todo!(),
        //    (Purple, Right) => my_todo!(),
        //}
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
