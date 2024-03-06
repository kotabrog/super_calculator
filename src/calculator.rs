mod setup;
mod variable_button;
mod format;
mod history;
mod variable_manager;
mod num;
mod term;
mod operator;
mod expression;
mod node;
mod paren;
mod fraction;

use anyhow::Result;
use expression::Expression;

const LEFT_PANEL: &str = "left-panel";
const MODE_SELECT: &str = "mode-select";
const CALCULATION_INPUT: &str = "calculation-input";
const VARIABLE_ASSIGNMENT_INPUT: &str = "variable-assignment-input";
const INPUT_AREA: &str = "input-area";
const INPUT_AREA1: &str = "input-area1";
const INPUT_AREA2: &str = "input-area2";
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
        let expression = Expression::parse(input)?;
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
        let expected = "対応していない文字です";
        match Calculator::calculate_and_format(input) {
            Ok(_) => panic!("should be error"),
            Err(e) => assert_eq!(expected, e),
        }
    }

    #[test]
    fn parse_missing_operator() {
        let input = "1 2";
        let expected = "数値が連続しています";
        match Calculator::calculate_and_format(input) {
            Ok(_) => panic!("should be error"),
            Err(e) => assert_eq!(expected, e),
        }
    }

    #[test]
    fn parse_missing_right_operand() {
        let input = "1 + ";
        let expected = "構文解析に失敗しました";
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
    fn parse_paren_normal() {
        let input = "2 - (1 + 2) * 3";
        let expected = "2 - ( 1 + 2 ) * 3 → -7";
        let actual = Calculator::calculate_and_format(input).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn parse_paren_unary() {
        let input = "-(-2 + 1) * 3";
        let expected = "- ( - 2 + 1 ) * 3 → 3";
        let actual = Calculator::calculate_and_format(input).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn parse_paren_num_to_paren() {
        let input = "1 + 2(2 * 3)";
        let expected = "1 + 2 * ( 2 * 3 ) → 13";
        let actual = Calculator::calculate_and_format(input).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn parse_paren_paren_to_num() {
        let input = "(1 + 2)2 * 3";
        let expected = "( 1 + 2 ) * 2 * 3 → 18";
        let actual = Calculator::calculate_and_format(input).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn parse_paren_missing_right_paren() {
        let input = "1 + (2 * 3";
        let expected = "括弧が閉じられていません";
        match Calculator::calculate_and_format(input) {
            Ok(_) => panic!("should be error"),
            Err(e) => assert_eq!(expected, e),
        }
    }

    #[test]
    fn parse_paren_missing_left_paren() {
        let input = "1 + 2) * 3";
        let expected = "括弧の対応が取れていません";
        match Calculator::calculate_and_format(input) {
            Ok(_) => panic!("should be error"),
            Err(e) => assert_eq!(expected, e),
        }
    }

    #[test]
    fn parse_paren_missing_element() {
        let input = "1 + ()";
        let expected = "括弧の中に要素がありませんでした";
        match Calculator::calculate_and_format(input) {
            Ok(_) => panic!("should be error"),
            Err(e) => assert_eq!(expected, e),
        }
    }

    #[test]
    fn parse_fraction_normal() {
        let input = "1/2 + 3/4";
        let expected = "1 / 2 + 3 / 4 → 5 / 4";
        let actual = Calculator::calculate_and_format(input).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn parse_fraction_denominator_minus() {
        let input = "1/2 + 3/(-4)";
        let expected = "1 / 2 + 3 / ( - 4 ) → -1 / 4";
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
        let input = "-2147483647 - 2";
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
        let input = "(-2147483647 - 1) / (-1)";
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

    #[test]
    fn test_plus_normal() {
        let input = "+1";
        let expected = "+ 1 → 1";
        let actual = Calculator::calculate_and_format(input).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_minus_normal() {
        let input = "-1";
        let expected = "- 1 → -1";
        let actual = Calculator::calculate_and_format(input).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_minus_not_supported() {
        let input = "-2147483648";
        let expected = "数値に変換できません";
        match Calculator::calculate_and_format(input) {
            Ok(_) => panic!("should be error"),
            Err(e) => assert_eq!(expected, e),
        }
    }

    #[test]
    fn test_pow_normal() {
        let input = "2 ^ 3";
        let expected = "2 ^ 3 → 8";
        let actual = Calculator::calculate_and_format(input).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_pow_fraction() {
        let input = "(3 / 2) ^ 3";
        let expected = "( 3 / 2 ) ^ 3 → 27 / 8";
        let actual = Calculator::calculate_and_format(input).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_pow_exp_zero() {
        let input = "2 ^ 0";
        let expected = "2 ^ 0 → 1";
        let actual = Calculator::calculate_and_format(input).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_pow_zero() {
        let input = "0 ^ 2";
        let expected = "0 ^ 2 → 0";
        let actual = Calculator::calculate_and_format(input).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_pow_zero_zero() {
        let input = "0 ^ 0";
        let expected = "0 ^ 0 → 1";
        let actual = Calculator::calculate_and_format(input).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_pow_priority() {
        let input = "-2 ^ 3 ^ 2 * 2 - 3";
        let expected = "- 2 ^ 3 ^ 2 * 2 - 3 → -131";
        let actual = Calculator::calculate_and_format(input).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_pow_negative() {
        let input = "2 ^ (-3)";
        let expected = "負数の累乗はできません";
        match Calculator::calculate_and_format(input) {
            Ok(_) => panic!("should be error"),
            Err(e) => assert_eq!(expected, e)
        }
    }

    #[test]
    fn test_pow_exp_fraction() {
        let input = "2 ^ (3 / 2)";
        let expected = "2の3 / 2乗は計算できません";
        match Calculator::calculate_and_format(input) {
            Ok(_) => panic!("should be error"),
            Err(e) => assert_eq!(expected, e)
        }
    }

    #[test]
    fn test_pow_overflow() {
        let input = "2147483647 ^ 2";
        let expected = "int32の範囲を超える累乗です";
        match Calculator::calculate_and_format(input) {
            Ok(_) => panic!("should be error"),
            Err(e) => assert_eq!(expected, e)
        }
    }

    #[test]
    fn test_pow_unary() {
        let input = "^ 2";
        let expected = "無効な演算子です";
        match Calculator::calculate_and_format(input) {
            Ok(_) => panic!("should be error"),
            Err(e) => assert_eq!(expected, e)
        }
    }
}
