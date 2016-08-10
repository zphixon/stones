/* Stones esoteric programming language
 * (c) 2016 Zack Hixon - see LICENSE.txt */

extern crate argparse;

use argparse::{ArgumentParser, StoreTrue, Store};

use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::error::Error;
use std::process;

#[derive(Debug)]
enum Color {
    Red,
    Orange,
    Yellow,
    Green,
    Blue,
    Purple,
    Invis
}

fn main() {
    let mut field: Vec<Vec<Color>> = /* so much for 80 columns */
        vec![vec![Color::Blue,  Color::Invis, Color::Invis, Color::Invis, Color::Invis, Color::Invis, Color::Orange,Color::Invis, Color::Invis, Color::Invis, Color::Invis ],
             vec![Color::Invis, Color::Invis, Color::Invis, Color::Invis, Color::Invis, Color::Invis, Color::Invis, Color::Invis, Color::Invis, Color::Invis, Color::Invis ],
             vec![Color::Invis, Color::Invis, Color::Red,   Color::Invis, Color::Invis, Color::Invis, Color::Invis, Color::Invis, Color::Green, Color::Invis, Color::Invis ],
             vec![Color::Invis, Color::Invis, Color::Invis, Color::Invis, Color::Invis, Color::Invis, Color::Invis, Color::Invis, Color::Invis, Color::Invis, Color::Invis ],
             vec![Color::Invis, Color::Invis, Color::Invis, Color::Invis, Color::Yellow,Color::Invis, Color::Invis, Color::Invis, Color::Invis, Color::Invis, Color::Purple],
             vec![Color::Invis, Color::Invis, Color::Invis, Color::Invis, Color::Invis, Color::Invis, Color::Invis, Color::Invis, Color::Invis, Color::Invis, Color::Invis ]];

    let mut debug = true;
    let mut filename: String = "".into();

    {
        let mut args = ArgumentParser::new();
        args.set_description("Run a stones language file");
        args.refer(&mut debug)
            .add_option(&["-d", "--debug"], StoreTrue, "Run debugging");
        args.refer(&mut filename)
            .add_argument("file", Store, "File to run")
            .required();
        args.parse_args_or_exit();
    }

    /* open file */
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
        Ok(_) => println!("Opened file, yay!"),
        Err(e) => panic!("Couldn't read {}: {}", display, e.description()),
    }

    /* split file into tokens */
    let tokens: Vec<&str> = s.split_whitespace().collect();

    for token in tokens {
        println!("{}", token);
    }

    if debug {
        for row in &field {
            for color in row {
                match color {
                    &Color::Red => print!("{:?}... ", color),
                    &Color::Orange => print!("{:?} ", color),
                    &Color::Yellow => print!("{:?} ", color),
                    &Color::Green => print!("{:?}. ", color),
                    &Color::Blue => print!("{:?}.. ", color),
                    &Color::Purple => print!("{:?} ", color),
                    &Color::Invis => print!("...... "), /* oh. */
                }
            }
            println!("");
        }
    }
}

