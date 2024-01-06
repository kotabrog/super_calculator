mod setup;
mod format;
mod history;

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
        if let Some(plus_index) = input.find('+') {
            let (left, right) = input.split_at(plus_index);
            let x = left.trim().parse::<i32>().map_err(|_| "左側の数値の解析に失敗しました")?;
            let y = right[1..].trim().parse::<i32>().map_err(|_| "右側の数値の解析に失敗しました")?;
            match x.checked_add(y) {
                Some(result) => Ok(format!("{} + {} → {}", x, y, result)),
                None => Err("int32の範囲を超える加算です".to_string()),
            }
        } else {
            Err("式に '+' が含まれていません".to_string())
        }
    }
}
