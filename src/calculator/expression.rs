use super::term::Term;
use super::operator::Operator;

#[derive(Debug, Clone)]
pub struct Expression {
    left: Term,
    right: Term,
    operator: Operator,
}

impl Expression {
    pub fn new(left: Term, right: Term, operator: Operator) -> Self {
        Self { left, right, operator }
    }

    pub fn calculate(&self) -> Result<Term, String> {
        self.operator.calculate(&self.left, &self.right)
    }
}

impl std::fmt::Display for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{} {} {}", self.left, self.operator, self.right)
    }
}
