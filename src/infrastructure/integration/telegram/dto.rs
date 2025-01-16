use serde::{Deserialize, Serialize};

// List of updates.
#[derive(Serialize, Deserialize, Debug)]
pub struct GetUpdatesResponse {
    pub ok: bool,
    pub result: Vec<Update>,
}
// SendMessageResponse struct present a telegram response on sendMessage method.
#[derive(Serialize, Deserialize, Debug)]
pub struct SendMessageResponse {
    pub ok: bool,
    pub result: Message,
}

// Update is a single message structure. Telegram sends a list of Update structs.
#[derive(Serialize, Deserialize, Debug)]
pub struct Update {
    pub update_id: i64,
    pub message: Option<Message>,
    pub edited_message: Option<Message>,
}
// Message details.
#[derive(Serialize, Deserialize, Debug)]
pub struct Message {
    pub message_id: i64,
    pub from: User,
    pub chat: Chat,
    pub date: i64,
    pub text: String,
}
// User details.
#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    pub id: i64,
    pub is_bot: bool,
    pub first_name: String,
    pub username: String,
    pub language_code: Option<String>,
    pub is_premium: Option<bool>,
}
// Chat details.
#[derive(Serialize, Deserialize, Debug)]
pub struct Chat {
    pub id: i64,
    pub first_name: String,
    pub username: String,
    pub r#type: String,
}

