mod setup;
mod format;
mod history;
mod num;
mod term;
mod operator;
mod expression;

use anyhow::Result;

const INPUT_AREA: &str = "input-area";
const FORMATTED_DISPLAY: &str = "formatted-display";
const HISTORY_CONTAINER: &str = "history-container";
const HELP_BUTTON: &str = "help-button";
const HELP_POPUP_CONTAINER: &str = "help-popup-container";
const CLOSE_HELP: &str = "close-help";
const HISTORY_ITEM: &str = "history-item";
const HIDDEN: &str = "hidden";

pub struct Calculator {}

impl Calculator {
    fn calculate_and_format(input: &str) -> Result<String, String> {
        let expression = Self::parse(input)?;
        let result = expression.calculate()?;
        Ok(format!("{} → {}", expression, result))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_normal() {
        let input = "1 + 2";
        let expected = "1 + 2 → 3";
        let actual = Calculator::calculate_and_format(input).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn parse_invalid_operator() {
        let input = "1 & 2";
        let expected = "無効な演算子です";
        match Calculator::calculate_and_format(input) {
            Ok(_) => panic!("should be error"),
            Err(e) => assert_eq!(expected, e),
        }
    }

    #[test]
    fn parse_missing_operator() {
        let input = "1 2";
        let expected = "無効な演算子です";
        match Calculator::calculate_and_format(input) {
            Ok(_) => panic!("should be error"),
            Err(e) => assert_eq!(expected, e),
        }
    }

    #[test]
    fn parse_only_number() {
        let input = "1";
        let expected = "演算子が見つかりません";
        match Calculator::calculate_and_format(input) {
            Ok(_) => panic!("should be error"),
            Err(e) => assert_eq!(expected, e),
        }
    }

    #[test]
    fn parse_missing_right_operand() {
        let input = "1 + ";
        let expected = "数値に変換できません";
        match Calculator::calculate_and_format(input) {
            Ok(_) => panic!("should be error"),
            Err(e) => assert_eq!(expected, e),
        }
    }

    #[test]
    fn parse_missing_left_operand() {
        let input = " + 2";
        let expected = "数値に変換できません";
        match Calculator::calculate_and_format(input) {
            Ok(_) => panic!("should be error"),
            Err(e) => assert_eq!(expected, e),
        }
    }

    #[test]
    fn parse_many_spaces() {
        let input = "  1  +  2  ";
        let expected = "1 + 2 → 3";
        let actual = Calculator::calculate_and_format(input).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_add_normal() {
        let input = "1 + 2";
        let expected = "1 + 2 → 3";
        let actual = Calculator::calculate_and_format(input).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_add_overflow() {
        let input = "2147483647 + 1";
        let expected = "int32の範囲を超える加算です";
        match Calculator::calculate_and_format(input) {
            Ok(_) => panic!("should be error"),
            Err(e) => assert_eq!(expected, e),
        }
    }

    #[test]
    fn test_sub_normal() {
        let input = "1 - 2";
        let expected = "1 - 2 → -1";
        let actual = Calculator::calculate_and_format(input).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_sub_overflow() {
        let input = "-2147483648 - 1";
        let expected = "int32の範囲を超える減算です";
        match Calculator::calculate_and_format(input) {
            Ok(_) => panic!("should be error"),
            Err(e) => assert_eq!(expected, e),
        }
    }

    #[test]
    fn test_mul_normal() {
        let input = "2 * 3";
        let expected = "2 * 3 → 6";
        let actual = Calculator::calculate_and_format(input).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_mul_overflow() {
        let input = "2147483647 * 2";
        let expected = "int32の範囲を超える乗算です";
        match Calculator::calculate_and_format(input) {
            Ok(_) => panic!("should be error"),
            Err(e) => assert_eq!(expected, e),
        }
    }

    #[test]
    fn test_div_normal() {
        let input = "6 / 3";
        let expected = "6 / 3 → 2";
        let actual = Calculator::calculate_and_format(input).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_div_overflow() {
        let input = "-2147483648 / -1";
        let expected = "int32の範囲を超える除算です";
        match Calculator::calculate_and_format(input) {
            Ok(_) => panic!("should be error"),
            Err(e) => assert_eq!(expected, e),
        }
    }

    #[test]
    fn test_div_by_zero() {
        let input = "1 / 0";
        let expected = "0で割ることはできません";
        match Calculator::calculate_and_format(input) {
            Ok(_) => panic!("should be error"),
            Err(e) => assert_eq!(expected, e),
        }
    }
}
