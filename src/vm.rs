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

impl From<usize> for RedNumber {
    fn from(i: usize) -> Self {
        match i {
            1 => RedNumber::One,
            2 => RedNumber::Two,
            3 => RedNumber::Three,
            _ => unreachable!(),
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

impl From<usize> for OrangeNumber {
    fn from(i: usize) -> Self {
        match i {
            1 => OrangeNumber::One,
            2 => OrangeNumber::Two,
            _ => unreachable!(),
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

impl std::fmt::Display for Op {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?} {:?}", self.color, self.dir)
    }
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

    pub fn change_magnitude(self, new_magnitude: usize) -> Op {
        match self.color {
            OpColor::Red(_) => Op {
                color: OpColor::Red(new_magnitude.into()),
                ..self
            },
            OpColor::Orange(_) => Op {
                color: OpColor::Orange(new_magnitude.into()),
                ..self
            },
            OpColor::Yellow | OpColor::Blue | OpColor::Green | OpColor::Purple => self,
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
            Stone::Orange => OpColor::Orange(OrangeNumber::One),
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
        self.field[row][col] = color;
    }

    fn get(&mut self, row: usize, col: usize) -> Stone {
        self.field[row][col]
    }

    pub const fn width(&self) -> usize {
        Width
    }

    pub const fn height(&self) -> usize {
        Height
    }

    fn next_row(row: usize, dir: Dir) -> usize {
        if dir == Dir::Up {
            if row == 0 {
                Height - 1
            } else {
                row - 1
            }
        } else if dir == Dir::Down {
            if row + 1 == Height {
                0
            } else {
                row + 1
            }
        } else {
            row
        }
    }

    fn next_col(col: usize, dir: Dir) -> usize {
        if dir == Dir::Left {
            if col == 0 {
                Width - 1
            } else {
                col - 1
            }
        } else if dir == Dir::Right {
            if col + 1 == Width {
                0
            } else {
                col + 1
            }
        } else {
            col
        }
    }

    pub fn step(&mut self, vm: &mut Vm, mut op: Op) {
        let (row_idx, col_idx) = self.find(op.color());
        let mag = op.magnitude();
        println!("****** begin");
        println!("called step {mag}x{op}@{row_idx},{col_idx}");
        println!("{self:?}");
        println!("******");

        let (mut current_row, mut current_col) = (row_idx, col_idx);
        for steps_taken in 1..=mag {
            let next_row = Self::next_row(current_row, op.dir);
            let next_col = Self::next_col(current_col, op.dir);
            let next = self.get(next_row, next_col);
            //println!(
            //    "  {:?}{steps_taken} steps to {next_row},{next_col}={next:?}",
            //    op.color()
            //);
            println!("next is {next:?}@{next_row},{next_col}");

            if next != Stone::X && next < op.color() {
                //println!("    pushing out of the way");
                let next_op = Op {
                    color: next.to_op(),
                    dir: op.dir,
                };

                println!("call step({next_op:?}) because {next:?} in way while stepping {op:?}",);
                println!("###### recur");
                println!("{self:?}");
                self.step(vm, next_op);
                println!("{self:?}");
                println!("###### end recur (moving {next:?}) to make way for {op:?}",);
            } else if next != Stone::X && next > op.color() {
                //println!("    too heavy");
                println!("{next:?} is heaver than {op:?}");
                break;
            } else {
                //println!("    unoccupied");
            }

            self.set(Stone::X, current_row, current_col);
            self.set(op.color(), next_row, next_col);
            println!("set {next_row},{next_col} to {:?}", op.color());

            current_row = next_row;
            current_col = next_col;
        }

        vm.exec(op);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    macro_rules! oplist {
        ($(($color:expr, $dir:ident),)*) => {
            vec![$(Op {
                color: $color,
                dir: Dir::$dir,
            },)*]
        };
    }

    macro_rules! mktest {
        (($color:expr, $dir:ident), $field:expr, $expfield:expr, $expvm:expr) => {
            let op = Op {
                color: $color,
                dir: Dir::$dir,
            };
            let mut vm = Vm::default();
            let mut field = Field { field: $field };

            println!("before\n{field:?}");
            field.step(&mut vm, op);
            println!("after\n{field:?}");

            println!("vm: {vm:?}");

            assert_eq!($expfield, field.field);
            assert_eq!($expvm, vm.history);
        };
    }

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
    fn simple() {
        mktest!(
            (OpColor::Blue, Right),
            [[Stone::Blue, Stone::X]],
            [[Stone::X, Stone::Blue]],
            oplist!((OpColor::Blue, Right),)
        );
    }

    #[test]
    fn non_interfere_left() {}
    #[test]
    fn non_interfere_right() {}
    #[test]
    fn non_interfere_up() {}
    #[test]
    fn non_interfere_down() {}
    #[test]
    fn non_interfere_left_wrap() {}
    #[test]
    fn non_interfere_right_wrap() {}
    #[test]
    fn non_interfere_up_wrap() {}
    #[test]
    fn non_interfere_down_wrap() {}

    #[test]
    fn left_wrap() {
        mktest!(
            (OpColor::Green, Left),
            [[Stone::Green, Stone::X, Stone::X]],
            [[Stone::X, Stone::X, Stone::Green]],
            oplist!((OpColor::Green, Left),)
        );
    }

    #[test]
    fn right_wrap() {
        mktest!(
            (OpColor::Green, Right),
            [[Stone::X, Stone::X, Stone::Green]],
            [[Stone::Green, Stone::X, Stone::X]],
            oplist!((OpColor::Green, Right),)
        );
    }

    #[test]
    fn up_wrap() {
        mktest!(
            (OpColor::Green, Up),
            [
                [Stone::X, Stone::Green],
                [Stone::X, Stone::X],
                [Stone::X, Stone::X],
            ],
            [
                [Stone::X, Stone::X],
                [Stone::X, Stone::X],
                [Stone::X, Stone::Green]
            ],
            oplist!((OpColor::Green, Up),)
        );
    }

    #[test]
    fn down_wrap() {
        mktest!(
            (OpColor::Green, Down),
            [
                [Stone::X, Stone::X],
                [Stone::X, Stone::X],
                [Stone::X, Stone::Green],
            ],
            [
                [Stone::X, Stone::Green],
                [Stone::X, Stone::X],
                [Stone::X, Stone::X],
            ],
            oplist!((OpColor::Green, Down),)
        );
    }

    #[test]
    fn push_left() {
        mktest!(
            (OpColor::Green, Left),
            [[Stone::X, Stone::Yellow, Stone::Green]],
            [[Stone::Yellow, Stone::Green, Stone::X]],
            oplist!((OpColor::Yellow, Left), (OpColor::Green, Left),)
        );
    }

    #[test]
    fn push_right() {
        mktest!(
            (OpColor::Green, Right),
            [[Stone::Green, Stone::Yellow, Stone::X]],
            [[Stone::X, Stone::Green, Stone::Yellow]],
            oplist!((OpColor::Yellow, Right), (OpColor::Green, Right),)
        );
    }

    #[test]
    fn push_up() {
        mktest!(
            (OpColor::Green, Up),
            [
                [Stone::X, Stone::X],
                [Stone::Yellow, Stone::X],
                [Stone::Green, Stone::X],
            ],
            [
                [Stone::Yellow, Stone::X],
                [Stone::Green, Stone::X],
                [Stone::X, Stone::X]
            ],
            oplist!((OpColor::Yellow, Up), (OpColor::Green, Up),)
        );
    }

    #[test]
    fn push_down() {
        mktest!(
            (OpColor::Green, Down),
            [
                [Stone::Green, Stone::X],
                [Stone::Yellow, Stone::X],
                [Stone::X, Stone::X],
            ],
            [
                [Stone::X, Stone::X],
                [Stone::Green, Stone::X],
                [Stone::Yellow, Stone::X],
            ],
            oplist!((OpColor::Yellow, Down), (OpColor::Green, Down),)
        );
    }

    #[test]
    fn push_left_wrap() {
        mktest!(
            (OpColor::Green, Left),
            [[Stone::Yellow, Stone::Green, Stone::X]],
            [[Stone::Green, Stone::X, Stone::Yellow]],
            oplist!((OpColor::Yellow, Left), (OpColor::Green, Left),)
        );
    }

    #[test]
    fn push_right_wrap() {
        mktest!(
            (OpColor::Green, Right),
            [[Stone::X, Stone::Green, Stone::Yellow]],
            [[Stone::Yellow, Stone::X, Stone::Green]],
            oplist!((OpColor::Yellow, Right), (OpColor::Green, Right),)
        );
    }

    #[test]
    fn push_up_wrap() {
        mktest!(
            (OpColor::Green, Up),
            [
                [Stone::Yellow, Stone::X],
                [Stone::Green, Stone::X],
                [Stone::X, Stone::X],
            ],
            [
                [Stone::Green, Stone::X],
                [Stone::X, Stone::X],
                [Stone::Yellow, Stone::X],
            ],
            oplist!((OpColor::Yellow, Up), (OpColor::Green, Up),)
        );
    }

    #[test]
    fn push_down_wrap() {
        mktest!(
            (OpColor::Green, Down),
            [
                [Stone::X, Stone::X],
                [Stone::Green, Stone::X],
                [Stone::Yellow, Stone::X],
            ],
            [
                [Stone::Yellow, Stone::X],
                [Stone::X, Stone::X],
                [Stone::Green, Stone::X],
            ],
            oplist!((OpColor::Yellow, Down), (OpColor::Green, Down),)
        );
    }

    #[test]
    fn double_push_left() {
        mktest!(
            (OpColor::Orange(OrangeNumber::Two), Right),
            [[Stone::Orange, Stone::Red, Stone::X, Stone::X]],
            [[Stone::X, Stone::X, Stone::Orange, Stone::Red]],
            oplist!(
                (OpColor::Red(RedNumber::One), Right),
                (OpColor::Red(RedNumber::One), Right),
                (OpColor::Orange(OrangeNumber::Two), Right),
            )
        );
    }

    #[test]
    fn double_push_right() {
        mktest!(
            (OpColor::Orange(OrangeNumber::Two), Right),
            [[Stone::Orange, Stone::Red, Stone::X, Stone::X]],
            [[Stone::X, Stone::X, Stone::Orange, Stone::Red]],
            oplist!(
                (OpColor::Red(RedNumber::One), Right),
                (OpColor::Red(RedNumber::One), Right),
                (OpColor::Orange(OrangeNumber::Two), Right),
            )
        );
    }

    #[test]
    fn choo_choo() {
        mktest!(
            (OpColor::Purple, Left),
            [[
                Stone::X,
                Stone::Red,
                Stone::Orange,
                Stone::Yellow,
                Stone::Green,
                Stone::Blue,
                Stone::Purple,
            ]],
            [[
                Stone::Red,
                Stone::Orange,
                Stone::Yellow,
                Stone::Green,
                Stone::Blue,
                Stone::Purple,
                Stone::X,
            ]],
            oplist!(
                (OpColor::Red(RedNumber::One), Left),
                (OpColor::Orange(OrangeNumber::One), Left),
                (OpColor::Yellow, Left),
                (OpColor::Green, Left),
                (OpColor::Blue, Left),
                (OpColor::Purple, Left),
            )
        );
    }
}
