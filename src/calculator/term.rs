use super::num::Num;
use super::operator::Operator;

#[derive(Debug, Clone, PartialEq)]
pub enum Term {
    Num(Num),
    Operator(Operator),
}

impl std::fmt::Display for Term {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Term::Num(num) => write!(f, "{}", num),
            Term::Operator(op) => write!(f, "{}", op),
        }
    }
}
