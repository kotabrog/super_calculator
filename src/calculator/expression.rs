mod parse;
mod display;
mod calculate;

use super::term::Term;
use super::ast::Node;

#[derive(Debug, Clone)]
pub struct Expression {
    ast: Node<Term>,
}

enum TermType {
    Num,
    Operator,
}

impl Expression {
    pub fn new(ast: Node<Term>) -> Self {
        Self {
            ast,
        }
    }

    fn get_term_type(node: &Node<Term>) -> Result<TermType, String> {
        match node.value().as_ref() {
            Some(Term::Num(_)) => Ok(TermType::Num),
            Some(Term::Operator(_)) => Ok(TermType::Operator),
            _ => Err("構文解析に失敗しました".to_string()),
        }
    }

    fn is_unary_operator(node: &Node<Term>) -> Result<bool, String> {
        match node.value().as_ref() {
            Some(Term::Operator(op)) => Ok(op.is_unary()),
            _ => Err("構文解析に失敗しました".to_string()),
        }
    }
}

impl std::fmt::Display for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.display())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::calculator::num::Num;

    #[test]
    fn test_parse() {
        let input = "1 + 2 * 3 - 1";
        let expression = Expression::parse(input).unwrap();
        println!("{} = {}", input, expression.display());
        assert_eq!(expression.display(), input);
    }

    #[test]
    fn test_calculate() {
        let input = "-1 + 2 * 3 - 1";
        let expression = Expression::parse(input).unwrap();
        let result = expression.calculate().unwrap();
        println!("{} = {}", input, result);
        assert_eq!(result, Term::Num(Num::I32(4)));
    }
}
