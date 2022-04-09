use gumdrop::Options;

#[derive(Debug, Options)]
struct Args {
    #[options(help = "Print this message.")]
    help: bool,

    #[options(free, help = "File to run. If no file is given, opens a REPL.")]
    filename: Option<String>,

    #[options(
        help = "If no filename is present, show the tokens scanned from each line of REPL input, or if given a filename show the tokens scanned from the file and exit."
    )]
    print_tokens: bool,

    #[options(
        help = "If no filename is present, show the AST parsed from each line of REPL input, or if given a filename show the AST parsed from the file and exit."
    )]
    print_ast: bool,

    #[options(help = "Print the operation being executed.")]
    operation: bool,

    #[options(help = "Display the field after every operation. Implies --operation.")]
    field: bool,

    #[options(help = "Display the stack after every operation. Implies --operation.")]
    stack: bool,

    #[options(
        help = "Continue an interactive session in the repl, overriding the exit behavior of --print-tokens and --print-ast."
    )]
    interactive: bool,
}

fn main() {
    let mut args: Args = Args::parse_args_default_or_exit();
    args.operation = args.operation || args.field || args.stack;

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

    if args.filename.is_some() && (args.print_tokens || args.print_ast) {
        return;
    }

    //let mut vm = stones::vm::Vm::new(ast);
    let mut field = stones::field::Field::new();
    let mut step = 0;

    if args.field {
        println!("init\n{field:?}\n");
    }

    // should be like
    // vm.step(mut field, op) {
    //   necessary_ops = field.step(op) {
    //     let ops = mut vec
    //     steprec(ops)
    //     return ops
    //   }
    //   for op inn necops {
    //     self.exec(op)
    //   }
    // }

    //for op in ast {
    //    //let err = field.step(&mut vm, op, args.operation);
    //    if matches!(err, Err(stones::Error::Quine)) {
    //        println!("{source}");
    //    } else {
    //        err.unwrap();
    //    }

    //    step += 1;
    //    if args.field {
    //        println!("{step}\n{field:?}");
    //    }
    //    if args.stack {
    //        println!("{vm:?}");
    //    }
    //}
}
