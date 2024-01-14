#[derive(Debug, Clone, PartialEq)]
pub enum Paren {
    Left,
    Right,
}

impl Paren {
    pub fn is_paren(c: char) -> bool {
        "()".contains(c)
    }

    pub fn parse(c: char) -> Result<Self, String> {
        match c {
            '(' => Ok(Paren::Left),
            ')' => Ok(Paren::Right),
            _ => Err("括弧にparseができませんでした".to_string()),
        }
    }

    pub fn is_left(&self) -> bool {
        match self {
            Paren::Left => true,
            Paren::Right => false,
        }
    }
}

impl std::fmt::Display for Paren {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Paren::Left => write!(f, "("),
            Paren::Right => write!(f, ")"),
        }
    }
}
