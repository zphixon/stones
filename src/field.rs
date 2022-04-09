use crate::vm::{Dir, Op, OpColor, OrangeNumber, RedNumber};
use crate::{Error, Token};

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

            // cannot get purple, it is the heaviest
            // cannot get X, it has no op
            _ => unreachable!(),
        }
    }

    pub fn has_number(&self) -> bool {
        match self {
            Stone::Red | Stone::Orange => true,
            _ => false,
        }
    }
}

impl TryFrom<Token> for Stone {
    type Error = Error;
    fn try_from(value: Token) -> Result<Stone, Self::Error> {
        match value {
            Token::Red => Ok(Stone::Red),
            Token::Orange => Ok(Stone::Orange),
            Token::Yellow => Ok(Stone::Yellow),
            Token::Green => Ok(Stone::Green),
            Token::Blue => Ok(Stone::Blue),
            Token::Purple => Ok(Stone::Purple),
            _ => Err(Error::ExpectedColor { got: value }),
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

    pub fn ops_for(&mut self, op: Op, print_op: bool) -> Vec<Op> {
        let mut ops = Vec::new();
        self.step_rec(&mut ops, op, print_op);
        ops
    }

    // returns whether the move was blocked
    fn step_rec(&mut self, ops: &mut Vec<Op>, op: Op, print_op: bool) -> bool {
        let (row_idx, col_idx) = self.find(op.color());
        let (mut current_row, mut current_col) = (row_idx, col_idx);

        let mag = op.magnitude();
        let mut steps_taken = 0;
        for _ in 1..=mag {
            let next_row = Self::next_row(current_row, op.dir);
            let next_col = Self::next_col(current_col, op.dir);
            let next = self.get(next_row, next_col);

            if next > op.color() {
                // next is heavier, quit early
                break;
            } else if next == Stone::X {
                // next is empty, just move it
            } else if next != Stone::X && next < op.color() {
                // next is lighter
                let next_op = Op {
                    color: next.to_op(),
                    dir: op.dir,
                    side_effect: true,
                };
                let blocked = self.step_rec(ops, next_op, print_op);
                if blocked {
                    break;
                }
                if print_op {
                    println!("pushed {op:?} > {next:?}");
                }
            } else {
                unreachable!()
            }

            self.set(Stone::X, current_row, current_col);
            self.set(op.color(), next_row, next_col);
            current_row = next_row;
            current_col = next_col;
            steps_taken += 1;
        }

        if steps_taken == 0 {
            // blocked completely, don't add any operations
            if print_op {
                println!("blocked {op:?}");
            }
            true
        } else if steps_taken == mag {
            // fully successful, add our op
            ops.push(op);
            false
        } else if 1 <= steps_taken && steps_taken < mag {
            // partially successful, add partial op
            let new_op = op.change_magnitude(steps_taken);
            if print_op {
                println!("partially blocked {op:?} -> {new_op:?}");
            }
            ops.push(new_op);
            false
        } else {
            unreachable!()
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    macro_rules! oplist {
        ($(($color:expr, $dir:ident, $side_effect:expr),)*) => {
            vec![$(Op {
                color: $color,
                dir: Dir::$dir,
                side_effect: $side_effect,
            },)*]
        };
    }

    macro_rules! mktest {
        (($color:expr, $dir:ident), $field:expr, $expfield:expr, $expvm:expr) => {
            let op = Op {
                color: $color,
                dir: Dir::$dir,
                side_effect: false,
            };
            let mut field = Field { field: $field };

            println!("before\n{field:?}");
            let ops = field.ops_for(op, true);
            println!("after\n{field:?}");

            assert_eq!($expfield, field.field, "expected left, got right");
            assert_eq!($expvm, ops, "expected left, got right");
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
            oplist!((OpColor::Blue, Right, false),)
        );
    }

    #[test]
    fn cancelled() {
        mktest!(
            (OpColor::Red(RedNumber::One), Right),
            [[Stone::Red, Stone::Blue, Stone::X]],
            [[Stone::Red, Stone::Blue, Stone::X]],
            Vec::<Op>::new()
        );
    }

    #[test]
    fn double_cancelled() {
        mktest!(
            (OpColor::Orange(OrangeNumber::Two), Right),
            [[Stone::Orange, Stone::X, Stone::Purple]],
            [[Stone::X, Stone::Orange, Stone::Purple]],
            oplist!((OpColor::Orange(OrangeNumber::One), Right, false),)
        );
    }

    // TODO necessary?
    //#[test]
    //fn non_interfere_left() {}
    //#[test]
    //fn non_interfere_right() {}
    //#[test]
    //fn non_interfere_up() {}
    //#[test]
    //fn non_interfere_down() {}
    //#[test]
    //fn non_interfere_left_wrap() {}
    //#[test]
    //fn non_interfere_right_wrap() {}
    //#[test]
    //fn non_interfere_up_wrap() {}
    //#[test]
    //fn non_interfere_down_wrap() {}
    //#[test]
    //fn non_interfere_push_left() {}
    //#[test]
    //fn non_interfere_push_right() {}
    //#[test]
    //fn non_interfere_push_up() {}
    //#[test]
    //fn non_interfere_push_down() {}
    //#[test]
    //fn non_interfere_push_left_wrap() {}
    //#[test]
    //fn non_interfere_push_right_wrap() {}
    //#[test]
    //fn non_interfere_push_up_wrap() {}
    //#[test]
    //fn non_interfere_push_down_wrap() {}

    #[test]
    fn left_wrap() {
        mktest!(
            (OpColor::Green, Left),
            [[Stone::Green, Stone::X, Stone::X]],
            [[Stone::X, Stone::X, Stone::Green]],
            oplist!((OpColor::Green, Left, false),)
        );
    }

    #[test]
    fn right_wrap() {
        mktest!(
            (OpColor::Green, Right),
            [[Stone::X, Stone::X, Stone::Green]],
            [[Stone::Green, Stone::X, Stone::X]],
            oplist!((OpColor::Green, Right, false),)
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
            oplist!((OpColor::Green, Up, false),)
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
            oplist!((OpColor::Green, Down, false),)
        );
    }

    #[test]
    fn push_left() {
        mktest!(
            (OpColor::Green, Left),
            [[Stone::X, Stone::Yellow, Stone::Green]],
            [[Stone::Yellow, Stone::Green, Stone::X]],
            oplist!((OpColor::Yellow, Left, true), (OpColor::Green, Left, false),)
        );
    }

    #[test]
    fn push_right() {
        mktest!(
            (OpColor::Green, Right),
            [[Stone::Green, Stone::Yellow, Stone::X]],
            [[Stone::X, Stone::Green, Stone::Yellow]],
            oplist!(
                (OpColor::Yellow, Right, true),
                (OpColor::Green, Right, false),
            )
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
            oplist!((OpColor::Yellow, Up, true), (OpColor::Green, Up, false),)
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
            oplist!((OpColor::Yellow, Down, true), (OpColor::Green, Down, false),)
        );
    }

    #[test]
    fn push_left_wrap() {
        mktest!(
            (OpColor::Green, Left),
            [[Stone::Yellow, Stone::Green, Stone::X]],
            [[Stone::Green, Stone::X, Stone::Yellow]],
            oplist!((OpColor::Yellow, Left, true), (OpColor::Green, Left, false),)
        );
    }

    #[test]
    fn push_right_wrap() {
        mktest!(
            (OpColor::Green, Right),
            [[Stone::X, Stone::Green, Stone::Yellow]],
            [[Stone::Yellow, Stone::X, Stone::Green]],
            oplist!(
                (OpColor::Yellow, Right, true),
                (OpColor::Green, Right, false),
            )
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
            oplist!((OpColor::Yellow, Up, true), (OpColor::Green, Up, false),)
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
            oplist!((OpColor::Yellow, Down, true), (OpColor::Green, Down, false),)
        );
    }

    #[test]
    fn double_push_left() {
        mktest!(
            (OpColor::Orange(OrangeNumber::Two), Right),
            [[Stone::Orange, Stone::Red, Stone::X, Stone::X]],
            [[Stone::X, Stone::X, Stone::Orange, Stone::Red]],
            oplist!(
                (OpColor::Red(RedNumber::One), Right, true),
                (OpColor::Red(RedNumber::One), Right, true),
                (OpColor::Orange(OrangeNumber::Two), Right, false),
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
                (OpColor::Red(RedNumber::One), Right, true),
                (OpColor::Red(RedNumber::One), Right, true),
                (OpColor::Orange(OrangeNumber::Two), Right, false),
            )
        );
    }

    #[test]
    fn double_push_left_wrap() {
        mktest!(
            (OpColor::Orange(OrangeNumber::Two), Right),
            [[Stone::X, Stone::X, Stone::Orange, Stone::Red]],
            [[Stone::Orange, Stone::Red, Stone::X, Stone::X]],
            oplist!(
                (OpColor::Red(RedNumber::One), Right, true),
                (OpColor::Red(RedNumber::One), Right, true),
                (OpColor::Orange(OrangeNumber::Two), Right, false),
            )
        );
    }

    #[test]
    fn double_push_right_wrap() {
        mktest!(
            (OpColor::Orange(OrangeNumber::Two), Right),
            [[Stone::X, Stone::X, Stone::Orange, Stone::Red]],
            [[Stone::Orange, Stone::Red, Stone::X, Stone::X]],
            oplist!(
                (OpColor::Red(RedNumber::One), Right, true),
                (OpColor::Red(RedNumber::One), Right, true),
                (OpColor::Orange(OrangeNumber::Two), Right, false),
            )
        );
    }

    #[test]
    fn double_push_cancelled() {
        mktest!(
            (OpColor::Orange(OrangeNumber::Two), Right),
            [[Stone::Orange, Stone::Red, Stone::X, Stone::Blue]],
            [[Stone::X, Stone::Orange, Stone::Red, Stone::Blue]],
            oplist!(
                (OpColor::Red(RedNumber::One), Right, true),
                (OpColor::Orange(OrangeNumber::One), Right, false),
            )
        );
    }

    #[test]
    fn double_push_cancelled_wrap() {
        mktest!(
            (OpColor::Orange(OrangeNumber::Two), Right),
            [[Stone::X, Stone::Blue, Stone::Orange, Stone::Red]],
            [[Stone::Red, Stone::Blue, Stone::X, Stone::Orange]],
            oplist!(
                (OpColor::Red(RedNumber::One), Right, true),
                (OpColor::Orange(OrangeNumber::One), Right, false),
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
                (OpColor::Red(RedNumber::One), Left, true),
                (OpColor::Orange(OrangeNumber::One), Left, true),
                (OpColor::Yellow, Left, true),
                (OpColor::Green, Left, true),
                (OpColor::Blue, Left, true),
                (OpColor::Purple, Left, false),
            )
        );
    }
}
