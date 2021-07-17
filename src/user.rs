use serde::{Deserialize, Serialize};

// Struct to store basic user data
#[derive(Clone, Serialize, Deserialize)]
pub struct User {
    pub name: String,
    pub pin_hashed: String,
}
