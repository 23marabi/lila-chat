use uuid::Uuid;
use chrono::prelude::*;
use crate::user::User;

#[derive(Clone)]
pub struct Message {
    pub id: Uuid,
    pub user: User,
    pub body: String,
    pub created_at: DateTime<Utc>,
}
