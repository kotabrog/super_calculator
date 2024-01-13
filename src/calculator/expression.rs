mod parse;
mod display;
mod calculate;

use super::num::Num;
use super::term::Term;
use super::operator::Operator;
use super::ast::Node;

#[derive(Debug, Clone)]
pub struct Expression {
    left: Term,
    right: Term,
    operator: Operator,
    ast: Node<Term>,
}

enum TermType {
    Num,
    Operator,
}

impl Expression {
    pub fn new(left: Term, right: Term, operator: Operator) -> Self {
        Self { left, right, operator, ast: Node::<Term>::new(None) }
    }

    pub fn new_from_ast(ast: Node<Term>) -> Self {
        Self {
            left: Term::Num(Num::I32(0)),
            right: Term::Num(Num::I32(0)),
            operator: Operator::Add,
            ast,
        }
    }

    pub fn calculate_temp(&self) -> Result<Term, String> {
        self.operator.calculate(&self.left, &self.right)
    }

    fn get_term_type(node: &Node<Term>) -> Result<TermType, String> {
        match node.value().as_ref() {
            Some(Term::Num(_)) => Ok(TermType::Num),
            Some(Term::Operator(_)) => Ok(TermType::Operator),
            _ => Err("構文解析に失敗しました".to_string()),
        }
    }
}

impl std::fmt::Display for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{} {} {}", self.left, self.operator, self.right)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let input = "1 + 2 * 3 - 1";
        let expression = Expression::parse(input).unwrap();
        println!("{} = {}", input, expression.display());
        assert_eq!(expression.display(), input);
    }

    #[test]
    fn test_calculate() {
        let input = "1 + 2 * 3 - 1";
        let expression = Expression::parse(input).unwrap();
        let result = expression.calculate().unwrap();
        println!("{} = {}", input, result);
        assert_eq!(result, Term::Num(Num::I32(6)));
    }
}
