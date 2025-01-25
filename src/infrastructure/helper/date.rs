use crate::domain::error::date::DateTimeParseError;
use regex::Regex;

// Pattern: YYYY-MM-DD H:i, example: 2025-01-16.
pub fn parse_yyyy_mm_dd_hm_from_str(s: &str)
    -> Result<chrono::NaiveDateTime, DateTimeParseError>
{
    let regex_ymd_t_hm = Regex::new(r"(\d{4}-\d{2}-\d{2}T\d{2}:\d{2})").unwrap();
    let regex_ymd_hm = Regex::new(r"(\d{4}-\d{2}-\d{2} \d{2}:\d{2})").unwrap();

    for capture in regex_ymd_t_hm.captures_iter(s.trim()) {
        if let Ok(date) = chrono::NaiveDateTime::parse_from_str(capture.get(1).unwrap().as_str(), "%Y-%m-%dT%H:%M") {
            return Ok(date);
        }
    }

    for capture in regex_ymd_hm.captures_iter(s.trim()) {
        if let Ok(date) = chrono::NaiveDateTime::parse_from_str(capture.get(1).unwrap().as_str(), "%Y-%m-%d %H:%M") {
            return Ok(date);
        }
    }

    Err(DateTimeParseError::new())
}