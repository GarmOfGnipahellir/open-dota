use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum ClientMessage {
    Join,
    Leave,
    ChatMessage { message: String },
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ServerMessage {
    InitClient,
    ChatMessage { message: String },
}
