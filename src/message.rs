use uuid::Uuid;
use chrono::prelude::*;
use crate::user::User;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Message {
    /*
    pub id: Uuid,
    pub user: User,
    pub body: String,
    pub created_at: DateTime<Utc>, */
    id: u8,
    body: String,
}
