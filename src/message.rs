use crate::user::User;
use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize, Serialize)]
pub struct Message<'r> {
    pub id: Uuid,
    pub user: User,
    pub body: &'r str,
    pub created_at: DateTime<Utc>,
}
