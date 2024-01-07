use super::term::Term;

#[derive(Debug, Clone)]
pub enum Operator {
    Add,
    Sub,
    Mul,
    Div,
}

impl Operator {
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
        }
    }

    fn sub(left: &Term, right: &Term) -> Result<Term, String> {
        match (left, right) {
            (Term::Num(x), Term::Num(y))
                => x.sub(y).map(Term::Num),
        }
    }

    fn mul(left: &Term, right: &Term) -> Result<Term, String> {
        match (left, right) {
            (Term::Num(x), Term::Num(y))
                => x.mul(y).map(Term::Num),
        }
    }

    fn div(left: &Term, right: &Term) -> Result<Term, String> {
        match (left, right) {
            (Term::Num(x), Term::Num(y))
                => x.div(y).map(Term::Num),
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
