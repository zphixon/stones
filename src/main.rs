fn main() {
    let src = include_str!("../examples/test.stn");
    println!("{src}");

    let tokens = stones::scan(src).collect::<Vec<_>>();
    println!("{tokens:?}");

    let ast = stones::parse(src).unwrap();
    println!("{ast:?}");

    let mut vm = stones::vm::Vm::default();
    let mut field = stones::field::Field::new();
    let mut step = 0;

    println!("init\n{field:?}\n");
    for op in ast {
        let err = field.step(&mut vm, op);
        if matches!(err, Err(stones::Error::Quine)) {
            println!("{src}");
        } else {
            err.unwrap();
        }

        step += 1;
        println!("{step}\n{field:?}\n{vm:?}\n\n");
    }
}
