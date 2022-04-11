use crate::{Command, Dir, Stone};

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

    pub fn commands_for(&mut self, cmd: Command, print_cmd: bool) -> Vec<Command> {
        let mut cmds = Vec::new();
        self.step_rec(&mut cmds, cmd, print_cmd);
        cmds
    }

    // returns whether the move was blocked
    fn step_rec(&mut self, cmds: &mut Vec<Command>, cmd: Command, print_cmd: bool) -> bool {
        let (row_idx, col_idx) = self.find(cmd.color);
        let (mut current_row, mut current_col) = (row_idx, col_idx);

        let mag = cmd.magnitude();
        let mut steps_taken = 0;
        for _ in 1..=mag {
            let next_row = Self::next_row(current_row, cmd.dir);
            let next_col = Self::next_col(current_col, cmd.dir);
            let next = self.get(next_row, next_col);

            if next > cmd.color {
                // next is heavier, quit early
                break;
            } else if next == Stone::X {
                // next is empty, just move it
            } else if next != Stone::X && next < cmd.color {
                // next is lighter
                let next_cmd = Command {
                    color: next,
                    number: next.number_one(),
                    dir: cmd.dir,
                    side_effect: true,
                };
                let blocked = self.step_rec(cmds, next_cmd, print_cmd);
                if blocked {
                    break;
                }
                if print_cmd {
                    println!("pushed {cmd:?} > {next:?}");
                }
            } else {
                unreachable!()
            }

            self.set(Stone::X, current_row, current_col);
            self.set(cmd.color, next_row, next_col);
            current_row = next_row;
            current_col = next_col;
            steps_taken += 1;
        }

        if steps_taken == 0 {
            // blocked completely, don't add any operations
            if print_cmd {
                println!("blocked {cmd:?}");
            }
            true
        } else if steps_taken == mag {
            // fully successful, add our op
            cmds.push(cmd);
            false
        } else if 1 <= steps_taken && steps_taken < mag {
            // partially successful, add partial op
            let new_cmd = cmd.change_magnitude(steps_taken);
            if print_cmd {
                println!("partially blocked {cmd:?} -> {new_cmd:?}");
            }
            cmds.push(new_cmd);
            false
        } else {
            unreachable!()
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::{orange, red};

    macro_rules! oplist {
        ($(($color:expr, $dir:ident, $number:expr, $side_effect:expr),)*) => {
            vec![$(Command {
                color: $color,
                dir: Dir::$dir,
                number: $number,
                side_effect: $side_effect,
            },)*]
        };
    }

    macro_rules! mktest {
        (($color:expr, $dir:ident, $number:expr), $field:expr, $expfield:expr, $expvm:expr) => {
            let op = Command {
                color: $color,
                dir: Dir::$dir,
                number: $number,
                side_effect: false,
            };
            let mut field = Field { field: $field };

            println!("before\n{field:?}");
            let ops = field.commands_for(op, true);
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
            (Stone::Blue, Right, None),
            [[Stone::Blue, Stone::X]],
            [[Stone::X, Stone::Blue]],
            oplist!((Stone::Blue, Right, None, false),)
        );
    }

    #[test]
    fn cancelled() {
        mktest!(
            (Stone::Red, Right, Some(red!(One))),
            [[Stone::Red, Stone::Blue, Stone::X]],
            [[Stone::Red, Stone::Blue, Stone::X]],
            Vec::<Command>::new()
        );
    }

    #[test]
    fn double_cancelled() {
        mktest!(
            (Stone::Orange, Right, Some(orange!(One))),
            [[Stone::Orange, Stone::X, Stone::Purple]],
            [[Stone::X, Stone::Orange, Stone::Purple]],
            oplist!((Stone::Orange, Right, Some(orange!(One)), false),)
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
            (Stone::Green, Left, None),
            [[Stone::Green, Stone::X, Stone::X]],
            [[Stone::X, Stone::X, Stone::Green]],
            oplist!((Stone::Green, Left, None, false),)
        );
    }

    #[test]
    fn right_wrap() {
        mktest!(
            (Stone::Green, Right, None),
            [[Stone::X, Stone::X, Stone::Green]],
            [[Stone::Green, Stone::X, Stone::X]],
            oplist!((Stone::Green, Right, None, false),)
        );
    }

    #[test]
    fn up_wrap() {
        mktest!(
            (Stone::Green, Up, None),
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
            oplist!((Stone::Green, Up, None, false),)
        );
    }

    #[test]
    fn down_wrap() {
        mktest!(
            (Stone::Green, Down, None),
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
            oplist!((Stone::Green, Down, None, false),)
        );
    }

    #[test]
    fn push_left() {
        mktest!(
            (Stone::Green, Left, None),
            [[Stone::X, Stone::Yellow, Stone::Green]],
            [[Stone::Yellow, Stone::Green, Stone::X]],
            oplist!(
                (Stone::Yellow, Left, None, true),
                (Stone::Green, Left, None, false),
            )
        );
    }

    #[test]
    fn push_right() {
        mktest!(
            (Stone::Green, Right, None),
            [[Stone::Green, Stone::Yellow, Stone::X]],
            [[Stone::X, Stone::Green, Stone::Yellow]],
            oplist!(
                (Stone::Yellow, Right, None, true),
                (Stone::Green, Right, None, false),
            )
        );
    }

    #[test]
    fn push_up() {
        mktest!(
            (Stone::Green, Up, None),
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
            oplist!(
                (Stone::Yellow, Up, None, true),
                (Stone::Green, Up, None, false),
            )
        );
    }

    #[test]
    fn push_down() {
        mktest!(
            (Stone::Green, Down, None),
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
            oplist!(
                (Stone::Yellow, Down, None, true),
                (Stone::Green, Down, None, false),
            )
        );
    }

    #[test]
    fn push_left_wrap() {
        mktest!(
            (Stone::Green, Left, None),
            [[Stone::Yellow, Stone::Green, Stone::X]],
            [[Stone::Green, Stone::X, Stone::Yellow]],
            oplist!(
                (Stone::Yellow, Left, None, true),
                (Stone::Green, Left, None, false),
            )
        );
    }

    #[test]
    fn push_right_wrap() {
        mktest!(
            (Stone::Green, Right, None),
            [[Stone::X, Stone::Green, Stone::Yellow]],
            [[Stone::Yellow, Stone::X, Stone::Green]],
            oplist!(
                (Stone::Yellow, Right, None, true),
                (Stone::Green, Right, None, false),
            )
        );
    }

    #[test]
    fn push_up_wrap() {
        mktest!(
            (Stone::Green, Up, None),
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
            oplist!(
                (Stone::Yellow, Up, None, true),
                (Stone::Green, Up, None, false),
            )
        );
    }

    #[test]
    fn push_down_wrap() {
        mktest!(
            (Stone::Green, Down, None),
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
            oplist!(
                (Stone::Yellow, Down, None, true),
                (Stone::Green, Down, None, false),
            )
        );
    }

    #[test]
    fn double_push_left() {
        mktest!(
            (Stone::Orange, Right, Some(orange!(Two))),
            [[Stone::Orange, Stone::Red, Stone::X, Stone::X]],
            [[Stone::X, Stone::X, Stone::Orange, Stone::Red]],
            oplist!(
                (Stone::Red, Right, Some(red!(One)), true),
                (Stone::Red, Right, Some(red!(One)), true),
                (Stone::Orange, Right, Some(orange!(Two)), false),
            )
        );
    }

    #[test]
    fn double_push_right() {
        mktest!(
            (Stone::Orange, Right, Some(orange!(Two))),
            [[Stone::Orange, Stone::Red, Stone::X, Stone::X]],
            [[Stone::X, Stone::X, Stone::Orange, Stone::Red]],
            oplist!(
                (Stone::Red, Right, Some(red!(One)), true),
                (Stone::Red, Right, Some(red!(One)), true),
                (Stone::Orange, Right, Some(orange!(Two)), false),
            )
        );
    }

    #[test]
    fn double_push_left_wrap() {
        mktest!(
            (Stone::Orange, Right, Some(orange!(Two))),
            [[Stone::X, Stone::X, Stone::Orange, Stone::Red]],
            [[Stone::Orange, Stone::Red, Stone::X, Stone::X]],
            oplist!(
                (Stone::Red, Right, Some(red!(One)), true),
                (Stone::Red, Right, Some(red!(One)), true),
                (Stone::Orange, Right, Some(orange!(Two)), false),
            )
        );
    }

    #[test]
    fn double_push_right_wrap() {
        mktest!(
            (Stone::Orange, Right, Some(orange!(Two))),
            [[Stone::X, Stone::X, Stone::Orange, Stone::Red]],
            [[Stone::Orange, Stone::Red, Stone::X, Stone::X]],
            oplist!(
                (Stone::Red, Right, Some(red!(One)), true),
                (Stone::Red, Right, Some(red!(One)), true),
                (Stone::Orange, Right, Some(orange!(Two)), false),
            )
        );
    }

    #[test]
    fn double_push_cancelled() {
        mktest!(
            (Stone::Orange, Right, Some(orange!(Two))),
            [[Stone::Orange, Stone::Red, Stone::X, Stone::Blue]],
            [[Stone::X, Stone::Orange, Stone::Red, Stone::Blue]],
            oplist!(
                (Stone::Red, Right, Some(red!(One)), true),
                (Stone::Orange, Right, Some(orange!(One)), false),
            )
        );
    }

    #[test]
    fn double_push_cancelled_wrap() {
        mktest!(
            (Stone::Orange, Right, Some(orange!(Two))),
            [[Stone::X, Stone::Blue, Stone::Orange, Stone::Red]],
            [[Stone::Red, Stone::Blue, Stone::X, Stone::Orange]],
            oplist!(
                (Stone::Red, Right, Some(red!(One)), true),
                (Stone::Orange, Right, Some(orange!(One)), false),
            )
        );
    }

    #[test]
    fn choo_choo() {
        mktest!(
            (Stone::Purple, Left, None),
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
                (Stone::Red, Left, Some(red!(One)), true),
                (Stone::Orange, Left, Some(orange!(One)), true),
                (Stone::Yellow, Left, None, true),
                (Stone::Green, Left, None, true),
                (Stone::Blue, Left, None, true),
                (Stone::Purple, Left, None, false),
            )
        );
    }
}
