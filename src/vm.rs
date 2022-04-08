#[derive(Clone, Copy, Debug)]
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

#[derive(Clone, Copy, Debug)]
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

#[derive(Clone, Copy, Debug)]
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

#[derive(Clone, Copy, Debug)]
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

#[derive(Default)]
pub struct Vm;

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

#[derive(Debug)]
pub struct Field<const W: usize = 12, const H: usize = 6> {
    field: [[Stone; W]; H],
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

impl<const W: usize, const H: usize> Field<W, H> {
    #[rustfmt::skip]
    pub fn new() -> Field<12, 6> {
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

    fn find(&self, color: Stone) -> (usize, usize) {
        for (row_idx, row) in self.field.iter().enumerate() {
            for (col_idx, my_color) in row.iter().enumerate() {
                if *my_color == color {
                    return (row_idx + 1, col_idx + 1);
                }
            }
        }

        unreachable!()
    }

    fn set(&mut self, color: Stone, row: usize, col: usize) {
        self.field[row - 1][col - 1] = color;
    }

    fn get(&mut self, row: usize, col: usize) -> Stone {
        self.field[row - 1][col - 1]
    }

    pub const fn width(&self) -> usize {
        W
    }

    pub const fn height(&self) -> usize {
        H
    }

    pub fn step(&mut self, op: Op, vm: &mut Vm) -> NewPosition {
        // find the position of the stone we need to move
        let (row_idx, col_idx) = self.find(op.color());
        // determine how many steps it needs to take
        let mag = op.magnitude();

        // determine if that many steps would wrap around the edges
        let wrap = Wrap {
            left: col_idx <= mag && op.dir == Dir::Left,
            right: col_idx + mag > W && op.dir == Dir::Right,
            up: row_idx <= mag && op.dir == Dir::Up,
            down: row_idx + mag > H && op.dir == Dir::Down,
        };

        // calculate the new position
        let new_col = if wrap.left && op.dir == Dir::Left {
            W - mag + col_idx
        } else if wrap.right && op.dir == Dir::Right {
            W - col_idx - mag
        } else if op.dir == Dir::Left {
            col_idx - 1
        } else if op.dir == Dir::Right {
            col_idx + 1
        } else {
            col_idx
        };

        let new_row = if wrap.up && op.dir == Dir::Up {
            H - mag + row_idx
        } else if wrap.down && op.dir == Dir::Down {
            H - row_idx - mag
        } else if op.dir == Dir::Up {
            row_idx - 1
        } else if op.dir == Dir::Down {
            row_idx + 1
        } else {
            row_idx
        };

        // determine if there's anything in the way, and if there is, move it

        match op.color() {
            Stone::Red => {}
            Stone::Orange => {}
            Stone::Yellow => {}
            Stone::Blue => {}
            Stone::Green => {}
            Stone::Purple => {}

            _ => unreachable!(),
        }

        NewPosition {
            wrap,
            new_col,
            new_row,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn step1() {
        let mut vm = Vm::default();
        let op = Op {
            color: OpColor::Red(RedNumber::One),
            dir: Dir::Left,
        };
        let mut field = Field {
            field: [[Stone::Red, Stone::X]],
        };
        let result = field.step(op, &mut vm);
        assert!(result.wrap.left());
        assert_eq!(result.new_col, 2);
    }

    #[test]
    fn step2() {
        let mut vm = Vm::default();
        let op = Op {
            color: OpColor::Red(RedNumber::One),
            dir: Dir::Right,
        };
        let mut field = Field {
            field: [[Stone::Red, Stone::X]],
        };
        let result = field.step(op, &mut vm);
        assert!(!result.wrap.right());
        assert_eq!(result.new_col, 2);
    }

    #[test]
    fn step3() {
        let mut vm = Vm::default();
        let op = Op {
            color: OpColor::Blue,
            dir: Dir::Up,
        };
        let mut field = Field {
            field: [[Stone::Blue, Stone::X]],
        };
        let result = field.step(op, &mut vm);
        assert!(result.wrap.up());
        assert_eq!(result.new_row, 1);
    }
}
