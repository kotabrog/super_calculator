mod setup;
mod format;
mod history;

use anyhow::Result;

const INPUT_AREA: &str = "input-area";
const FORMATTED_DISPLAY: &str = "formatted-display";
const HISTORY_CONTAINER: &str = "history-container";
const HISTORY_ITEM: &str = "history-item";

pub struct Calculator {}

impl Calculator {
    fn calculate_and_format(input: &str) -> Result<String, String> {
        if let Some(plus_index) = input.find('+') {
            let (left, right) = input.split_at(plus_index);
            let x = left.trim().parse::<i32>().map_err(|_| "左側の数値の解析に失敗しました")?;
            let y = right[1..].trim().parse::<i32>().map_err(|_| "右側の数値の解析に失敗しました")?;
    
            Ok(format!("{} + {} → {}", x, y, x + y))
        } else {
            Err("式に '+' が含まれていません".to_string())
        }
    }
}
