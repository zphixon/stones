/* Stones esoteric programming language
 * (c) 2016 Zack Hixon - see LICENSE.txt */

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

