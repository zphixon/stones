use crate::{command::Command, field::Field, Error, Value};

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Comparison {
    Equal,
    LessThan,
    GreaterThan,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Math {
    Multiply,
    Add,
    Subtract,
    Divide,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Opcode {
    PushNumber(i64),
    PushBool(bool),
    StartArray,
    PushArray,
    EndArray,
    NthArray,
    Comparison(Comparison),
    Quine,
    Math(Math),
    Roll,
    Dup,
    Drop,
    Not,
    Print,
    Input,
    Printc,
    Swap,
    JumpFalse(usize),    // always forward: head of if/while
    JumpForward(usize),  // else command of if
    JumpBackward(usize), // end of while
    Die,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Operation {
    pub command: Command,
    pub opcode: Opcode,
}

impl Operation {
    pub fn die() -> Operation {
        Operation {
            command: Command::empty(),
            opcode: Opcode::Die,
        }
    }

    pub fn from(command: Command) -> Operation {
        Operation {
            opcode: command.get_opcode().unwrap(),
            command,
        }
    }
}

#[derive(Debug)]
pub struct Vm {
    stack: Vec<Value>,
    program: Vec<Operation>,
    ip: usize,
    array_in_progress: Option<Vec<Value>>,
    field: Field<12, 6>,
}

impl Vm {
    pub fn new(program: Vec<Operation>) -> Vm {
        Vm {
            stack: Vec::new(),
            program,
            ip: 0,
            array_in_progress: None,
            field: Field::new(),
        }
    }

    fn push(&mut self, value: Value) {
        self.stack.push(value);
    }

    fn pop(&mut self) -> Result<Value, Error> {
        self.stack.pop().ok_or(Error::StackUnderflow)
    }

    fn peek(&mut self, depth: usize) -> Result<&Value, Error> {
        self.stack
            .get(self.stack.len() - depth - 1)
            .ok_or(Error::StackUnderflow)
    }

    pub fn run(
        &mut self,
        print_op: bool,
        print_field: bool,
        print_stack: bool,
    ) -> Result<(), Error> {
        loop {
            if self.ip >= self.program.len() {
                break;
            }

            let operation = self.program[self.ip];
            self.ip += 1;
            let Operation { command, opcode } = operation;

            let commands = self.field.commands_for(command, print_op);
            for command in commands {
                let opcode = command.get_opcode().unwrap_or(opcode);
                if print_op {
                    crate::print_command_opcode(self.ip, &command, opcode);
                }

                match opcode {
                    Opcode::PushNumber(num) => self.push(Value::Num(num)),

                    Opcode::PushBool(bool) => self.push(Value::Bool(bool)),

                    Opcode::StartArray => self.array_in_progress = Some(Vec::new()),

                    Opcode::PushArray => {
                        let arr = self
                            .array_in_progress
                            .take()
                            .ok_or_else(|| todo!())
                            .unwrap();
                        self.push(Value::Arr(arr));
                    }

                    Opcode::EndArray => {
                        let in_progress = self.array_in_progress.take();
                        if let Some(mut in_progress) = in_progress {
                            in_progress.push(self.pop()?);
                            self.array_in_progress = Some(in_progress);
                        } else {
                            todo!();
                        }
                    }

                    Opcode::NthArray => {
                        let idx: i64 = self.pop()?.try_into()?;
                        let maybe_arr = self.peek(0)?;
                        let arr: &[Value] = maybe_arr.get_slice().ok_or(Error::TypeMismatch {
                            wanted: "array",
                            got: maybe_arr.type_name(),
                        })?;
                        let dup = arr[idx as usize].clone();
                        self.push(dup);
                    }

                    Opcode::Comparison(comparison) => {
                        let lhs = self.pop()?;
                        let rhs = self.pop()?;
                        self.push(Value::Bool(match comparison {
                            Comparison::Equal => lhs == rhs,
                            Comparison::LessThan => !(lhs < rhs) && lhs != rhs,
                            Comparison::GreaterThan => !(lhs > rhs) && lhs != rhs,
                        }));
                    }

                    Opcode::Quine => Err(Error::Quine)?,

                    Opcode::Math(math) => {
                        let lhs: i64 = self.pop()?.try_into()?;
                        let rhs: i64 = self.pop()?.try_into()?;
                        self.push(Value::Num(match math {
                            Math::Multiply => lhs * rhs,
                            Math::Add => lhs + rhs,
                            Math::Subtract => lhs - rhs,
                            Math::Divide => lhs / rhs,
                        }));
                    }

                    Opcode::Roll => {
                        let d: i64 = self.pop()?.try_into()?;
                        if d > 0 {
                            let mut to_roll = Vec::new();
                            for _ in 0..d + 1 {
                                to_roll.push(self.pop()?);
                            }
                            to_roll.reverse();
                            let top = to_roll.pop().unwrap();
                            to_roll.insert(0, top);
                            for elem in to_roll {
                                self.push(elem);
                            }
                        }
                    }

                    Opcode::Dup => {
                        let dup = self.pop()?;
                        self.push(dup.clone());
                        self.push(dup);
                    }

                    Opcode::Drop => {
                        let _ = self.pop()?;
                    }

                    Opcode::Not => {
                        let bool = self.pop()?.is_truthy();
                        self.push(Value::Bool(!bool));
                    }

                    Opcode::Print => self.pop()?.print_as_num(),

                    Opcode::Input => {
                        let mut line = String::new();
                        std::io::stdin().read_line(&mut line)?;
                        let line = line.trim();
                        if let Ok(num) = line.parse() {
                            self.push(Value::Num(num));
                        } else if let Ok(bool) = line.parse() {
                            self.push(Value::Bool(bool));
                        } else {
                            todo!();
                        }
                    }

                    Opcode::Printc => self.pop()?.print_as_char(),

                    Opcode::Swap => {
                        let a = self.pop()?;
                        let b = self.pop()?;
                        self.push(a);
                        self.push(b);
                    }

                    // always forward: head of if/while
                    Opcode::JumpFalse(offset) => {
                        if !self.pop()?.is_truthy() {
                            self.ip = offset + 1;
                        }
                    }

                    // else command of if
                    Opcode::JumpForward(offset) => {
                        self.ip = offset;
                    }

                    // end of while
                    Opcode::JumpBackward(offset) => {
                        self.ip = offset;
                    }

                    Opcode::Die => unreachable!(),
                }

                if print_field {
                    println!("{:?}", self.field);
                }

                if print_stack {
                    for value in &self.stack {
                        println!("{value:?}");
                    }
                }
            }
        }

        Ok(())
    }

    pub fn field(&self) -> &Field {
        &self.field
    }
}
