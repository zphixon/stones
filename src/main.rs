use gumdrop::Options;

#[derive(Debug, Options)]
struct Args {
    #[options(help = "Print this message.", short = "h")]
    help: bool,

    #[options(free, help = "File to run. If no file is given, opens a REPL.")]
    filename: Option<String>,

    #[options(
        help = "If no filename is present, show the tokens scanned from each line of REPL input, or if given a filename show the tokens scanned from the file and exit.",
        short = "t"
    )]
    print_tokens: bool,

    #[options(
        help = "If no filename is present, show the AST parsed from each line of REPL input, or if given a filename show the AST parsed from the file and exit.",
        short = "a"
    )]
    print_ast: bool,

    #[options(
        help = "If no filename is present, show the operations compiled from the AST from each line of REPL input, or if given a filename show the operations compiled from the file and exit.",
        short = "c"
    )]
    print_compiled: bool,

    #[options(help = "Quit after parsing and compiling a file.", short = "v")]
    verify_syntax: bool,

    #[options(help = "Print the operation being executed.", short = "o")]
    print_operation: bool,

    #[options(
        help = "Display the field after every operation. Implies --operation.",
        short = "f"
    )]
    print_field: bool,

    #[options(
        help = "Display the stack after every operation. Implies --operation.",
        short = "s"
    )]
    print_stack: bool,

    #[options(
        help = "Continue an interactive session in the repl, overriding the exit behavior of --print-tokens and --print-ast.",
        short = "i"
    )]
    interactive: bool,
}

fn main() {
    let mut args: Args = Args::parse_args_default_or_exit();

    // TODO handle more combinations
    args.print_operation = args.print_operation || args.print_field || args.print_stack;

    if args.filename.is_none() || args.interactive {
        todo!();
    }

    let source = std::fs::read_to_string(args.filename.as_ref().unwrap()).unwrap();
    if args.print_tokens {
        println!("{:#?}", stones::scan(&source).collect::<Vec<_>>());
    }

    let ast = stones::parse(&source).unwrap();
    if args.print_ast {
        println!("{ast:#?}");
    }

    let program = stones::compile(&ast);
    if args.print_compiled {
        println!("{program:#?}");
    }

    if args.filename.is_some() && args.verify_syntax {
        return;
    }

    let mut vm = stones::vm::Vm::new(program);

    if args.print_field {
        println!("init\n{:?}\n", vm.field());
    }

    vm.run(args.print_operation, args.print_field, args.print_stack)
        .unwrap();
}
