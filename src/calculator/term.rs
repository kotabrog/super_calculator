use super::num::Num;
use super::operator::Operator;
use super::paren::Paren;

#[derive(Debug, Clone, PartialEq)]
pub enum Term {
    Num(Num),
    Operator(Operator),
    Paren(Paren),
}

impl std::fmt::Display for Term {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Term::Num(num) => write!(f, "{}", num),
            Term::Operator(op) => write!(f, "{}", op),
            Term::Paren(paren) => write!(f, "{}", paren),
        }
    }
}
