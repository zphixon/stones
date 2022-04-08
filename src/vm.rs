#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RedNumber {
    One,
    Two,
    Three,
}

impl RedNumber {
    fn magnitude(&self) -> usize {
        match self {
            RedNumber::One => 1,
            RedNumber::Two => 2,
            RedNumber::Three => 3,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OrangeNumber {
    One,
    Two,
}

impl OrangeNumber {
    fn magnitude(&self) -> usize {
        match self {
            OrangeNumber::One => 1,
            OrangeNumber::Two => 2,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OpColor {
    Red(RedNumber),
    Orange(OrangeNumber),
    Yellow,
    Green,
    Blue,
    Purple,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Dir {
    Left,
    Right,
    Up,
    Down,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Op {
    color: OpColor,
    dir: Dir,
}

impl Op {
    pub fn color(&self) -> Stone {
        match self.color {
            OpColor::Red(_) => Stone::Red,
            OpColor::Orange(_) => Stone::Orange,
            OpColor::Yellow => Stone::Yellow,
            OpColor::Blue => Stone::Blue,
            OpColor::Green => Stone::Green,
            OpColor::Purple => Stone::Purple,
        }
    }

    pub fn magnitude(&self) -> usize {
        match self.color {
            OpColor::Red(step) => step.magnitude(),
            OpColor::Orange(step) => step.magnitude(),
            OpColor::Yellow | OpColor::Blue | OpColor::Green | OpColor::Purple => 1,
        }
    }
}

#[derive(Default, Debug)]
pub struct Vm {
    pub history: Vec<Op>,
}

impl Vm {
    fn exec(&mut self, op: Op) {
        println!("exec {op:?}");
        self.history.push(op);
        match op.color() {
            Stone::Red => {}
            Stone::Orange => {}
            Stone::Yellow => {}
            Stone::Blue => {}
            Stone::Green => {}
            Stone::Purple => {}

            _ => unreachable!(),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub enum Stone {
    X,
    Red,
    Orange,
    Yellow,
    Green,
    Blue,
    Purple,
}

impl Stone {
    pub fn to_op(&self) -> OpColor {
        match self {
            Stone::Red => OpColor::Red(RedNumber::One),
            Stone::Orange => OpColor::Orange(OrangeNumber::Two),
            Stone::Yellow => OpColor::Yellow,
            Stone::Blue => OpColor::Blue,
            Stone::Green => OpColor::Green,
            Stone::Purple => OpColor::Purple,

            _ => unreachable!(),
        }
    }
}

pub struct Field<const W: usize = 12, const H: usize = 6> {
    field: [[Stone; W]; H],
}

impl<const W: usize, const H: usize> std::fmt::Debug for Field<W, H> {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        // ┌┐└┘─│
        writeln!(formatter, "┌{:─<width$}┐", "─", width = W * 10)?;
        for row in self.field {
            write!(formatter, "│")?;
            for col in row {
                write!(formatter, "{:10}", format!("{col:?}"))?;
            }
            writeln!(formatter, "│")?;
        }
        writeln!(formatter, "└{:─<width$}┘", "─", width = W * 10)
    }
}

pub struct Wrap {
    left: bool,
    right: bool,
    up: bool,
    down: bool,
}

impl Wrap {
    pub fn left(&self) -> bool {
        self.left && !self.right && !self.up && !self.down
    }

    pub fn right(&self) -> bool {
        !self.left && self.right && !self.up && !self.down
    }

    pub fn up(&self) -> bool {
        !self.left && !self.right && self.up && !self.down
    }

    pub fn down(&self) -> bool {
        !self.left && !self.right && !self.up && self.down
    }
}

pub struct NewPosition {
    pub wrap: Wrap,
    pub new_row: usize,
    pub new_col: usize,
}

impl Field<12, 6> {
    #[rustfmt::skip]
    pub fn new() -> Field {
        use Stone::*;

        Field {
            field: [
                [Blue, X, X,   X, X,      X, Orange, X, X,     X, X,      X],
                [X,    X, X,   X, X,      X, X,      X, X,     X, X,      X],
                [X,    X, Red, X, X,      X, X,      X, Green, X, X,      X],
                [X,    X, X,   X, X,      X, X,      X, X,     X, X,      X],
                [X,    X, X,   X, Yellow, X, X,      X, X,     X, Purple, X],
                [X,    X, X,   X, X,      X, X,      X, X,     X, X,      X],
            ],
        }
    }
}

#[allow(non_upper_case_globals)]
impl<const Width: usize, const Height: usize> Field<Width, Height> {
    fn find(&self, color: Stone) -> (usize, usize) {
        for (row_idx, row) in self.field.iter().enumerate() {
            for (col_idx, my_color) in row.iter().enumerate() {
                if *my_color == color {
                    return (row_idx, col_idx);
                }
            }
        }

        unreachable!()
    }

    fn set(&mut self, color: Stone, row: usize, col: usize) {
        self.field[row % Height][col % Width] = color;
    }

    fn get(&mut self, row: usize, col: usize) -> Stone {
        self.field[row % Height][col % Width]
    }

    pub const fn width(&self) -> usize {
        Width
    }

    pub const fn height(&self) -> usize {
        Height
    }

    pub fn step(&mut self, vm: &mut Vm, op: Op) {
        let (row_idx, col_idx) = self.find(op.color());
        let mag = op.magnitude();
        println!("step {op:?}@{row_idx},{col_idx}");

        for _ in 1..=mag {
            let new_col = if op.dir == Dir::Left {
                if col_idx == 0 {
                    Width - 1
                } else {
                    col_idx - 1
                }
            } else if op.dir == Dir::Right {
                col_idx + 1
            } else {
                col_idx
            };

            let new_row = if op.dir == Dir::Up {
                if row_idx == 0 {
                    Height - 1
                } else {
                    row_idx - 1
                }
            } else if op.dir == Dir::Down {
                row_idx + 1
            } else {
                row_idx
            };
            println!("  -> {new_row},{new_col}");

            let there = self.get(new_row, new_col);

            if there != Stone::X {
                if there < op.color() {
                    println!("  collide with {there:?}, power through");
                    self.step(
                        vm,
                        Op {
                            color: there.to_op(),
                            dir: op.dir,
                        },
                    );
                } else {
                    println!("  collide with {there:?}, too weak");
                    break;
                }
            }

            self.set(op.color(), new_row, new_col);
            self.set(Stone::X, row_idx, col_idx);
        }

        vm.exec(op);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn cmp() {
        assert!(Stone::X < Stone::Red);
        assert!(Stone::Red < Stone::Orange);
        assert!(Stone::Orange < Stone::Yellow);
        assert!(Stone::Yellow < Stone::Green);
        assert!(Stone::Green < Stone::Blue);
        assert!(Stone::Blue < Stone::Purple);
    }

    #[test]
    fn regular_step() {
        let op = Op {
            color: OpColor::Red(RedNumber::One),
            dir: Dir::Right,
        };
        let mut vm = Vm::default();
        let mut field = Field {
            field: [[Stone::Red, Stone::X]],
        };
        field.step(&mut vm, op);

        assert_eq!(field.field, [[Stone::X, Stone::Red]]);
    }

    #[test]
    fn wrap_left() {
        let op = Op {
            color: OpColor::Red(RedNumber::One),
            dir: Dir::Left,
        };
        let mut vm = Vm::default();
        let mut field = Field {
            field: [[Stone::Red, Stone::X]],
        };
        field.step(&mut vm, op);

        assert_eq!(field.field, [[Stone::X, Stone::Red]]);
    }

    #[test]
    fn wrap_right() {
        let op = Op {
            color: OpColor::Red(RedNumber::Two),
            dir: Dir::Left,
        };
        let mut vm = Vm::default();
        let mut field = Field {
            field: [[Stone::Red, Stone::X]],
        };
        field.step(&mut vm, op);

        assert_eq!(field.field, [[Stone::Red, Stone::X]]);
    }

    #[test]
    fn step2() {
        let op = Op {
            color: OpColor::Blue,
            dir: Dir::Right,
        };
        let mut vm = Vm::default();
        let mut field = Field {
            field: [[Stone::Blue, Stone::Red, Stone::X]],
        };
        field.step(&mut vm, op);

        assert_eq!(field.field, [[Stone::X, Stone::Blue, Stone::Red]]);
        assert_eq!(
            vm.history,
            vec![
                Op {
                    color: OpColor::Red(RedNumber::One),
                    dir: Dir::Right,
                },
                Op {
                    color: OpColor::Blue,
                    dir: Dir::Right,
                }
            ]
        );
    }

    #[test]
    fn step3() {
        let op = Op {
            color: OpColor::Red(RedNumber::Two),
            dir: Dir::Left,
        };
        let mut vm = Vm::default();
        let mut field = Field {
            field: [[Stone::Red, Stone::X, Stone::X, Stone::Blue, Stone::X]],
        };
        println!("{field:?}");
        field.step(&mut vm, op);
        println!("{field:?}");

        panic!("{vm:?}");
    }
}
