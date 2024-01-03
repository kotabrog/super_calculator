use super::Calculator;

impl Calculator {
    pub(super) fn format_input(input: &str) -> String {
        input.chars().filter(|c| !c.is_whitespace()).collect::<String>()
    }
}
