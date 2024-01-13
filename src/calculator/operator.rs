use super::term::Term;

#[derive(Debug, Clone, PartialEq)]
pub enum Operator {
    Add,
    Sub,
    Mul,
    Div,
    Plus,
    Minus,
}

impl Operator {
    pub fn is_operator(c: char) -> bool {
        "+-*/".contains(c)
    }

    pub fn is_unary(&self) -> bool {
        match self {
            Operator::Add => false,
            Operator::Sub => false,
            Operator::Mul => false,
            Operator::Div => false,
            Operator::Plus => true,
            Operator::Minus => true,
        }
    }

    pub fn parse(c: char, unary: bool) -> Result<Self, String> {
        match (c, unary) {
            ('+', false) => Ok(Operator::Add),
            ('-', false) => Ok(Operator::Sub),
            ('*', false) => Ok(Operator::Mul),
            ('/', false) => Ok(Operator::Div),
            ('+', true) => Ok(Operator::Plus),
            ('-', true) => Ok(Operator::Minus),
            _ => Err("無効な演算子です".to_string()),
        }
    }

    pub fn priority(&self) -> u8 {
        match self {
            Operator::Add => 1,
            Operator::Sub => 1,
            Operator::Mul => 2,
            Operator::Div => 2,
            Operator::Plus => 3,
            Operator::Minus => 3,
        }
    }

    pub fn calculate_unary(&self, term: &Term) -> Result<Term, String> {
        match self {
            Operator::Plus => Self::plus(term),
            Operator::Minus => Self::minus(term),
            _ => Err("無効な演算です".to_string()),
        }
    }

    pub fn calculate_binary(&self, left: &Term, right: &Term) -> Result<Term, String> {
        match self {
            Operator::Add => Self::add(left, right),
            Operator::Sub => Self::sub(left, right),
            Operator::Mul => Self::mul(left, right),
            Operator::Div => Self::div(left, right),
            _ => Err("無効な演算です".to_string()),
        }
    }

    // pub fn calculate(&self, terms: &[Term]) -> Result<Term, String> {
    //     let terms_len = terms.len();
    //     match terms_len {
    //         1 => self.calculate_unary(&terms[0]),
    //         2 => self.calculate_binary(&terms[0], &terms[1]),
    //         _ => Err("無効な演算です".to_string()),
    //     }
    // }

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

    fn plus(term: &Term) -> Result<Term, String> {
        match term {
            Term::Num(x) => x.plus().map(Term::Num),
            _ => Err("無効な演算です".to_string()),
        }
    }

    fn minus(term: &Term) -> Result<Term, String> {
        match term {
            Term::Num(x) => x.minus().map(Term::Num),
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
            Operator::Plus => "+",
            Operator::Minus => "-",
        };
        write!(f, "{}", op)
    }
}
