fn main() {
    let src = include_str!("../examples/test.stn");
    println!("{src}");

    let tokens = stones::scan(src).collect::<Vec<_>>();
    println!("{tokens:?}");

    let ast = stones::parse(src).unwrap();
    println!("{ast:?}");
}
