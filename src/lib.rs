
mod color;
mod direction;
mod number;
mod token;
mod statement;
mod field;

pub use color::*;
pub use direction::*;
pub use number::*;
pub use token::*;
pub use statement::*;
pub use field::*;

use std::cmp::Ordering;

#[derive(Clone, Debug)]
pub enum Value {
    Num(i64),
    Arr(Vec<Value>),
    Bool(bool)
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
            _ => false
        }
    }

    pub fn is_arr(&self) -> bool {
        match self {
            &Value::Arr(_) => true,
            _ => false
        }
    }

    pub fn is_bool(&self) -> bool {
        match self {
            &Value::Bool(_) => true,
            _ => false
        }
    }

    pub fn get_num(&self) -> i64 {
        let res = match self {
            &Value::Num(n) => n,
            _ => panic!("called get_num on non-num")
        };
        res
    }

    pub fn get_arr(&self) -> Vec<Value> {
        // derp derpity derp
        let res = match self {
            &Value::Arr(ref a) => a,
            _ => panic!("called get_arr on non-arr")
        };
        res.to_vec()
    }

    pub fn get_bool(&self) -> bool {
        let res = match self {
            &Value::Bool(b) => b,
            _ => panic!("called get_bool on non-bool")
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

pub fn lex(stokens: Vec<&str>) -> Vec<Token> {
    let mut ttokens: Vec<Token> = vec![];
    for token in stokens {
        ttokens.push(match token {
            "red" => Token::Red,
            "orange" => Token::Orange,
            "yellow" => Token::Yellow,
            "green" => Token::Green,
            "blue" => Token::Blue,
            "purple" => Token::Purple,
            "up" => Token::Up,
            "down" => Token::Down,
            "left" => Token::Left,
            "right" => Token::Right,
            "one" => Token::One,
            "two" => Token::Two,
            "three" => Token::Three,
            _ => Token::Nop
        });
    }

    ttokens.iter().cloned().filter(|x| *x != Token::Nop).collect()
}

pub fn parse(tokens: Vec<Token>) -> Result<Vec<statement::Statement>, ()> {
    let mut statements: Vec<statement::Statement> = vec![];
    let mut i = 0;
    let mut k = 0;

    // loop through list of tokens
    while i < tokens.len() {
        // check if color
        if tokens[i].is_color() {
            // out of bounds check
            if i + 1 < tokens.len() {
                // check if direction
                if tokens[i + 1].is_direction() {
                    // out of bounds check
                    if i + 2 < tokens.len() {
                        // check if number
                        if tokens[i + 2].is_number() {
                            // check if color is orange/red, only they take magnitudes
                            if tokens[i] == Token::Red || tokens[i] == Token::Orange {
                                // add statement
                                statements.push(Statement::new(tokens[i].to_stone(), tokens[i + 1].to_direction(), tokens[i + 2].to_number()));
                                i += 3;
                                k += 1;
                            } else {
                                println!("{} Did not expect number for {:?}", k, tokens[i]);
                                return Err(());
                            }
                        } else {
                            // if not a number, make sure it didn't need one
                            if tokens[i] != Token::Red && tokens[i] != Token::Orange {
                                // add statement
                                statements.push(Statement::new(tokens[i].to_stone(), tokens[i + 1].to_direction(), Number::None));
                                i += 2;
                                k += 1;
                            } else {
                                println!("{} Expected number for {:?}", k, tokens[i]);
                                return Err(());
                            }
                        }
                    } else {
                        // last two tokens
                        if tokens[i] != Token::Red && tokens[i] != Token::Orange {
                            if tokens[i + 1].is_direction() {
                                statements.push(Statement::new(tokens[i].to_stone(), tokens[i + 1].to_direction(), Number::None));
                                i += 2;
                                k += 1;
                            } else {
                                println!("{} Expected direction for {:?}", k, tokens[i]);
                                return Err(());
                            }
                        } else {
                            println!("{} Expected number for {:?}", k, tokens[i]);
                            return Err(());
                        }
                    }
                } else {
                    println!("{} Expected direction for {:?}", k, tokens[i]);
                }
            } else {
                // last token
                i += 1;
            }
        } else {
            println!("{} Expected color at {:?}", k, tokens[i]);
            return Err(());
        }
    }

    Ok(statements)
}

