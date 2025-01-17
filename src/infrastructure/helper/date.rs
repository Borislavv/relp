use regex::Regex;
use crate::domain::error::date::DateTimeParseError;

// Pattern: YYYY-MM-DD H:i, example: 2025-01-16.
pub fn parse_yyyy_mm_dd_hm_from_str(s: &str)
    -> Result<chrono::NaiveDateTime, DateTimeParseError>
{
    let regex = Regex::new(r"(\d{4}-\d{2}-\d{2}T\d{2}:\d{2})").unwrap();

    for capture in regex.captures_iter(s.trim()) {
        if let Ok(date) = chrono::NaiveDateTime::parse_from_str(
            capture.get(1).unwrap().as_str(), "%Y-%m-%dT%H:%M"
        ) {
            return Ok(date);
        }
    }
    Err(DateTimeParseError::new())
}