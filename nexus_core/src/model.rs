use chrono::{DateTime, Utc};
pub use serde::{Deserialize, Serialize};
use uuid::Uuid;

// #[derive(Debug, Serialize, Deserialize)]
// pub enum Request {
//     Login {
//         username: String,
//         password: String,
//     },
//     Logout {
//         username: String,
//     },
//     Message {
//         from: String,
//         to: String,
//         content: MessageContent,
//     },
// }

// #[derive(Serialize, Deserialize, Debug)]
// pub enum Response {
//     LoginResponse {
//         success: bool,
//         description: String,
//         user_id: Option<String>,
//     },
//     LogoutResponse {
//         success: bool,
//         description: String,
//     },
//     MessageResponse {
//         success: bool,
//         message_id: Option<String>,
//         description: String,
//     },
//     Error {
//         code: u16,
//         description: String,
//     },
// }

// #[derive(Serialize, Deserialize, Debug)]
// pub enum MessageContentType {
//     Text,
//     Image,
//     Audio,
//     Video,
// }

// #[derive(Serialize, Deserialize, Debug)]
// pub struct MessageContent {
//     r#type: MessageContentType,
//     content: Vec<u8>,
// }

// // 推送消息结构
// #[derive(Serialize, Deserialize, Debug)]
// pub struct PushMessage {
//     pub from: String,
//     pub to: String,
//     pub content: MessageContent,
//     pub message_id: String,
// }

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginResponse {
    pub is_success: bool,
    pub message: String,
    pub token: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Message {
    pub sender_id: Uuid,
    pub received_id: Uuid,
    pub timestamp: DateTime<Utc>,
    pub r#type: MessageType,
    pub content: Vec<u8>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum MessageType {
    Text,
    Image,
    Audio,
    Video,
}
