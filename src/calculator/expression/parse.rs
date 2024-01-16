use super::super::num::Num;
use super::super::term::Term;
use super::super::operator::Operator;
use super::super::paren::Paren;
use super::super::node::Node;
use super::{Expression, TermType};

impl Expression {
    fn search_left_paren_loop(marker: Node<Term>) -> Result<Node<Term>, String> {
        match marker.parent() {
            Some((parent, _)) => {
                let recursive_flag = match parent.value().as_ref().unwrap() {
                    Term::Paren(paren) => {
                        !paren.is_left()
                    },
                    _ => true,
                };
                if recursive_flag {
                    Self::search_left_paren_loop(parent)
                } else {
                    Ok(parent)
                }
            },
            None => {
                Err("括弧の対応が取れていません".to_string())
            },
        }
    }

    fn append_right_paren_to_node(marker: Node<Term>) -> Result<Node<Term>, String> {
        match marker.value().as_ref().unwrap() {
            Term::Paren(paren) => {
                if paren.is_left() {
                    return Err("括弧の中に要素がありませんでした".to_string());
                }
            },
            _ => {}
        }
        let mut marker = Self::search_left_paren_loop(marker)?;
        marker.set_value(Term::Paren(Paren::Right));
        Ok(marker)
    }

    fn append_left_paren_to_node(mut marker: Node<Term>) -> Result<Node<Term>, String> {
        if marker.value().is_none() {
            marker.set_value(Term::Paren(Paren::Left));
            return Ok(marker);
        }
        let term_type = Self::get_term_type(&marker)?;
        let mut marker = match term_type {
            TermType::Num => {
                Self::append_op_to_node(marker, Operator::Mul)?
            },
            TermType::Operator => {
                marker
            },
            TermType::Paren => {
                if Self::is_right_paren(&marker)? {
                    Self::append_op_to_node(marker, Operator::Mul)?
                } else {
                    marker
                }
            },
        };
        let new_marker = Node::new(Some(Term::Paren(Paren::Left)));
        marker.add_child(new_marker.clone());
        Ok(new_marker)
    }

    fn append_to_ast_paren(marker: Node<Term>, c: char) -> Result<Node<Term>, String> {
        let paren = Paren::parse(c)?;

        match paren {
            Paren::Left => Self::append_left_paren_to_node(marker),
            Paren::Right => Self::append_right_paren_to_node(marker),
        }
    }

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
                    Term::Paren(paren) => {
                        if paren.is_left() {
                            false
                        } else {
                            error!(
                                "構文解析に失敗しました。{} の親ノードが括弧 {} になっています",
                                op, paren,
                            );
                            return Err("構文解析に失敗しました".to_string())
                        }
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

    fn append_to_ast_ops(mut marker: Node<Term>, c: char) -> Result<Node<Term>, String> {
        let term_type = Self::get_term_type(&marker);
        let op = match term_type {
            Ok(TermType::Operator) => {
                return Err("演算子が連続しています".to_string());
            }
            Ok(TermType::Paren) => {
                if Self::is_right_paren(&marker)? {
                    Operator::parse(c, false)?
                } else {
                    let op = Operator::parse(c, true)?;
                    let new_marker = Node::new(Some(Term::Operator(op)));
                    marker.add_child(new_marker.clone());
                    return Ok(new_marker);
                }
            },
            Err(_) => {
                let op = Operator::parse(c, true)?;
                marker.set_value(Term::Operator(op));
                return Ok(marker);
            }
            _ => Operator::parse(c, false)?
        };
        Self::append_op_to_node(marker, op)
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
            TermType::Paren => {
                if Self::is_right_paren(&marker)? {
                    let mut marker = Self::append_op_to_node(marker, Operator::Mul)?;
                    let new_marker = Node::new(Some(Term::Num(num)));
                    marker.add_child(new_marker.clone());
                    Ok(new_marker)
                } else {
                    let new_marker = Node::new(Some(Term::Num(num)));
                    marker.add_child(new_marker.clone());
                    Ok(new_marker)
                }
            },
        }
    }

    pub fn parse(input: &str) -> Result<Self, String> {
        let mut root = Node::<Term>::new(None);
        let mut marker = root.clone();

        let mut chars = input
            .trim().chars().peekable();

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
                    marker = Self::append_to_ast_ops(marker, c)?;
                } else if Paren::is_paren(c) {
                    marker = Self::append_to_ast_paren(marker, c)?;
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
