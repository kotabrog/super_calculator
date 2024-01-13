use super::term::Term;

#[derive(Debug, Clone, PartialEq)]
pub enum Operator {
    Add,
    Sub,
    Mul,
    Div,
}

impl Operator {
    pub fn is_operator(c: char) -> bool {
        "+-*/".contains(c)
    }

    pub fn parse(c: char) -> Result<Self, String> {
        match c {
            '+' => Ok(Operator::Add),
            '-' => Ok(Operator::Sub),
            '*' => Ok(Operator::Mul),
            '/' => Ok(Operator::Div),
            _ => Err("無効な演算子です".to_string()),
        }
    }

    pub fn priority(&self) -> u8 {
        match self {
            Operator::Add => 1,
            Operator::Sub => 1,
            Operator::Mul => 2,
            Operator::Div => 2,
        }
    }

    pub fn calculate(&self, left: &Term, right: &Term) -> Result<Term, String> {
        match self {
            Operator::Add => Self::add(left, right),
            Operator::Sub => Self::sub(left, right),
            Operator::Mul => Self::mul(left, right),
            Operator::Div => Self::div(left, right),
        }
    }

    fn add(left: &Term, right: &Term) -> Result<Term, String> {
        match (left, right) {
            (Term::Num(x), Term::Num(y))
                => x.add(y).map(Term::Num),
            _ => Err("無効な演算です".to_string()),
        }
    }

    fn sub(left: &Term, right: &Term) -> Result<Term, String> {
        match (left, right) {
            (Term::Num(x), Term::Num(y))
                => x.sub(y).map(Term::Num),
            _ => Err("無効な演算です".to_string()),
        }
    }

    fn mul(left: &Term, right: &Term) -> Result<Term, String> {
        match (left, right) {
            (Term::Num(x), Term::Num(y))
                => x.mul(y).map(Term::Num),
            _ => Err("無効な演算です".to_string()),
        }
    }

    fn div(left: &Term, right: &Term) -> Result<Term, String> {
        match (left, right) {
            (Term::Num(x), Term::Num(y))
                => x.div(y).map(Term::Num),
            _ => Err("無効な演算です".to_string()),
        }
    }
}

impl std::fmt::Display for Operator {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let op = match self {
            Operator::Add => "+",
            Operator::Sub => "-",
            Operator::Mul => "*",
            Operator::Div => "/",
        };
        write!(f, "{}", op)
    }
}
