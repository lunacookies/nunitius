pub mod server;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Event {
    Message(Message),
    Login(User),
    Logout(User),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub body: String,
    pub author: User,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub nickname: String,
    pub color: Option<Color>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Color {
    Red,
    Green,
    Yellow,
    Blue,
    Purple,
    Cyan,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginResponse {
    pub nickname_taken: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ConnectionKind {
    Sender,
    Viewer,
}
