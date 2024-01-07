use super::Calculator;
use super::num::Num;
use super::term::Term;
use super::operator::Operator;
use super::expression::Expression;

impl Calculator {
    pub(super) fn format_input(input: &str) -> String {
        input.to_string()
    }

    pub(super) fn parse(input: &str) -> Result<Expression, String> {
        let mut chars = input
            .trim().chars().peekable();
        let mut left_str = String::new();
        let mut right_str = String::new();

        if chars.peek() == Some(&'-') {
            left_str.push('-');
            chars.next();
        }

        while let Some(&c) = chars.peek() {
            if c.is_digit(10) {
                left_str.push(c);
                chars.next();
            } else {
                break;
            }
        }

        while let Some(&c) = chars.peek() {
            if c.is_whitespace() {
                chars.next();
            } else {
                break;
            }
        }

        let operator = if let Some(op) = chars.next() {
            if "+-*/".contains(op) {
                match op {
                    '+' => Operator::Add,
                    '-' => Operator::Sub,
                    '*' => Operator::Mul,
                    '/' => Operator::Div,
                    _ => unreachable!(),
                }
            } else {
                return Err("無効な演算子です".to_string());
            }
        } else {
            return Err("演算子が見つかりません".to_string());
        };

        while let Some(&c) = chars.peek() {
            if c.is_whitespace() {
                chars.next();
            } else {
                break;
            }
        }

        right_str.extend(chars);

        Ok(Expression::new(
            Term::Num(Num::parse(&left_str)?),
            Term::Num(Num::parse(&right_str)?),
            operator,
        ))
    }
}
