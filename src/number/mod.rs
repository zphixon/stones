
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Number {
    One,
    Two,
    Three,
    None
}

impl Number {
    pub fn from_str(num: &str) -> Number {
        match num {
            "1" => Number::One,
            "2" => Number::Two,
            "3" => Number::Three,
            _ => Number::None
        }
    }
}

