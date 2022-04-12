use gumdrop::Options;

#[derive(Debug, Options)]
struct Args {
    #[options(help = "Print this message.", short = "h")]
    help: bool,

    #[options(free, help = "File to run. If no file is given, opens a REPL.")]
    filename: Option<String>,

    #[options(
        help = "If no filename is present, show the tokens scanned from the file or each line of REPL input.",
        short = "t"
    )]
    print_tokens: bool,

    #[options(
        help = "If no filename is present, show the AST parsed from the file or each line of REPL input.",
        short = "a"
    )]
    print_ast: bool,

    #[options(
        help = "Show compiled bytecode from the file or each line of REPL input.",
        short = "c"
    )]
    print_compiled: bool,

    #[options(
        help = "Quit after parsing and compiling a file. Requires filename.",
        short = "v"
    )]
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

impl Args {
    fn print_any(&self) -> bool {
        self.print_tokens
            || self.print_ast
            || self.print_compiled
            || self.print_operation
            || self.print_field
            || self.print_stack
    }
}

fn main() {
    let mut args: Args = Args::parse_args_default_or_exit();
    args.print_operation = args.print_operation || args.print_field || args.print_stack;

    if args.filename.is_none() || args.interactive {
        todo!("repl not supported yet");
    }

    let source = std::fs::read_to_string(args.filename.as_ref().unwrap()).unwrap();
    let ast = stones::parse(&source).unwrap();
    let program = stones::compile(&ast);

    if args.print_tokens {
        println!("tokens:\n{:#?}", stones::scan(&source).collect::<Vec<_>>());
    }
    if args.print_ast {
        println!("ast:\n{ast:#?}");
    }
    if args.print_compiled {
        println!("bytecode:");
        for (i, stones::vm::Operation { command, opcode }) in program.iter().enumerate() {
            stones::print_command_opcode(i, command, *opcode);
        }
    }

    if args.filename.is_some() && args.verify_syntax {
        return;
    }

    let mut vm = stones::vm::Vm::new(program);

    if args.print_any() {
        println!("program run:");
    }

    if args.print_field {
        println!("init\n{:?}\n", vm.field());
    }

    let mut result = vm.run(args.print_operation, args.print_field, args.print_stack);
    while matches!(result, Err(stones::Error::Quine)) {
        print!("{source}");
        result = vm.run(args.print_operation, args.print_field, args.print_stack);
    }
    result.unwrap();
}
