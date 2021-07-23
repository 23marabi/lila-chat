use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize, Serialize)]
pub struct MessageInput<'r> {
    pub name: &'r str,
    pub body: &'r str,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum MessageType {
    Normal,
    Announcement,
    Emote,
    Command,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Message {
    pub id: Uuid,
    pub event_type: MessageType,
    pub user: String,
    pub body: String,
    pub created_at: DateTime<Utc>,
}
