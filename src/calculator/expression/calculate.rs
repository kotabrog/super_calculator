use super::super::term::Term;
use super::super::ast::Node;
use super::{Expression, TermType};

impl Expression {
    fn calculate_loop(node: &Node<Term>) -> Result<Term, String> {
        let term_type = Self::get_term_type(node)?;
        match term_type {
            TermType::Operator => {
                if node.len_children() != 2 {
                    return Err("構文解析に失敗しました".to_string());
                }
                let left_result = Self::calculate_loop(
                    node.children().get(0).unwrap())?;
                let right_result = Self::calculate_loop(
                    node.children().get(1).unwrap())?;
                match node.value().as_ref().unwrap() {
                    Term::Operator(operator) => {
                        operator.calculate(&left_result, &right_result)
                    },
                    Term::Num(_) => {
                        Err("構文解析に失敗しました".to_string())
                    },
                }
            },
            TermType::Num => {
                if node.len_children() != 0 {
                    return Err("構文解析に失敗しました".to_string());
                }
                Ok(node.value().as_ref().unwrap().clone())
            }
        }
    }

    pub fn calculate(&self) -> Result<Term, String> {
        Self::calculate_loop(&self.ast)
    }
}
