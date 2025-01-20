use chrono::NaiveDateTime;

pub trait Event {
    // key is the event batch identifier for extract (Sender, Receiver)
    fn key(&self) -> String;
    // scheduled_for is the date for which created this event
    fn scheduled_for(&self) -> NaiveDateTime;
    // is_ready automatically checks the current event is ready for sending
    fn is_ready(&self) -> bool;
    // created_at is the date when the event was born
    fn created_at(&self) -> NaiveDateTime;
}