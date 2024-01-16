use super::super::term::Term;
use super::super::node::Node;
use super::{Expression, TermType};

impl Expression {
    fn display_loop(node: &Node<Term>) -> Result<Vec<String>, String> {
        let mut result = Vec::new();
        let term_type = Self::get_term_type(node)?;
        match term_type {
            TermType::Operator => {
                if Self::is_unary_operator(node)? {
                    if node.len_children() != 1 {
                        return Err("構文解析に失敗しました".to_string());
                    }
                    result.push(
                        node.value().as_ref().unwrap().to_string());
                    let child_result = Self::display_loop(
                        node.children().get(0).unwrap())?;
                    result.extend(child_result);
                } else {
                    if node.len_children() != 2 {
                        return Err("構文解析に失敗しました".to_string());
                    }
                    let left_result = Self::display_loop(
                        node.children().get(0).unwrap())?;
                    result.extend(left_result);
                    result.push(
                        node.value().as_ref().unwrap().to_string());
                    let right_result = Self::display_loop(
                        node.children().get(1).unwrap())?;
                    result.extend(right_result);
                }
            },
            TermType::Num => {
                if node.len_children() != 0 {
                    return Err("構文解析に失敗しました".to_string());
                }
                result.push(node.value().as_ref().unwrap().to_string());
            }
            TermType::Paren => {
                if !Self::is_right_paren(node)? {
                    return Err("括弧が閉じられていません".to_string());
                }
                if node.len_children() != 1 {
                    return Err("構文解析に失敗しました".to_string());
                }
                result.push("(".to_string());
                let child_result = Self::display_loop(
                    node.children().get(0).unwrap())?;
                result.extend(child_result);
                result.push(")".to_string());
            },
        }
        Ok(result)
    }

    pub fn display(&self) -> String {
        let mut result = Self::display_loop(&self.ast);
        match result {
            Err(_) => "構文解析に失敗しました".to_string(),
            Ok(ref mut result) => {
                result.join(" ")
            },
        }
    }
}
