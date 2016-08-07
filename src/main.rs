/* Stones esoteric programming language
 * (c) 2016 Zack Hixon - see LICENSE.txt */

#[derive(Debug)]
struct Stone {
    id: i32,
    color: String
}

fn main() {
    let mut stone = Stone {id: 1, color: String::from("yellow")};
    println!("{:?}", stone);

    stone.id = 2;
    stone.color = String::from("blue");

    println!("{:?}", stone);
}

