use crate::user::User;
use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize, Serialize)]
pub struct MessageInput<'r> {
    pub name: &'r str,
    pub body: &'r str,
    pub date: &'r str,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Message {
    pub id: Uuid,
    pub user: User,
    pub body: String,
    pub created_at: DateTime<Utc>,
}
