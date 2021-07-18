use uuid::Uuid;
use chrono::prelude::*;
use crate::user::User;
use rocket::serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Message<'r> {
    pub id: Uuid,
    pub user: User,
    pub body: &'r str,
    pub created_at: DateTime<Utc>,
}
