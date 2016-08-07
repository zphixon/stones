/* Stones esoteric programming language
 * (c) 2016 Zack Hixon - see LICENSE.txt */

#[derive(Debug)]
struct Stone {
    x: i32,
    y: i32,
    id: i32,
    color: String
}

fn main() {
    let mut stone = Stone {
        x: 0,
        y: 0,
        id: 1,
        color: String::from("yellow")
    };

    println!("{:?}", stone);

    stone.x = 1;
    stone.y = 1;
    stone.id = 2;
    stone.color = String::from("blue");

    println!("{:?}", stone);
}

