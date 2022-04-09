pub mod field;
pub mod vm;

use std::{cmp::Ordering, iter::Peekable};

#[derive(Debug, PartialEq)]
pub enum Error {
    UnknownToken { token: String },
    UnexpectedToken { token: Token },
    ExpectedColor { got: Token },
    ExpectedRedNumber { got: Token },
    ExpectedOrangeNumber { got: Token },
    ExpectedDir { got: Token },
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
    pub fn is_color_without_number(&self) -> bool {
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

pub fn scan(source: &str) -> impl Iterator<Item = Token> + '_ {
    source
        .split_whitespace()
        .filter_map(|sub| Token::try_from(sub).ok())
}

pub fn parse(source: &str) -> Result<Vec<vm::Op>, Error> {
    let mut ops = Vec::new();
    let mut tokens = scan(source).peekable();

    while let Some(op) = parse_command(&mut tokens)? {
        ops.push(op);
    }

    Ok(ops)
}

fn parse_command<I: Iterator<Item = Token>>(
    scanner: &mut Peekable<I>,
) -> Result<Option<vm::Op>, Error> {
    if let Some(&token) = scanner.peek() {
        let color_token = scanner.next().unwrap();
        let dir = parse_dir(scanner)?;

        let color = if color_token == Token::Red {
            let num = parse_red_number(scanner)?;
            vm::OpColor::Red(num)
        } else if color_token == Token::Orange {
            let num = parse_orange_number(scanner)?;
            vm::OpColor::Orange(num)
        } else if color_token.is_color_without_number() {
            color_token.try_into()?
        } else {
            return Err(Error::UnexpectedToken { token });
        };

        Ok(Some(vm::Op { color, dir }))
    } else {
        Ok(None)
    }
}

fn parse_red_number<I: Iterator<Item = Token>>(
    scanner: &mut Peekable<I>,
) -> Result<vm::RedNumber, Error> {
    if let Some(token) = scanner.next() {
        token.try_into()
    } else {
        Err(Error::ExpectedRedNumber { got: Token::Eof })
    }
}

fn parse_orange_number<I: Iterator<Item = Token>>(
    scanner: &mut Peekable<I>,
) -> Result<vm::OrangeNumber, Error> {
    if let Some(token) = scanner.next() {
        token.try_into()
    } else {
        Err(Error::ExpectedOrangeNumber { got: Token::Eof })
    }
}

fn parse_dir<I: Iterator<Item = Token>>(scanner: &mut Peekable<I>) -> Result<vm::Dir, Error> {
    if let Some(token) = scanner.next() {
        Ok(token.try_into()?)
    } else {
        Err(Error::ExpectedColor { got: Token::Eof })
    }
}

#[derive(Clone, Debug)]
pub enum Value {
    Num(i64),
    Arr(Vec<Value>),
    Bool(bool),
}

impl PartialEq for Value {
    fn eq(&self, rhs: &Value) -> bool {
        if !Value::same_type(self, rhs) {
            false
        } else {
            if self.is_num() {
                self.get_num() == rhs.get_num()
            } else if self.is_arr() {
                self.get_arr() == rhs.get_arr()
            } else {
                self.get_bool() == rhs.get_bool()
            }
        }
    }
}

impl PartialOrd for Value {
    fn partial_cmp(&self, rhs: &Value) -> Option<Ordering> {
        if !Value::same_type(self, rhs) {
            None
        } else {
            if self.is_bool() {
                None
            } else if self.is_arr() {
                self.get_arr().partial_cmp(&rhs.get_arr())
            } else {
                Some(self.get_num().cmp(&rhs.get_num()))
            }
        }
    }
}

impl Value {
    pub fn print_as_char(&self) {
        if self.is_num() {
            print!("{}", self.get_num() as u8 as char);
        } else if self.is_arr() {
            for c in self.get_arr() {
                c.print_as_char();
            }
        } else {
            print!("{}", self.get_bool());
        }
    }

    pub fn print_as_num(&self) {
        if self.is_num() {
            print!("{}", self.get_num());
        } else if self.is_arr() {
            print!("{:?}", self.get_arr());
        } else {
            print!("{}", self.get_bool());
        }
    }

    pub fn is_num(&self) -> bool {
        match self {
            &Value::Num(_) => true,
            _ => false,
        }
    }

    pub fn is_arr(&self) -> bool {
        match self {
            &Value::Arr(_) => true,
            _ => false,
        }
    }

    pub fn is_bool(&self) -> bool {
        match self {
            &Value::Bool(_) => true,
            _ => false,
        }
    }

    pub fn get_num(&self) -> i64 {
        let res = match self {
            &Value::Num(n) => n,
            _ => panic!("called get_num on non-num"),
        };
        res
    }

    pub fn get_arr(&self) -> Vec<Value> {
        // derp derpity derp
        let res = match self {
            &Value::Arr(ref a) => a,
            _ => panic!("called get_arr on non-arr"),
        };
        res.to_vec()
    }

    pub fn get_bool(&self) -> bool {
        let res = match self {
            &Value::Bool(b) => b,
            _ => panic!("called get_bool on non-bool"),
        };
        res
    }

    pub fn same_type(a: &Value, b: &Value) -> bool {
        (a.is_num() && b.is_num()) || (a.is_arr() && b.is_arr()) || (a.is_bool() && b.is_bool())
    }

    pub fn is_truthy(&self) -> bool {
        if self.is_bool() {
            self.get_bool()
        } else {
            true
        }
    }
}
