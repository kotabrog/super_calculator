use super::super::term::Term;
use super::super::node::Node;
use super::{Expression, TermType};

impl Expression {
    fn calculate_loop(node: &Node<Term>) -> Result<Term, String> {
        let term_type = Self::get_term_type(node)?;
        match term_type {
            TermType::Operator => {
                if Self::is_unary_operator(node)? {
                    if node.len_children() != 1 {
                        return Err("構文解析に失敗しました".to_string());
                    }
                    let child_result = Self::calculate_loop(
                        node.children().get(0).unwrap())?;
                    match node.value().as_ref().unwrap() {
                        Term::Operator(operator) => {
                            operator.calculate_unary(&child_result)
                        },
                        _ => Err("構文解析に失敗しました".to_string())
                    }
                } else {
                    if node.len_children() != 2 {
                        return Err("構文解析に失敗しました".to_string());
                    }
                    let left_result = Self::calculate_loop(
                        node.children().get(0).unwrap())?;
                    let right_result = Self::calculate_loop(
                        node.children().get(1).unwrap())?;
                    match node.value().as_ref().unwrap() {
                        Term::Operator(operator) => {
                            operator.calculate_binary(&left_result, &right_result)
                        },
                        _ => Err("構文解析に失敗しました".to_string())
                    }
                }
            },
            TermType::Num => {
                if node.len_children() != 0 {
                    return Err("構文解析に失敗しました".to_string());
                }
                Ok(node.value().as_ref().unwrap().clone())
            },
            TermType::Paren => {
                if !Self::is_right_paren(node)? {
                    return Err("括弧が閉じられていません".to_string());
                }
                if node.len_children() != 1 {
                    return Err("構文解析に失敗しました".to_string());
                }
                Self::calculate_loop(node.children().get(0).unwrap())
            },
        }
    }

    pub fn calculate(&self) -> Result<Term, String> {
        Self::calculate_loop(&self.ast)
    }
}
