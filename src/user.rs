use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub enum UserType {
    Normal,
    Moderator,
    Admin,
}

// Struct to store basic user data
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct User {
    pub name: String,
    pub pin_hashed: String,
    pub pronouns: String,
    pub session_token: String,
    pub role: UserType,
}
