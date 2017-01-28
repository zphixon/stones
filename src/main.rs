/* Stones esoteric programming language
 * (c) 2016 Zack Hixon - see LICENSE.txt */

static VERSION: &'static str = "0.4.0";

extern crate argparse;
extern crate stones;

use stones::*;

use argparse::{ArgumentParser, StoreTrue, Store, Print};

use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::error::Error;

static mut DEBUG: bool = false;
static mut SHOW_FIELD: bool = false;
static mut SHOW_STACK: bool = false;

fn main() {
    // arguments
    let mut debug = false;
    let mut show_field = false;
    let mut show_stack = false;
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
        args.refer(&mut filename)
            .add_argument("file", Store, "File to run/output to")
            .required();
        args.add_option(&["-V", "--version"],
                        Print("stones version ".to_string() + VERSION), "Show version");
        args.parse_args_or_exit();
    }

    unsafe {
        DEBUG = debug;
        SHOW_FIELD = show_field;
        SHOW_STACK = show_stack;
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
    let stmts = parse(tokens);

    if stmts.is_ok() {
        let mut stack: Vec<Value> = vec![];
        let mut field: Vec<Vec<Color>> = Field::new();
        eval_prog(stmts.unwrap(), &mut field, &mut stack);
    } else {
        println!("couldn't parse");
    }
}

fn eval_prog(prog: Vec<Statement>, field: &mut Vec<Vec<Color>>, stack: &mut Vec<Value>) { // {{{
    let mut frames = vec![true];
    let mut current_frame = frames.len() - 1;

    let mut whiles = vec![false];
    let mut current_while = whiles.len() - 1;

    let mut nmove = 0;
    let mut arraying = false;
    let mut fake_array: Vec<Value> = vec![];

    let mut k = 0;

    while k < prog.len() {
        let stmt = prog[k];

        unsafe {
            if DEBUG {
                println!("{}: {:?} {:?} {:?} {} {} {} {}", k, stmt.color, stmt.direction, stmt.number,
                         frames[current_frame], frames.len(), whiles[current_while], whiles.len());
            }

            if SHOW_STACK {
                println!("{}: stack", k);
                for val in stack.clone() {
                    println!("{:?}", val);
                }
                if !fake_array.is_empty() {
                    println!("{}: arraying", k);
                    for val in &fake_array {
                        println!("{:?}", val);
                    }
                }
            }

            if SHOW_FIELD {
                println!("{}: field", k);
                for row in field.clone() {
                    for color in row {
                        match color {
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
        }

        if stmt.color == Color::Red {
            if frames[current_frame] {
                if stmt.direction == Direction::Up {
                    if stmt.number == Number::One {                                       // 0
                        if move_field(stmt.color, stmt.direction, field, stack) {
                            stack.push(Value::Num(0));
                        }
                    } else if stmt.number == Number::Two {                                // 4
                        if move_field(stmt.color, stmt.direction, field, stack)
                                && move_field(stmt.color, stmt.direction, field, stack) {
                            stack.push(Value::Num(4));
                        }
                    } else if stmt.number == Number::Three {                              // 8
                        if move_field(stmt.color, stmt.direction, field, stack)
                                && move_field(stmt.color, stmt.direction, field, stack)
                                && move_field(stmt.color, stmt.direction, field, stack) {
                            stack.push(Value::Num(8));
                        }
                    }
                } else if stmt.direction == Direction::Down {
                    if stmt.number == Number::One {                                       // 1
                        if move_field(stmt.color, stmt.direction, field, stack) {
                            stack.push(Value::Num(1));
                        }
                    } else if stmt.number == Number::Two {                                // 5
                        if move_field(stmt.color, stmt.direction, field, stack)
                                && move_field(stmt.color, stmt.direction, field, stack) {
                            stack.push(Value::Num(5));
                        }
                    } else if stmt.number == Number::Three {                              // 9
                        if move_field(stmt.color, stmt.direction, field, stack)
                                && move_field(stmt.color, stmt.direction, field, stack)
                                && move_field(stmt.color, stmt.direction, field, stack) {
                            stack.push(Value::Num(9));
                        }
                    }
                } else if stmt.direction == Direction::Left {
                    if stmt.number == Number::One {                                       // 2
                        if move_field(stmt.color, stmt.direction, field, stack) {
                            stack.push(Value::Num(2));
                        }
                    } else if stmt.number == Number::Two {                                // 6
                        if move_field(stmt.color, stmt.direction, field, stack)
                                && move_field(stmt.color, stmt.direction, field, stack) {
                            stack.push(Value::Num(6));
                        }
                    } else if stmt.number == Number::Three {                              // true
                        if move_field(stmt.color, stmt.direction, field, stack)
                                && move_field(stmt.color, stmt.direction, field, stack)
                                && move_field(stmt.color, stmt.direction, field, stack) {
                            stack.push(Value::Bool(true));
                        }
                    }
                } else if stmt.direction == Direction::Right {
                    if stmt.number == Number::One {                                       // 3
                        if move_field(stmt.color, stmt.direction, field, stack) {
                            stack.push(Value::Num(3));
                        }
                    } else if stmt.number == Number::Two {                                // 7
                        if move_field(stmt.color, stmt.direction, field, stack)
                                && move_field(stmt.color, stmt.direction, field, stack) {
                            stack.push(Value::Num(7));
                        }
                    } else if stmt.number == Number::Three {                              // false
                        if move_field(stmt.color, stmt.direction, field, stack)
                                && move_field(stmt.color, stmt.direction, field, stack)
                                && move_field(stmt.color, stmt.direction, field, stack) {
                            stack.push(Value::Bool(false));
                        }
                    }
                }
            }
        }

        else if stmt.color == Color::Orange {
            if frames[current_frame] {
                if stmt.direction == Direction::Up {
                    if stmt.number == Number::One {                                       // [
                        if move_field(stmt.color, stmt.direction, field, stack) {
                            arraying = true;
                        }
                    } else if stmt.number == Number::Two {                                // ==
                        if move_field(stmt.color, stmt.direction, field, stack)
                                && move_field(stmt.color, stmt.direction, field, stack) {
                            let lhs = stack.pop().expect("Stack underflow");
                            let rhs = stack.pop().expect("Stack underflow");
                            if lhs == rhs {
                                stack.push(Value::Bool(true));
                            } else {
                                stack.push(Value::Bool(false));
                            }
                        }
                    }
                } else if stmt.direction == Direction::Down {
                    if stmt.number == Number::One {                                       // ]
                        if move_field(stmt.color, stmt.direction, field, stack) {
                            arraying = false;
                            stack.push(Value::Arr(fake_array.clone()));
                            fake_array = vec![];
                        }
                    } else if stmt.number == Number::Two {                                // <
                        if move_field(stmt.color, stmt.direction, field, stack)
                                && move_field(stmt.color, stmt.direction, field, stack) {
                            let lhs = stack.pop().expect("Stack underflow");
                            let rhs = stack.pop().expect("Stack underflow");
                            if !(lhs < rhs) { // hold up
                                stack.push(Value::Bool(true));
                            } else {
                                stack.push(Value::Bool(false));
                            }
                        }
                    }
                } else if stmt.direction == Direction::Left {
                    if stmt.number == Number::One {                                       // ,
                        if move_field(stmt.color, stmt.direction, field, stack) {
                            if arraying {
                                fake_array.push(stack.pop().expect("Stack underflow"));
                            } else {
                                //panic!("Not arraying");
                            }
                        }
                    } else if stmt.number == Number::Two {                                // >
                        if move_field(stmt.color, stmt.direction, field, stack)
                                && move_field(stmt.color, stmt.direction, field, stack) {
                            let lhs = stack.pop().expect("Stack underflow");
                            let rhs = stack.pop().expect("Stack underflow");
                            if !(lhs > rhs) { // what the hell?
                                stack.push(Value::Bool(true));
                            } else {
                                stack.push(Value::Bool(false));
                            }
                        }
                    }
                } else if stmt.direction == Direction::Right {
                    if stmt.number == Number::One {                                       // nth
                        if move_field(stmt.color, stmt.direction, field, stack) {
                            let arr = stack.pop().expect("Stack underflow");
                            let ind = stack.pop().expect("Stack underflow");
                            if arr.is_arr() && ind.is_num() {
                                let rarr = match arr {
                                    Value::Arr(a) => a,
                                    _ => panic!("everthing is awful")
                                };
                                let rind = match ind {
                                    Value::Num(i) => i,
                                    _ => panic!("everything is awful")
                                };
                                stack.push(rarr[rind as usize].clone());
                            }
                        }
                    } else if stmt.number == Number::Two {                                // nothing yet
                        if move_field(stmt.color, stmt.direction, field, stack)
                                && move_field(stmt.color, stmt.direction, field, stack) {
                        }
                    }
                }
            }
        }

        else if stmt.color == Color::Yellow {
            if frames[current_frame] {
                if stmt.direction == Direction::Up {                                      // *
                    if move_field(stmt.color, stmt.direction, field, stack) {
                    }
                } else if stmt.direction == Direction::Down {                             // +
                    if move_field(stmt.color, stmt.direction, field, stack) {
                    }
                } else if stmt.direction == Direction::Left {                             // -
                    if move_field(stmt.color, stmt.direction, field, stack) {
                    }
                } else if stmt.direction == Direction::Right {                            // /
                    if move_field(stmt.color, stmt.direction, field, stack) {
                    }
                }
            }
        }

        else if stmt.color == Color::Green {
            if frames[current_frame] {
                if stmt.direction == Direction::Up {                                      // roll
                    if move_field(stmt.color, stmt.direction, field, stack) {
                    }
                } else if stmt.direction == Direction::Down {                             // dup
                    if move_field(stmt.color, stmt.direction, field, stack) {
                    }
                } else if stmt.direction == Direction::Left {                             // drop
                    if move_field(stmt.color, stmt.direction, field, stack) {
                    }
                } else if stmt.direction == Direction::Right {                            // not
                    if move_field(stmt.color, stmt.direction, field, stack) {
                    }
                }
            }
        }

        else if stmt.color == Color::Blue {
            if frames[current_frame] {
                if stmt.direction == Direction::Up {                                      // print
                    if move_field(stmt.color, stmt.direction, field, stack) {
                    }
                } else if stmt.direction == Direction::Down {                             // input
                    if move_field(stmt.color, stmt.direction, field, stack) {
                    }
                } else if stmt.direction == Direction::Left {                             // printc
                    if move_field(stmt.color, stmt.direction, field, stack) {
                    }
                } else if stmt.direction == Direction::Right {                            // nothing yet
                    if move_field(stmt.color, stmt.direction, field, stack) {
                    }
                }
            }
        }

        else if stmt.color == Color::Purple {
            if stmt.direction == Direction::Up {                                          // if
                if move_field(stmt.color, stmt.direction, field, stack) {
                }
            } else if stmt.direction == Direction::Down {                                 // else
                if move_field(stmt.color, stmt.direction, field, stack) {
                }
            } else if stmt.direction == Direction::Left {                                 // while
                if move_field(stmt.color, stmt.direction, field, stack) {
                }
            } else if stmt.direction == Direction::Right {                                // end
                if move_field(stmt.color, stmt.direction, field, stack) {
                }
            }
        }

        k += 1;
    }

    unsafe {
        if DEBUG {
            println!("{}: end", k);
        }

        if SHOW_STACK {
            println!("{}: stack end", k);
            for val in stack.clone() {
                println!("{:?}", val);
            }
        }

        if SHOW_FIELD {
            println!("{}: field end", k);
            for row in field {
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
    }
} // }}}

fn move_field(target: Color, dir: Direction, field: &mut Vec<Vec<Color>>, stack: &mut Vec<Value>) -> bool { // {{{
    let field_height = field.len() - 1;
    let field_width = field[0].len() - 1;

    // loop through rows
    for y in 0..6 {
        // loop through columns
        for x in 0..12 {
            // search for target
            if field[y][x] == target {
                // match direction
                if dir == Direction::Up {
                    // check for wrap-around
                    if y != 0 {
                        // check weight of stone
                        if field[y - 1][x] <= target {
                            if field[y - 1][x] != Color::Invis {
                                let tm = field[y - 1][x];
                                eval_prog(vec![Statement::new(tm, Direction::Up, Number::One)], field, stack);
                            }
                            field[y][x] = Color::Invis;
                            field[y - 1][x] = target;
                            return true;
                        } else {
                            return false;
                        }
                    } else {
                        // wrapping around
                        if field[field_height][x] <= target {
                            if field[field_height][x] != Color::Invis {
                                let tm = field[field_height][x];
                                eval_prog(vec![Statement::new(tm, Direction::Up, Number::One)], field, stack);
                            }
                            field[y][x] = Color::Invis;
                            field[field_height][x] = target;
                            return true;
                        } else {
                            return false;
                        }
                    }
                } else if dir == Direction::Down {
                    if y != field_height {
                        if field[y + 1][x] <= target {
                            if field[y + 1][x] != Color::Invis {
                                let tm = field[y + 1][x];
                                eval_prog(vec![Statement::new(tm, Direction::Down, Number::One)], field, stack);
                            }
                            field[y][x] = Color::Invis;
                            field[y + 1][x] = target;
                            return true;
                        } else {
                            return false;
                        }
                    } else {
                        if field[0][x] <= target {
                            if field[0][x] != Color::Invis {
                                let tm = field[0][x];
                                eval_prog(vec![Statement::new(tm, Direction::Down, Number::One)], field, stack);
                            }
                            field[y][x] = Color::Invis;
                            field[0][x] = target;
                            return true;
                        } else {
                            return false;
                        }
                    }
                } else if dir == Direction::Left {
                    if x != 0 {
                        if field[y][x - 1] <= target {
                            if field[y][x - 1] != Color::Invis {
                                let tm = field[y][x - 1];
                                eval_prog(vec![Statement::new(tm, Direction::Left, Number::One)], field, stack);
                            }
                            field[y][x] = Color::Invis;
                            field[y][x - 1] = target;
                            return true;
                        } else {
                            return false;
                        }
                    } else {
                        if field[y][field_width] <= target {
                            if field[y][field_width] != Color::Invis {
                                let tm = field[y][field_width];
                                eval_prog(vec![Statement::new(tm, Direction::Left, Number::One)], field, stack);
                            }
                            field[y][x] = Color::Invis;
                            field[y][field_width] = target;
                            return true;
                        } else {
                            return false;
                        }
                    }
                } else if dir == Direction::Right {
                    if x != field_width {
                        if field[y][x + 1] <= target {
                            if field[y][x + 1] != Color::Invis {
                                let tm = field[y][x + 1];
                                eval_prog(vec![Statement::new(tm, Direction::Right, Number::One)], field, stack);
                            }
                            field[y][x] = Color::Invis;
                            field[y][x + 1] = target;
                            return true;
                        } else {
                            return false;
                        }
                    } else {
                        if field[y][0] <= target {
                            if field[y][0] != Color::Invis {
                                let tm = field[y][0];
                                eval_prog(vec![Statement::new(tm, Direction::Left, Number::One)], field, stack);
                            }
                            field[y][x] = Color::Invis;
                            field[y][0] = target;
                            return true;
                        } else {
                            return false;
                        }
                    }
                }
            }
        }
    }

    true
} // }}}

