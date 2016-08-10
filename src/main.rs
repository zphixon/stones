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
            color: Color::Invisible
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
    Invisible
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

    let mut field: Vec<Vec<Stone>> =
        vec![vec![Stone::new(0, 0, Color::Red),
                  Stone::new(1, 0, Color::Blue),
                  Stone::new(2, 0, Color::Green)],
             vec![Stone::empty(),
                  Stone::empty(),
                  Stone::empty()],
             vec![Stone::empty(),
                  Stone::empty(),
                  Stone::empty()]];

    println!("\nall stones:");
    for row in field {
        for stone in row {
            print!("{} {} {:?}\n", stone.x, stone.y, stone.color);
        }
        println!("");
    }
}

