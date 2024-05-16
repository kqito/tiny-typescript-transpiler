use regex::Regex;

pub const SPACE_REG: &str = r"[[:space:]]";
pub const NUMBER_REG: &str = r"[0-9]";
pub const STRING_REG: &str = r"[_a-zA-Z0-9]";

pub(crate) fn is_match_char(char: char, re: &str) -> bool {
    let regex = Regex::new(re).unwrap();
    regex.is_match(&char.to_string())
}
