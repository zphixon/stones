/* Stones esoteric programming language
 * (c) 2016 Zack Hixon - see LICENSE.txt */

#[derive(Debug)]
//#[derive(Copy, Clone)]
struct Stone {
    x: i32,
    y: i32,
    id: i32
}

fn main() {
    let mut stone = Stone {x: 32, y: 32, id: 1};
    println!("{:?}", stone);

    stone.x = 64;
    stone.y = 64;
    stone.id = 2;

    println!("{:?}", stone);
}

