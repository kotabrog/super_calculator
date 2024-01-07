use super::num::Num;

#[derive(Debug, Clone)]
pub enum Term {
    Num(Num),
}

impl std::fmt::Display for Term {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Term::Num(num) => write!(f, "{}", num),
        }
    }
}
