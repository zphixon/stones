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
        do_print(filename);
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

    // parse tokens into tokens... yes
    let mut tokens_nop: Vec<Token> = vec![];
    for token in &tlist {
        tokens_nop.push(match *token {
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

    let mut tokens: Vec<Token> = tokens_nop.into_iter().filter(|t| *t != Token::Nop).collect::<Vec<Token>>();

    // field vector, represents stone field
    let mut field: Vec<Vec<Color>> = // so much for 80 columns
        vec![vec![Color::Blue,  Color::Invis, Color::Invis, Color::Invis, Color::Invis, Color::Invis, Color::Orange,Color::Invis, Color::Invis, Color::Invis, Color::Invis ],
             vec![Color::Invis, Color::Invis, Color::Invis, Color::Invis, Color::Invis, Color::Invis, Color::Invis, Color::Invis, Color::Invis, Color::Invis, Color::Invis ],
             vec![Color::Invis, Color::Invis, Color::Red,   Color::Invis, Color::Invis, Color::Invis, Color::Invis, Color::Invis, Color::Green, Color::Invis, Color::Invis ],
             vec![Color::Invis, Color::Invis, Color::Invis, Color::Invis, Color::Invis, Color::Invis, Color::Invis, Color::Invis, Color::Invis, Color::Invis, Color::Invis ],
             vec![Color::Invis, Color::Invis, Color::Invis, Color::Invis, Color::Yellow,Color::Invis, Color::Invis, Color::Invis, Color::Invis, Color::Invis, Color::Purple],
             vec![Color::Invis, Color::Invis, Color::Invis, Color::Invis, Color::Invis, Color::Invis, Color::Invis, Color::Invis, Color::Invis, Color::Invis, Color::Invis ]];

    // stack vector, pretty ezpz
    let mut stack: Vec<i64> = vec![];

    // store scopes/whether or not to execute
    let mut frame: Vec<bool> = vec![true];
    let mut current_frame = 0;

    // execute program
    let mut count = 0; // track token counter
    let mut current_stone = Color::Invis; // keep track of current stone
    let mut current_direction = Direction::None; // track direction
    let mut current_number = Number::None; // used for red movement

    // loop through tokens
    while count < tokens.len() {
        if show_field {
            for row in &field {
                for color in row {
                    match *color {
                        Color::Red => print!("{:?}... ", color),
                        Color::Orange => print!("{:?} ", color),
                        Color::Yellow => print!("{:?} ", color),
                        Color::Green => print!("{:?}. ", color),
                        Color::Blue => print!("{:?}.. ", color),
                        Color::Purple => print!("{:?} ", color),
                        Color::Invis => print!("...... "), // oh.
                    }
                }
                println!("");
            }
            println!("");
        }

        match tokens[count] {
            // colors
            Token::Red => {
                current_stone = Color::Red;
                current_direction = Direction::None;
                current_number = Number::None;
                if count != 0 && is_color(tokens[count - 1]) {
                    println!("Unexpected color!");
                }
                if count != tokens.len() - 2 &&
                   tokens[count + 2] != Token::One &&
                   tokens[count + 2] != Token::Two &&
                   tokens[count + 2] != Token::Three {
                    println!("Expected number!");
                }
            },
            Token::Orange => {
                current_stone = Color::Orange;
                current_direction = Direction::None;
                current_number = Number::None;
                if count != 0 && is_color(tokens[count - 1]) {
                    println!("Unexpected color!");
                }
                if count != tokens.len() - 2 &&
                   tokens[count + 2] != Token::One &&
                   tokens[count + 2] != Token::Two {
                        println!("Expected number!");
                }
            },
            Token::Yellow => {
                current_stone = Color::Yellow;
                current_direction = Direction::None;
                current_number = Number::None;
                if count != 0 && is_color(tokens[count - 1]) {
                    println!("Unexpected color!");
                }
            },
            Token::Green => {
                current_stone = Color::Green;
                current_direction = Direction::None;
                current_number = Number::None;
                if count != 0 && is_color(tokens[count - 1]) {
                    println!("Unexpected color!");
                }
            },
            Token::Blue => {
                current_stone = Color::Blue;
                current_direction = Direction::None;
                current_number = Number::None;
                if count != 0 && is_color(tokens[count - 1]) {
                    println!("Unexpected color!");
                }
            },
            Token::Purple => {
                current_stone = Color::Purple;
                current_direction = Direction::None;
                current_number = Number::None;
                if count != 0 && is_color(tokens[count - 1]) {
                    println!("Unexpected color!");
                }
            },

            // directions
            Token::Up => {
                current_direction = Direction::Up;
                // red stone is handled in number area
                if current_stone != Color::Red && current_stone != Color::Orange {
                    match current_stone {
                        Color::Yellow => { // multiply
                            // may use let = match later
                            if frame[current_frame] {
                                let tmp1 = stack.pop().expect("Stack is empty!");
                                let tmp2 = stack.pop().expect("Stack is empty!");
                                stack.push(tmp1 * tmp2);
                                // move stones
                                field = move_stone(current_stone, current_direction, field, &mut tokens, count as usize);
                            }
                        },
                        //Color::Green => { // roll },
                        Color::Blue => { // print as number
                            if frame[current_frame] {
                                print!("{}\n", stack.pop().expect("Stack is empty!"));
                                field = move_stone(current_stone, current_direction, field, &mut tokens, count as usize);
                            }
                        },
                        Color::Purple => { // if
                            if frame[current_frame] {
                                let tmp = stack.pop().expect("Stack is empty!");
                                if tmp == 1 {
                                    // condition is met, will execute
                                    current_frame += 1;
                                    frame.push(true);
                                } else {
                                    // condition not met, will not execute
                                    current_frame += 1;
                                    frame.push(false);
                                }
                                field = move_stone(current_stone, current_direction, field, &mut tokens, count as usize);
                            }
                        },
                        _ => {}
                    }
                    current_stone = Color::Invis;
                    current_direction = Direction::None;
                }
            },
            Token::Down => {
                current_direction = Direction::Down;
                if current_stone != Color::Red && current_stone != Color::Orange {
                    match current_stone {
                        Color::Yellow => { // add
                            if frame[current_frame] {
                                let tmp1 = stack.pop().expect("Stack is empty!");
                                let tmp2 = stack.pop().expect("Stack is empty!");
                                stack.push(tmp1 + tmp2);
                                field = move_stone(current_stone, current_direction, field, &mut tokens, count as usize);
                            }
                        },
                        Color::Green => { // dup
                            if frame[current_frame] {
                                let tmp = stack.pop().expect("Stack is empty!");
                                stack.push(tmp);
                                stack.push(tmp);
                            }
                        },
                        //Color::Blue => { // input },
                        Color::Purple => { // else
                            frame[current_frame] = !frame[current_frame];
                            field = move_stone(current_stone, current_direction, field, &mut tokens, count as usize);
                        },
                        _ => {}
                    }
                    current_stone = Color::Invis;
                    current_direction = Direction::None;
                }
            },
            Token::Left => {
                current_direction = Direction::Left;
                if current_stone != Color::Red && current_stone != Color::Orange {
                    match current_stone {
                        Color::Yellow => { // subtract
                            if frame[current_frame] {
                                let tmp1 = stack.pop().expect("Stack is empty!");
                                let tmp2 = stack.pop().expect("Stack is empty!");
                                stack.push(tmp1 - tmp2);
                                field = move_stone(current_stone, current_direction, field, &mut tokens, count as usize);
                            }
                        },
                        Color::Green => { // drop
                            stack.pop().expect("Stack is empty!");
                        },
                        Color::Blue => { // print as character
                            if frame[current_frame] {
                                print!("{}", stack.pop().expect("Stack is empty!") as u8 as char);
                                field = move_stone(current_stone, current_direction, field, &mut tokens, count as usize);
                            }
                        },
                        //Color::Purple => { // while },
                        _ => {}
                    }
                    current_stone = Color::Invis;
                    current_direction = Direction::None;
                }
            },
            Token::Right => {
                current_direction = Direction::Right;
                if current_stone != Color::Red && current_stone != Color::Orange {
                    match current_stone {
                        //Color::Orange => {},
                        Color::Yellow => { // divide
                            if frame[current_frame] {
                                let tmp1 = stack.pop().expect("Stack is empty!");
                                let tmp2 = stack.pop().expect("Stack is empty!");
                                stack.push(tmp1 / tmp2);
                                field = move_stone(current_stone, current_direction, field, &mut tokens, count as usize);
                            }
                        },
                        //Color::Green => {},
                        Color::Blue => { // quine :)
                            if tokens.len() == 2 {
                                print!("blue right");
                            } else {
                                // muahahaha
                                std::process::exit(0);
                            }
                        },
                        Color::Purple => { // end
                            if current_frame == 0 { panic!("Mismatching purple up/down/right"); }
                            frame.pop();
                            current_frame -= 1;
                        },
                        _ => {}
                    }
                    current_stone = Color::Invis;
                    current_direction = Direction::None;
                }
            },

            // numbers
            Token::One => {
                current_number = Number::One;
                if frame[current_frame] {
                    match current_stone {
                        Color::Red => {
                            match current_direction {
                                Direction::Up => stack.push(0),
                                Direction::Down => stack.push(1),
                                Direction::Left => stack.push(2),
                                Direction::Right => stack.push(3),
                                _ => panic!("Unexpected reserved word!"),
                            }
                            field = move_stone(current_stone, current_direction, field, &mut tokens, count as usize);
                        },
                        Color::Orange => { // array stuff
                            match current_direction {
                                _ => {}
                            }
                            field = move_stone(current_stone, current_direction, field, &mut tokens, count as usize);
                        },
                        _ => println!("That {:?} stone is too heavy!", &current_stone)
                    }
                }
            },
            Token::Two => {
                current_number = Number::Two;
                if frame[current_frame] {
                    match current_stone {
                        Color::Red => {
                            match current_direction {
                                Direction::Up => stack.push(4),
                                Direction::Down => stack.push(5),
                                Direction::Left => stack.push(6),
                                Direction::Right => stack.push(7),
                                _ => panic!("Unexpected reserved word!"),
                            }
                            field = move_stone(current_stone, current_direction, field, &mut tokens, count as usize);
                            field = move_stone(current_stone, current_direction, field, &mut tokens, count as usize);
                        },
                        Color::Orange => { // equality stuff
                            match current_direction {
                                Direction::Up => {
                                    let tmp1 = stack.pop();
                                    let tmp2 = stack.pop();
                                    if tmp1 == tmp2 {
                                        stack.push(1);
                                    } else {
                                        stack.push(0);
                                    }
                                },
                                Direction::Down => {
                                    let tmp1 = stack.pop();
                                    let tmp2 = stack.pop();
                                    if tmp1 < tmp2 {
                                        stack.push(1);
                                    } else {
                                        stack.push(0);
                                    }
                                },
                                Direction::Left => {
                                    let tmp1 = stack.pop();
                                    let tmp2 = stack.pop();
                                    if tmp1 > tmp2 {
                                        stack.push(1);
                                    } else {
                                        stack.push(0);
                                    }
                                },
                                _ => {}
                            }
                            field = move_stone(current_stone, current_direction, field, &mut tokens, count as usize);
                            field = move_stone(current_stone, current_direction, field, &mut tokens, count as usize);
                        },
                        _ => println!("That {:?} stone is too heavy!", &current_stone)
                    }
                }
            },
            Token::Three => {
                current_number = Number::Three;
                if frame[current_frame] {
                    match current_stone {
                        Color::Red => {
                            match current_direction {
                                Direction::Up => stack.push(8),
                                Direction::Down => stack.push(9),
                                Direction::Left => stack.push(1),
                                Direction::Right => stack.push(0),
                                _ => panic!("Unexpected reserved word!"),
                            }
                            field = move_stone(current_stone, current_direction, field, &mut tokens, count as usize);
                            field = move_stone(current_stone, current_direction, field, &mut tokens, count as usize);
                            field = move_stone(current_stone, current_direction, field, &mut tokens, count as usize);
                        },
                        _ => println!("That {:?} stone is too heavy!", &current_stone)
                    }
                }
            },
            _ => { }
        }

        if debug {
            println!("Token:     {:?}", tokens[count]);
            println!("Color:     {:?}", current_stone);
            println!("Direction: {:?}", current_direction);
            println!("Number:    {:?}", current_number);
            println!("Frame:     {}", current_frame);
            println!("Current:   {}", frame[current_frame]);
            println!("-----------");
        }

        if show_stack {
            println!("stack: {}", stack.len());
            for item in &stack {
                println!("{}", item);
            }
        }

        if show_frames {
            println!("frames: {}", frame.len());
            for item in &frame {
                println!("{}", item);
            }
        }

        count += 1;
    }
}

fn is_color(c: Token) -> bool {
    match c {
        Token::Red | Token::Orange | Token::Yellow | Token::Green | Token::Blue | Token::Purple => true,
        _ => false
    }
}

// I wasn't aware that there was a way to enumerate a Vec without moving it
// it's too late to change it right now, but I might consider fixing it later.
#[allow(unknown_lints)]
#[allow(needless_range_loop)]
fn move_stone(stone: Color, dir: Direction, _field: Vec<Vec<Color>>, tokens: &mut Vec<Token>, count: usize)
        -> Vec<Vec<Color>> {
    let mut field = _field; // FIXME: really not sure why
    let field_height = field.len() - 1;   // == 5
    let field_width = field[0].len() - 1; // == 10

    // go through vertical rows first
    'y: for y in 0..(field_height + 1) {
        // columns left to right
        for x in 0..(field_width + 1) {
            // find a match
            if field[y][x] == stone {
                match dir {
                    Direction::Up => {
                        // set color to invisible
                        field[y][x] = Color::Invis;
                        // protect overflow crashes
                        if y != 0 {
                            // check for stone in the way
                            if field[y - 1][x] != Color::Invis {
                                // move it up
                                tokens.insert(count, Token::from_stone(field[y - 1][x]));
                                tokens.insert(count + 1, Token::from_direction(dir));
                                tokens.insert(count + 2, Token::One);
                                field = move_stone(field[y - 1][x], dir, field, tokens, count);
                            }
                            // move stone up one
                            field[y - 1][x] = stone;
                        } else {
                            // check for stone in the way
                            if field[field_height][x] != Color::Invis {
                                tokens.insert(count, Token::from_stone(field[field_height][x]));
                                tokens.insert(count + 1, Token::from_direction(dir));
                                tokens.insert(count + 2, Token::One);
                                field = move_stone(field[field_height][x], dir, field, tokens, count);
                            }
                            // wrap around to bottom
                            field[field_height][x] = stone;
                        }
                        // break loop - put this at the end of every direction
                        break 'y;
                    },
                    Direction::Down => {
                        field[y][x] = Color::Invis;
                        if y != field_height {
                            if field[y + 1][x] != Color::Invis {
                                tokens.insert(count, Token::from_stone(field[y + 1][x]));
                                tokens.insert(count + 1, Token::from_direction(dir));
                                tokens.insert(count + 2, Token::One);
                                field = move_stone(field[y + 1][x], dir, field, tokens, count);
                            }
                            // move stone down one
                            field[y + 1][x] = stone;
                        } else {
                            if field[0][x] != Color::Invis {
                                tokens.insert(count, Token::from_stone(field[0][x]));
                                tokens.insert(count + 1, Token::from_direction(dir));
                                tokens.insert(count + 2, Token::One);
                                field = move_stone(field[0][x], dir, field, tokens, count);
                            }
                            // wrap to bottom
                            field[0][x] = stone;
                        }
                        break 'y;
                    },
                    Direction::Left => {
                        field[y][x] = Color::Invis;
                        if x != 0 {
                            if field[y][x - 1] != Color::Invis {
                                tokens.insert(count, Token::from_stone(field[y][x - 1]));
                                tokens.insert(count + 1, Token::from_direction(dir));
                                tokens.insert(count + 2, Token::One);
                                field = move_stone(field[y][x - 1], dir, field, tokens, count);
                            }
                            field[y][x - 1] = stone;
                        } else {
                            if field[y][field_width] != Color::Invis {
                                tokens.insert(count, Token::from_stone(field[y][field_width]));
                                tokens.insert(count + 1, Token::from_direction(dir));
                                tokens.insert(count + 2, Token::One);
                                field = move_stone(field[y][field_width], dir, field, tokens, count);
                            }
                            field[y][field_width] = stone;
                        }
                        break 'y;
                    },
                    Direction::Right => {
                        field[y][x] = Color::Invis;
                        if x != field_width {
                            if field[y][x + 1] != Color::Invis {
                                tokens.insert(count, Token::from_stone(field[y][x + 1]));
                                tokens.insert(count + 1, Token::from_direction(dir));
                                tokens.insert(count + 2, Token::One);
                                field = move_stone(field[y][x + 1], dir, field, tokens, count);
                            }
                            field[y][x + 1] = stone;
                        } else {
                            if field[y][0] != Color::Invis {
                                tokens.insert(count, Token::from_stone(field[y][0]));
                                tokens.insert(count + 1, Token::from_direction(dir));
                                tokens.insert(count + 2, Token::One);
                                field = move_stone(field[y][0], dir, field, tokens, count);
                            }
                            field[y][0] = stone;
                        }
                        break 'y;
                    },
                    _ => {
                        //println!("can't move {:?} {:?} yet", stone, dir);
                        break 'y;
                    }
                }
            }
        }
    }

    field
}

// read from a file, convert text to stones commands
#[allow(unknown_lints)]
#[allow(match_same_arms)]
fn do_print(filename: String) { // {{{1
    let path = Path::new(&filename);
    let display = path.display();

    let mut file = match File::open(&path) {
        Ok(file) => file,
        Err(_) => {
            println!("File doesn't exist: {}", display);
            std::process::exit(1);
        }
    };

    let mut contents = String::new();
    match file.read_to_string(&mut contents) {
        Ok(_) => {},
        Err(e) => panic!("Couldn't read {}: {}", display, e.description()),
    }

    let chars = contents.chars();

    let mut output = String::new();

    for c in chars {
        output.push_str(
            match c {
                '\t' => "red down 3\nblue left\n\n",
                '\n' => "red down 2\nred down 2\nyellow down\nblue left\n\n",
                '\r' => "red down 3\nred up 2\nyellow down\nblue left\n\n",
                ' ' => "red up 2\nred up 2\nred left 1\nyellow up\nyellow up\nblue left\n\n",
                '!' => "red right 1\nred left 1\nred down 3\nyellow down\nyellow up\nblue left\n\n",
                '\"' => "red down 2\nred down 2\nred right 2\nyellow down\nyellow down \nred left 1\nyellow up\nred right 1\nyellow down\nblue left\n\n",
                '#' => "red right 2\nred down 2\nyellow up\nblue left\n\n",
                '$' => "red left 2\nred left 2\nyellow up\nblue left\n\n",
                '%' => "red down 2\nred down 2\nyellow down\nred right 1\nyellow up\nred right 2\nyellow down\nblue left\n\n",
                '&' => "red down 3\nred up 3\nyellow down\nred left 1\nyellow down\nred left 1\nyellow up\nblue left\n\n",
                '\'' => "red up 2\nred down 3\nyellow down\nred right 1\nyellow up\nblue left\n\n",
                '(' => "red down 2\nred down 2\nyellow down\nred up 2\nyellow up\nblue left\n\n",
                ')' => "red down 2\nred down 2\nyellow down\nred up 2\nyellow up\nred down 1\nyellow down\nblue left\n\n",
                '*' => "red left 2\nred right 2\nyellow up\nblue left\n\n",
                '+' => "red left 2\nred right 2\nyellow up\nred down 1\nyellow down\nblue left\n\n",
                ',' => "red left 1\nred down 3\nyellow down\nred up 2\nyellow up\nblue left\n\n",
                '-' => "red down 3\nred down 2\nyellow up\nblue left\n\n",
                '.' => "red down 3\nred down 2\nyellow up\nred down 1\nyellow down\nblue left\n\n",
                '/' => "red down 3\nred down 2\nyellow up\nred left 1\nyellow down\nblue left\n\n",
                '0' => "red left 2\nred up 3\nyellow up\nblue left\n\n",
                '1' => "red left 2\nred up 3\nyellow up\nred down 1\nyellow down\nblue left\n\n",
                '2' => "red down 2\nred left 1\nred down 2\nyellow up\nyellow up\nblue left\n\n",
                '3' => "\n", // yeah, this is awful
                '4' => "\n",
                '5' => "\n",
                '6' => "\n",
                '7' => "\n",
                '8' => "\n",
                '9' => "\n",
                ':' => "\n",
                ';' => "\n",
                '<' => "\n",
                '=' => "\n",
                '>' => "\n",
                '?' => "\n",
                '@' => "\n",
                'A' => "\n",
                'B' => "\n",
                'C' => "\n",
                'D' => "\n",
                'E' => "\n",
                'F' => "\n",
                'G' => "\n",
                'H' => "\n",
                'I' => "\n",
                'J' => "\n",
                'K' => "\n",
                'L' => "\n",
                'M' => "\n",
                'N' => "\n",
                'O' => "\n",
                'P' => "\n",
                'Q' => "\n",
                'R' => "\n",
                'S' => "\n",
                'T' => "\n",
                'U' => "\n",
                'V' => "\n",
                'W' => "\n",
                'X' => "\n",
                'Y' => "\n",
                'Z' => "\n",
                '[' => "\n",
                '\\' => "\n",
                ']' => "\n",
                '^' => "\n",
                '_' => "\n",
                '`' => "\n",
                'a' => "\n",
                'b' => "\n",
                'c' => "\n",
                'd' => "\n",
                'e' => "\n",
                'f' => "\n",
                'g' => "\n",
                'h' => "\n",
                'i' => "\n",
                'j' => "\n",
                'k' => "\n",
                'l' => "\n",
                'm' => "\n",
                'n' => "\n",
                'o' => "\n",
                'p' => "\n",
                'q' => "\n",
                'r' => "\n",
                's' => "\n",
                't' => "\n",
                'u' => "\n",
                'v' => "\n",
                'w' => "\n",
                'x' => "\n",
                'y' => "\n",
                'z' => "\n",
                '{' => "\n",
                '|' => "\n",
                '}' => "\n",
                '~' => "\n",
                _ => "\n",
        });
    }

    println!("{}", output);
}
// }}}1

