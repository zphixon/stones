/* Stones esoteric programming language
 * (c) 2016 Zack Hixon - see LICENSE.txt */

/** NOTE: Hey there! This isn't even remotely started yet. I'm gonna need a
 * WHOLE lot of stuff to make this even *resemble* an interpreter. It also
 * isn't currently up-to-date with the "specification," so it features
 * misleading stuff like the Stone struct right below this, along with the
 * field vector.  Expect a lot of this stuff to change. */

#[derive(Debug)]
struct Stone {
    x: i32,
    y: i32,
    color: Color
}

impl Stone {
    fn new(x: i32, y: i32, color: Color) -> Stone {
        Stone {
            x: x,
            y: y,
            color: color
        }
    }

    /* only check stones with 0 <= x <= 9 and 0 <= y <= 9 */
    fn empty() -> Stone {
        Stone {
            x: -1,
            y: -1,
            color: Color::Invis
        }
    }
}

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
    println!("test stone: (debug)");

    let mut test_stone = Stone {
        x: 0,
        y: 0,
        color: Color::Yellow
    };

    println!("{:?}", test_stone);

    test_stone.x = 1;
    test_stone.y = 1;
    test_stone.color = Color::Blue;

    println!("{:?}", test_stone);

    let mut field: Vec<Vec<Color>> = /* so much for 80 columns */
        vec![vec![Color::Blue,  Color::Invis, Color::Invis, Color::Invis, Color::Invis, Color::Invis, Color::Orange,Color::Invis, Color::Invis, Color::Invis, Color::Invis ],
             vec![Color::Invis, Color::Invis, Color::Invis, Color::Invis, Color::Invis, Color::Invis, Color::Invis, Color::Invis, Color::Invis, Color::Invis, Color::Invis ],
             vec![Color::Invis, Color::Invis, Color::Red,   Color::Invis, Color::Invis, Color::Invis, Color::Invis, Color::Invis, Color::Green, Color::Invis, Color::Invis ],
             vec![Color::Invis, Color::Invis, Color::Invis, Color::Invis, Color::Invis, Color::Invis, Color::Invis, Color::Invis, Color::Invis, Color::Invis, Color::Invis ],
             vec![Color::Invis, Color::Invis, Color::Invis, Color::Invis, Color::Yellow,Color::Invis, Color::Invis, Color::Invis, Color::Invis, Color::Invis, Color::Purple],
             vec![Color::Invis, Color::Invis, Color::Invis, Color::Invis, Color::Invis, Color::Invis, Color::Invis, Color::Invis, Color::Invis, Color::Invis, Color::Invis ]];

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

