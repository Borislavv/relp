use regex::Regex;
use crate::infrastructure::service::command::error::DateTimeParseError;

pub fn parse_date_from_str(s: &str) -> Result<chrono::NaiveDateTime, DateTimeParseError> {
    let regex = Regex::new(r"(\d{4}-\d{2}-\d{2} \d{2}:\d{2})").unwrap();

    for capture in regex.captures_iter(s.trim()) {
        if let Ok(date) = chrono::NaiveDateTime::parse_from_str(
            capture.get(1).unwrap().as_str(), "%Y-%m-%d %H:%M"
        ) {
            return Ok(date);
        }
    }
    Err(DateTimeParseError::new())
}