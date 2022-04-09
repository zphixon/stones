pub mod field;
pub mod vm;

use std::{cmp::Ordering, iter::Peekable};

use field::Stone;
use vm::{Dir, Op, OrangeNumber, RedNumber};

#[derive(Debug)]
pub enum Error {
    UnknownToken {
        token: String,
    },
    UnexpectedToken {
        token: Token,
    },
    UnexpectedEof,
    ExpectedColor {
        got: Token,
    },
    ExpectedNumber {
        got: Token,
    },
    ExpectedRedNumber {
        got: Token,
    },
    ExpectedOrangeNumber {
        got: Token,
    },
    ExpectedDir {
        got: Token,
    },
    StackUnderflow,
    TypeMismatch {
        wanted: &'static str,
        got: &'static str,
    },
    Quine,
    IoError {
        err: std::io::Error,
    },
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error::IoError { err }
    }
}

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
    Eof,
}

impl Token {
    pub fn is_color(&self) -> bool {
        match self {
            Token::Red
            | Token::Orange
            | Token::Yellow
            | Token::Green
            | Token::Blue
            | Token::Purple => true,
            _ => false,
        }
    }

    pub fn is_number(&self) -> bool {
        match self {
            Token::One | Token::Two | Token::Three => true,
            _ => false,
        }
    }
}

impl TryFrom<&str> for Token {
    type Error = Error;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "red" => Ok(Token::Red),
            "orange" => Ok(Token::Orange),
            "yellow" => Ok(Token::Yellow),
            "green" => Ok(Token::Green),
            "blue" => Ok(Token::Blue),
            "purple" => Ok(Token::Purple),
            "up" => Ok(Token::Up),
            "down" => Ok(Token::Down),
            "left" => Ok(Token::Left),
            "right" => Ok(Token::Right),
            "one" => Ok(Token::One),
            "two" => Ok(Token::Two),
            "three" => Ok(Token::Three),
            _ => Err(Error::UnknownToken {
                token: value.to_string(),
            }),
        }
    }
}

#[derive(Debug)]
pub enum Ast {
    While {
        color: Token,
        dir: Dir,
        stmts: Vec<Ast>,
        end: Token,
    },
    If {
        color: Token,
        dir: Token,
        true_body: Vec<Ast>,
        else_: Option<Token>,
        false_body: Vec<Ast>,
        end: Token,
    },
    Red {
        color: Token,
        number: RedNumber,
        dir: Dir,
    },
    Orange {
        color: Token,
        number: OrangeNumber,
        dir: Dir,
    },
    Normal {
        color: Token,
        dir: Dir,
    },
}

pub enum EitherNumber {
    Red(RedNumber),
    Orange(OrangeNumber),
}

pub struct Number {
    number: EitherNumber,
    number_token: Token,
}

pub struct Command {
    color: Stone,
    color_token: Token,
    dir: Dir,
    dir_token: Token,
    number: Option<Number>,
}

pub fn scan(source: &str) -> impl Iterator<Item = Token> + '_ {
    source
        .split_whitespace()
        .filter_map(|sub| Token::try_from(sub).ok())
}

pub fn parse(source: &str) -> Result<Vec<Op>, Error> {
    let mut ops = Vec::new();
    let mut tokens = scan(source).peekable();

    while !matches!(tokens.peek(), Some(Token::Eof)) {
        let command = parse_command(&mut tokens)?;
        if command.color == Stone::Purple {}
    }

    Ok(ops)
}

fn parse_command<I: Iterator<Item = Token>>(scanner: &mut Peekable<I>) -> Result<Command, Error> {
    let color_token = next(scanner)?;
    let color = color_token.try_into()?;
    let dir_token = next(scanner)?;
    let dir = dir_token.try_into()?;
    let number = parse_number(scanner, color)?;

    Ok(Command {
        color,
        color_token,
        dir,
        dir_token,
        number,
    })
}

fn next<I: Iterator<Item = Token>>(scanner: &mut Peekable<I>) -> Result<Token, Error> {
    scanner.next().ok_or(Error::UnexpectedEof)
}

fn parse_number<I: Iterator<Item = Token>>(
    scanner: &mut Peekable<I>,
    color: Stone,
) -> Result<Option<Number>, Error> {
    if !color.has_number() {
        return Ok(None);
    }

    let number_token = next(scanner)?;
    if !number_token.is_number() {
        return Err(Error::ExpectedNumber { got: number_token });
    }

    let number = match color {
        Stone::Red => EitherNumber::Red(number_token.try_into()?),
        Stone::Orange => EitherNumber::Orange(number_token.try_into()?),
        _ => unreachable!(),
    };

    Ok(Some(Number {
        number,
        number_token,
    }))
}

#[derive(Clone, Debug)]
pub enum Value {
    Num(i64),
    Arr(Vec<Value>),
    Bool(bool),
}

impl PartialEq for Value {
    fn eq(&self, rhs: &Value) -> bool {
        if self.is_num() && rhs.is_num() {
            self.get_num() == rhs.get_num()
        } else if self.is_arr() && rhs.is_arr() {
            self.get_slice() == rhs.get_slice()
        } else if self.is_bool() && rhs.is_bool() {
            self.get_bool() == rhs.get_bool()
        } else {
            false
        }
    }
}

impl PartialOrd for Value {
    fn partial_cmp(&self, rhs: &Value) -> Option<Ordering> {
        if self.is_bool() || rhs.is_bool() {
            self.get_bool().partial_cmp(&rhs.get_bool())
        } else if self.is_arr() && rhs.is_arr() {
            self.get_slice().partial_cmp(&rhs.get_slice())
        } else if self.is_num() && rhs.is_num() {
            Some(self.get_num().cmp(&rhs.get_num()))
        } else {
            None
        }
    }
}

impl TryInto<i64> for Value {
    type Error = Error;
    fn try_into(self) -> Result<i64, Self::Error> {
        if self.is_num() {
            Ok(self.as_num())
        } else {
            Err(Error::TypeMismatch {
                wanted: "number",
                got: self.type_name(),
            })
        }
    }
}

impl TryInto<Vec<Value>> for Value {
    type Error = Error;
    fn try_into(self) -> Result<Vec<Value>, Self::Error> {
        if self.is_arr() {
            Ok(self.as_arr())
        } else {
            Err(Error::TypeMismatch {
                wanted: "array",
                got: self.type_name(),
            })
        }
    }
}

impl TryInto<bool> for Value {
    type Error = Error;
    fn try_into(self) -> Result<bool, Self::Error> {
        if self.is_bool() {
            Ok(self.as_bool())
        } else {
            Err(Error::TypeMismatch {
                wanted: "bool",
                got: self.type_name(),
            })
        }
    }
}

impl Value {
    pub fn type_name(&self) -> &'static str {
        match self {
            Value::Num(_) => "number",
            Value::Arr(_) => "array",
            Value::Bool(_) => "bool",
        }
    }

    pub fn print_as_char(&self) {
        if self.is_num() {
            print!("{}", self.as_num() as u8 as char);
        } else if self.is_arr() {
            for c in self.as_slice() {
                c.print_as_char();
            }
        } else if self.is_bool() {
            print!("{}", self.as_bool());
        } else {
            print!("null");
        }
    }

    pub fn print_as_num(&self) {
        if self.is_num() {
            print!("{}", self.as_num());
        } else if self.is_arr() {
            print!("{:?}", self.as_slice());
        } else if self.is_bool() {
            print!("{}", self.as_bool());
        } else {
            print!("null");
        }
    }

    pub fn is_num(&self) -> bool {
        matches!(self, Value::Num(_))
    }

    pub fn is_bool(&self) -> bool {
        matches!(self, Value::Bool(_))
    }

    pub fn is_arr(&self) -> bool {
        matches!(self, Value::Arr(_))
    }

    pub fn as_num(&self) -> i64 {
        assert!(self.is_num());
        self.get_num().unwrap()
    }

    pub fn as_bool(&self) -> bool {
        assert!(self.is_bool());
        self.get_bool().unwrap()
    }

    pub fn as_arr(&self) -> Vec<Value> {
        assert!(self.is_arr());
        self.get_arr().unwrap()
    }

    pub fn as_slice(&self) -> &[Value] {
        assert!(self.is_arr());
        self.get_slice().unwrap()
    }

    pub fn get_num(&self) -> Option<i64> {
        match self {
            Value::Num(n) => Some(*n),
            _ => None,
        }
    }

    pub fn get_bool(&self) -> Option<bool> {
        match self {
            Value::Bool(b) => Some(*b),
            _ => None,
        }
    }

    pub fn get_arr(&self) -> Option<Vec<Value>> {
        match self {
            Value::Arr(a) => Some(a.to_vec()),
            _ => None,
        }
    }

    pub fn get_slice(&self) -> Option<&[Value]> {
        match self {
            Value::Arr(a) => Some(a),
            _ => None,
        }
    }

    pub fn is_truthy(&self) -> bool {
        if self.is_bool() {
            self.as_bool()
        } else {
            true
        }
    }
}
