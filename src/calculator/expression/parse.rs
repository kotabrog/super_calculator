use super::super::num::Num;
use super::super::term::Term;
use super::super::operator::Operator;
use super::super::ast::Node;
use super::{Expression, TermType};

impl Expression {
    fn append_op_to_node(marker: Node<Term>, op: Operator) -> Result<Node<Term>, String> {
        match marker.clone().parent() {
            Some((mut parent, index)) => {
                let recursive_flag = match parent.clone().value().as_ref().unwrap() {
                    Term::Operator(parent_op) => {
                        parent_op.priority() >= op.priority()
                    },
                    Term::Num(num) => {
                        error!(
                            "構文解析に失敗しました。{} の親ノードが数字 {} になっています",
                            op, num,
                        );
                        return Err("構文解析に失敗しました".to_string())
                    },
                };
                if recursive_flag {
                    Self::append_op_to_node(parent, op)
                } else {
                    let mut new_marker = Node::new(Some(Term::Operator(op)));
                    new_marker.add_child(marker.clone());
                    parent.replace_child(index, new_marker.clone());
                    Ok(new_marker)
                }
            },
            None => {
                let mut new_marker = Node::new(Some(Term::Operator(op)));
                new_marker.add_child(marker.clone());
                Ok(new_marker)
            },
        }
    }

    fn append_to_ast_ops(mut marker: Node<Term>, c: char, unary: bool) -> Result<Node<Term>, String> {
        let op = Operator::parse(c, unary)?;
        if marker.value().is_none() {
            marker.set_value(Term::Operator(op));
            return Ok(marker);
        }
        let term_type = Self::get_term_type(&marker)?;
        match term_type {
            TermType::Num => {
                Self::append_op_to_node(marker, op)
            },
            TermType::Operator => {
                Err("演算子が連続しています".to_string())
            },
        }
    }

    fn append_to_ast_num(mut marker: Node<Term>, target: &mut String) -> Result<Node<Term>, String> {
        let num = Num::parse(target)?;
        target.clear();
        if marker.value().is_none() {
            marker.set_value(Term::Num(num));
            return Ok(marker);
        }
        let term_type = Self::get_term_type(&marker)?;
        match term_type {
            TermType::Num => {
                Err("数値が連続しています".to_string())
            },
            TermType::Operator => {
                let new_marker = Node::new(Some(Term::Num(num)));
                marker.add_child(new_marker.clone());
                Ok(new_marker)
            },
        }
    }

    pub fn parse(input: &str) -> Result<Self, String> {
        let mut root = Node::<Term>::new(None);
        let mut marker = root.clone();

        let mut chars = input
            .trim().chars().peekable();

        if let Some(&c) = chars.peek() {
            if Operator::is_operator(c) {
                marker = Self::append_to_ast_ops(marker, c, true)?;
                chars.next();
            }
        }

        let mut target_str = String::new();
        while let Some(&c) = chars.peek() {
            if c.is_digit(10) {
                target_str.push(c);
            } else {
                if !target_str.is_empty() {
                    marker = Self::append_to_ast_num(marker, &mut target_str)?;
                }
                if c.is_whitespace() {
                    // skip
                } else if Operator::is_operator(c) {
                    marker = Self::append_to_ast_ops(marker, c, false)?;
                } else {
                    return Err("対応していない文字です".to_string());
                }
            }
            chars.next();
        }

        if !target_str.is_empty() {
            marker = Self::append_to_ast_num(marker, &mut target_str)?;
        }

        root = marker.root();

        Ok(Self::new(root))
    }
}
