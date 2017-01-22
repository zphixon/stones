/* Stones esoteric programming language
 * (c) 2016 Zack Hixon - see LICENSE.txt */

static VERSION: &'static str = "0.3.0";

extern crate argparse;
extern crate stones;

use stones::*;

use argparse::{ArgumentParser, StoreTrue, Store, Print};

use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::error::Error;

fn main() {
    // arguments
    let mut debug = false;
    let mut show_field = false;
    let mut show_stack = false;
    let mut show_frames = false;
    let mut create_print = false;
    let mut filename: String = "".into();

    {
        let mut args = ArgumentParser::new();
        args.set_description("Run a stones language file");
        args.refer(&mut debug)
            .add_option(&["-d", "--debug"], StoreTrue, "Run debugging");
        args.refer(&mut show_field)
            .add_option(&["-f", "--field"], StoreTrue, "Show field");
        args.refer(&mut show_frames)
            .add_option(&["-r", "--frames"], StoreTrue, "Show frames");
        args.refer(&mut show_stack)
            .add_option(&["-s", "--stack"], StoreTrue, "Show stack");
        args.refer(&mut create_print)
            .add_option(&["-p", "--print"], StoreTrue, "Create print");
        args.refer(&mut filename)
            .add_argument("file", Store, "File to run/output to")
            .required();
        args.add_option(&["-V", "--version"],
                        Print("stones version ".to_string() + VERSION), "Show version");
        args.parse_args_or_exit();
    }

    if create_print {
        //do_print(filename);
        std::process::exit(0);
    }

    // open file
    let path = Path::new(&filename);
    let display = path.display();
    let mut file = match File::open(&path) {
        Ok(file) => file,
        Err(_) => {
                println!("File doesn't exist: {}", display);
                std::process::exit(1);
            }
    };

    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Ok(_) => {},
        Err(e) => panic!("Couldn't read {}: {}", display, e.description()),
    }

    // split file into tokens
    let tlist: Vec<&str> = s.split_whitespace().collect();

    let tokens = lex(tlist);
    let stmts = parse(tokens).unwrap_or(std::process::exit(1));

    eval_prog(stmts);
}

fn eval_prog(prog: Vec<Statement>) {
    let mut frames = vec![true];
    let mut current_frame = frames.len() - 1;

    let mut whiles = vec![false];
    let mut current_while = whiles.len() - 1;

    let mut stack: Vec<Value> = vec![];

    let mut nmove = 0;
    let mut arraying = false;
    let mut fake_array: Vec<i64> = vec![];

    let mut k = 0;

    while k < prog.len() {
        let stmt = prog[k];

        if stmt.color == Color::Red {
            if frames[current_frame] {
                if stmt.direction == Direction::Up {
                    if stmt.number == Number::One {
                    } else if stmt.number == Number::Two {
                    } else if stmt.number == Number::Three {
                    }
                } else if stmt.direction == Direction::Down {
                    if stmt.number == Number::One {
                    } else if stmt.number == Number::Two {
                    } else if stmt.number == Number::Three {
                    }
                } else if stmt.direction == Direction::Left {
                    if stmt.number == Number::One {
                    } else if stmt.number == Number::Two {
                    } else if stmt.number == Number::Three {
                    }
                } else if stmt.direction == Direction::Right {
                    if stmt.number == Number::One {
                    } else if stmt.number == Number::Two {
                    } else if stmt.number == Number::Three {
                    }
                }
            }
        }
        else if stmt.color == Color::Orange {
            if frames[current_frame] {
                if stmt.direction == Direction::Up {
                    if stmt.number == Number::One {
                    } else if stmt.number == Number::Two {
                    }
                } else if stmt.direction == Direction::Down {
                    if stmt.number == Number::One {
                    } else if stmt.number == Number::Two {
                    }
                } else if stmt.direction == Direction::Left {
                    if stmt.number == Number::One {
                    } else if stmt.number == Number::Two {
                    }
                } else if stmt.direction == Direction::Right {
                    if stmt.number == Number::One {
                    } else if stmt.number == Number::Two {
                    }
                }
            }
        }
        else if stmt.color == Color::Yellow {
            if frames[current_frame] {
                if stmt.direction == Direction::Up {
                } else if stmt.direction == Direction::Down {
                } else if stmt.direction == Direction::Left {
                } else if stmt.direction == Direction::Right {
                }
            }
        }
        else if stmt.color == Color::Green {
            if frames[current_frame] {
                if stmt.direction == Direction::Up {
                } else if stmt.direction == Direction::Down {
                } else if stmt.direction == Direction::Left {
                } else if stmt.direction == Direction::Right {
                }
            }
        }
        else if stmt.color == Color::Blue {
            if frames[current_frame] {
                if stmt.direction == Direction::Up {
                } else if stmt.direction == Direction::Down {
                } else if stmt.direction == Direction::Left {
                } else if stmt.direction == Direction::Right {
                }
            }
        }
        else if stmt.color == Color::Purple {
            if stmt.direction == Direction::Up {
            } else if stmt.direction == Direction::Down {
            } else if stmt.direction == Direction::Left {
            } else if stmt.direction == Direction::Right {
            }
        }
    }
}

fn move_field(stone: Color, dir: Direction, field: &mut Vec<Vec<Color>>) -> Vec<Vec<Color>> {
    vec![vec![Color::Blue]]
}

