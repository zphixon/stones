
mod color;
mod direction;
mod number;
mod token;
mod statement;

pub use color::*;
pub use direction::*;
pub use number::*;
pub use token::*;
pub use statement::*;

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
            "1" => Token::One,
            "2" => Token::Two,
            "3" => Token::Three,
            _ => Token::Nop
        });
    }

    ttokens.iter().cloned().filter(|x| *x == Token::Nop).collect()
}

pub fn parse(tokens: Vec<Token>) -> Result<Vec<statement::Statement>, ()> {
    let mut statements: Vec<statement::Statement> = vec![];
    let mut i = 0;

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
                                statements.push(Statement::new(tokens[i], tokens[i + 1], tokens[i + 2]));
                                i += 3;
                            } else {
                                println!("Did not expect number for {:?}", tokens[i]);
                                return Err(());
                            }
                        } else {
                            // if not a number, make sure it didn't need one
                            if tokens[i] != Token::Red && tokens[i] != Token::Orange {
                                statements.push(Statement::new(tokens[i]), tokens[i + 1], );
                            } else {
                                println!("Expected number for {:?}", tokens[i]);
                                return Err(());
                            }
                        }
                    }
                }
            }
        }
    }

    Ok(statements)
}

