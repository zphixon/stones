/* Stones esoteric programming language
 * (c) 2016 Zack Hixon - see LICENSE.txt */

static VERSION: &'static str = "0.3.0";

extern crate argparse;

use argparse::{ArgumentParser, StoreTrue, Store, Print};

use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::error::Error;

// color enum, represent stones
#[derive(Debug, PartialEq, Clone, Copy)]
enum Color {
    Red,
    Orange,
    Yellow,
    Green,
    Blue,
    Purple,
    Invis
}

// direction enum, used for parsing directions
#[derive(Debug, PartialEq, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
    No
}

// main() has a cyclomatic complexity of 54. should I be proud?
#[allow(unknown_lints)]
#[allow(cyclomatic_complexity)]
fn main() {
    // arguments
    let mut debug = false;
    let mut show_field = false;
    let mut show_stack = false;
    let mut create_print = false;
    let mut filename: String = "".into();

    {
        let mut args = ArgumentParser::new();
        args.set_description("Run a stones language file");
        args.refer(&mut debug)
            .add_option(&["-d", "--debug"], StoreTrue, "Run debugging");
        args.refer(&mut show_field)
            .add_option(&["-f", "--field"], StoreTrue, "Show field");
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
    let tokens: Vec<&str> = s.split_whitespace().collect();

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

    // store scopes/wether or not to execute
    let mut frame: Vec<bool> = vec![true];
    let mut current_frame = 0;

    // execute program
    let mut count = 0; // track token counter
    let mut current_stone = Color::Invis; // keep track of current stone
    let mut current_direction = Direction::No; // track direction
    let mut current_number = "none"; // used for red movement

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
            "red" => {
                current_stone = Color::Red;
                current_direction = Direction::No;
                current_number = "none";
                if count != 0 && is_color(tokens[count - 1]) {
                    println!("Unexpected color!");
                }
                if count != tokens.len() - 2 &&
                   tokens[count + 2] != "1" &&
                   tokens[count + 2] != "2" &&
                   tokens[count + 2] != "3" {
                    println!("Expected number!");
                }
            },
            "orange" => {
                current_stone = Color::Orange;
                current_direction = Direction::No;
                current_number = "none";
                if count != 0 && is_color(tokens[count - 1]) {
                    println!("Unexpected color!");
                }
                if count != tokens.len() - 2 &&
                   tokens[count + 2] != "1" &&
                   tokens[count + 2] != "2" {
                        println!("Expected number!");
                }
            },
            "yellow" => {
                current_stone = Color::Yellow;
                current_direction = Direction::No;
                current_number = "none";
                if count != 0 && is_color(tokens[count - 1]) {
                    println!("Unexpected color!");
                }
            },
            "green" => {
                current_stone = Color::Green;
                current_direction = Direction::No;
                current_number = "none";
                if count != 0 && is_color(tokens[count - 1]) {
                    println!("Unexpected color!");
                }
            },
            "blue" => {
                current_stone = Color::Blue;
                current_direction = Direction::No;
                current_number = "none";
                if count != 0 && is_color(tokens[count - 1]) {
                    println!("Unexpected color!");
                }
            },
            "purple" => {
                current_stone = Color::Purple;
                current_direction = Direction::No;
                current_number = "none";
                if count != 0 && is_color(tokens[count - 1]) {
                    println!("Unexpected color!");
                }
            },

            // directions
            "up" => {
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
                                field = move_stone(current_stone, current_direction, field);
                            }
                        },
                        //Color::Green => { // roll },
                        Color::Blue => { // print as number
                            if frame[current_frame] {
                                print!("{}\n", stack.pop().expect("Stack is empty!"));
                                field = move_stone(current_stone, current_direction, field);
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
                                field = move_stone(current_stone, current_direction, field);
                            }
                        },
                        _ => {}
                    }
                    current_stone = Color::Invis;
                    current_direction = Direction::No;
                }
            },
            "down" => {
                current_direction = Direction::Down;
                if current_stone != Color::Red && current_stone != Color::Orange {
                    match current_stone {
                        Color::Yellow => { // add
                            if frame[current_frame] {
                                let tmp1 = stack.pop().expect("Stack is empty!");
                                let tmp2 = stack.pop().expect("Stack is empty!");
                                stack.push(tmp1 + tmp2);
                                field = move_stone(current_stone, current_direction, field);
                            }
                        },
                        //Color::Green => { // dup
                        //    let tmp = stack.pop().expect("Stack is empty!");
                        //    stack.push(&tmp); // might cause problemos...
                        //    stack.push(&tmp);
                        //},
                        //Color::Blue => { // input },
                        Color::Purple => { // else
                            frame[current_frame] = !frame[current_frame];
                            field = move_stone(current_stone, current_direction, field);
                        },
                        _ => {}
                    }
                    current_stone = Color::Invis;
                    current_direction = Direction::No;
                }
            },
            "left" => {
                current_direction = Direction::Left;
                if current_stone != Color::Red && current_stone != Color::Orange {
                    match current_stone {
                        Color::Yellow => { // subtract
                            if frame[current_frame] {
                                let tmp1 = stack.pop().expect("Stack is empty!");
                                let tmp2 = stack.pop().expect("Stack is empty!");
                                stack.push(tmp1 - tmp2);
                                field = move_stone(current_stone, current_direction, field);
                            }
                        },
                        //Color::Green => { // drop },
                        Color::Blue => { // print as character
                            if frame[current_frame] {
                                // ewwww
                                print!("{}", to_char(stack.pop().expect("Stack is empty!")));
                                field = move_stone(current_stone, current_direction, field);
                            }
                        },
                        //Color::Purple => { // while },
                        _ => {}
                    }
                    current_stone = Color::Invis;
                    current_direction = Direction::No;
                }
            },
            "right" => {
                current_direction = Direction::Right;
                if current_stone != Color::Red && current_stone != Color::Orange {
                    match current_stone {
                        //Color::Orange => {},
                        Color::Yellow => { // divide
                            if frame[current_frame] {
                                let tmp1 = stack.pop().expect("Stack is empty!");
                                let tmp2 = stack.pop().expect("Stack is empty!");
                                stack.push(tmp1 / tmp2);
                                field = move_stone(current_stone, current_direction, field);
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
                    current_direction = Direction::No;
                }
            },

            // numbers
            "1" => {
                current_number = "1";
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
                            field = move_stone(current_stone, current_direction, field);
                        },
                        Color::Orange => { // array stuff
                            match current_direction {
                                _ => {}
                            }
                            field = move_stone(current_stone, current_direction, field);
                        },
                        _ => println!("That {:?} stone is too heavy!", &current_stone)
                    }
                }
            },
            "2" => {
                current_number = "2";
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
                            field = move_stone(current_stone, current_direction, field);
                            field = move_stone(current_stone, current_direction, field);
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
                            field = move_stone(current_stone, current_direction, field);
                            field = move_stone(current_stone, current_direction, field);
                        },
                        _ => println!("That {:?} stone is too heavy!", &current_stone)
                    }
                }
            },
            "3" => {
                current_number = "3";
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
                            field = move_stone(current_stone, current_direction, field);
                            field = move_stone(current_stone, current_direction, field);
                            field = move_stone(current_stone, current_direction, field);
                        },
                        _ => println!("That {:?} stone is too heavy!", &current_stone)
                    }
                }
            },
            _ => { }
        }

        if debug {
            println!("Token:     {}", tokens[count]);
            println!("Color:     {:?}", current_stone);
            println!("Direction: {:?}", current_direction);
            println!("Number:    {}", current_number);
            println!("Frame:     {}", current_frame);
            println!("Current:   {}", frame[current_frame]);
            println!("-----------");
        }

        if show_stack {
            println!("-------");
            for item in &stack {
                println!("{}", item);
            }
        }

        count += 1;
    }
}

fn is_color(c: &str) -> bool {
    match c {
        "red" | "orange" | "yellow" | "green" | "blue" | "purple" => true,
        _ => false
    }
}

// I wasn't aware that there was a way to enumerate a Vec without moving it
// it's too late to change it right now, but I might consider fixing it later.
#[allow(unknown_lints)]
#[allow(needless_range_loop)]
fn move_stone(stone: Color, dir: Direction, _field: Vec<Vec<Color>>)
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
                                field = move_stone(field[y - 1][x], dir, field);
                            }
                            // move stone up one
                            field[y - 1][x] = stone;
                        } else {
                            // check for stone in the way
                            if field[field_height][x] != Color::Invis {
                                field = move_stone(field[field_height][x], dir, field);
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
                                field = move_stone(field[y + 1][x], dir, field);
                            }
                            // move stone down one
                            field[y + 1][x] = stone;
                        } else {
                            if field[0][x] != Color::Invis {
                                field = move_stone(field[0][x], dir, field);
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
                                field = move_stone(field[y][x - 1], dir, field);
                            }
                            field[y][x - 1] = stone;
                        } else {
                            if field[y][field_width] != Color::Invis {
                                field = move_stone(field[y][field_width], dir, field);
                            }
                            field[y][field_width] = stone;
                        }
                        break 'y;
                    },
                    Direction::Right => {
                        field[y][x] = Color::Invis;
                        if x != field_width {
                            if field[y][x + 1] != Color::Invis {
                                field = move_stone(field[y][x + 1], dir, field);
                            }
                            field[y][x + 1] = stone;
                        } else {
                            if field[y][0] != Color::Invis {
                                field = move_stone(field[y][0], dir, field);
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
fn do_print(filename: String) {
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

// copied from superfish.rs
fn to_char(i: i64) -> String {
    let a = match i {
        0 => "NUL",
        1 => "SOH",
        2 => "STX",
        3 => "ETX",
        4 => "EOT",
        5 => "ENQ",
        6 => "ACK",
        7 => "BEL",
        8 => "BS",
        9 => "\t",
        10 => "\n",
        11 => "VT",
        12 => "TT",
        13 => "\r",
        14 => "SO",
        15 => "SI",
        16 => "DLE",
        17 => "DC1",
        18 => "DC2",
        19 => "DC3",
        20 => "DC4",
        21 => "NAK",
        22 => "SYN",
        23 => "ETB",
        24 => "CAN",
        25 => "EM",
        26 => "SUB",
        27 => "ESC",
        28 => "FS",
        29 => "GS",
        30 => "RS",
        31 => "US",
        32 => " ",
        33 => "!",
        34 => "\"",
        35 => "#",
        36 => "$",
        37 => "%",
        38 => "&",
        39 => "\'",
        40 => "(",
        41 => ")",
        42 => "*",
        43 => "+",
        44 => ",",
        45 => "-",
        46 => ".",
        47 => "/",
        48 => "0",
        49 => "1",
        50 => "2",
        51 => "3",
        52 => "4",
        53 => "5",
        54 => "6",
        55 => "7",
        56 => "8",
        57 => "9",
        58 => ":",
        59 => ";",
        60 => "<",
        61 => "=",
        62 => ">",
        63 => "?",
        64 => "@",
        65 => "A",
        66 => "B",
        67 => "C",
        68 => "D",
        69 => "E",
        70 => "F",
        71 => "G",
        72 => "H",
        73 => "I",
        74 => "J",
        75 => "K",
        76 => "L",
        77 => "M",
        78 => "N",
        79 => "O",
        80 => "P",
        81 => "Q",
        82 => "R",
        83 => "S",
        84 => "T",
        85 => "U",
        86 => "V",
        87 => "W",
        88 => "X",
        89 => "Y",
        90 => "Z",
        91 => "[",
        92 => "\\",
        93 => "]",
        94 => "^",
        95 => "_",
        96 => "`",
        97 => "a",
        98 => "b",
        99 => "c",
        100 => "d",
        101 => "e",
        102 => "f",
        103 => "g",
        104 => "h",
        105 => "i",
        106 => "j",
        107 => "k",
        108 => "l",
        109 => "m",
        110 => "n",
        111 => "o",
        112 => "p",
        113 => "q",
        114 => "r",
        115 => "s",
        116 => "t",
        117 => "u",
        118 => "v",
        119 => "w",
        120 => "x",
        121 => "y",
        122 => "z",
        123 => "{",
        124 => "|",
        125 => "}",
        126 => "~",
        127 => "DEL",
        _ => "",
    };
    String::from(a)
}
