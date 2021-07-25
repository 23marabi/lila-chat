use serde::{Deserialize, Serialize};
use uuid::Uuid;

/* User Data */
// enum of different user types
#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub enum UserType {
    Normal,
    Moderator,
    Admin,
}

// Struct to store basic user data
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct User {
    pub name: String, // unique username
    pub pin_hashed: String, // sha1 hash of the pin
    pub pronouns: String, // user's pronouns
    pub session_token: String, // generated session token
    pub role: UserType, // type/role of user
    pub id: Uuid,
}

/* Moderation Data */
// enum of different moderator actions
#[derive(Deserialize, Debug)]
pub enum ModActions {
    Kick, // Log the user out of their current session
    Ban, // Remove the user
    Demote, // Demote a user to a lower role
    Premote, // Premote a user to a higher role
}

// struct to use for json input
#[derive(Deserialize, Debug)]
pub struct ModerationAction {
    pub name: String, // name of the moderator
    pub action: ModActions, // what action to take
    pub target: String, // who to take the action on
}

/* Miscellaneous Events */
// logout event struct
#[derive(Deserialize, Debug)]
pub struct LogoutEvent {
    pub name: String,
}

// register event struct
#[derive(Deserialize, Debug)]
pub struct RegisterEvent {
    pub name: String,
    pub pin: String,
    pub pronouns: String,
}

// login event struct
#[derive(Deserialize, Debug)]
pub struct LoginEvent {
    pub name: String,
    pub pin: String,
}

// change event type
#[derive(Deserialize, Debug)]
pub enum ChangeEventType {
    Name,
    Pin,
    Pronouns,
}

// change info event struct
#[derive(Deserialize, Debug)]
pub struct ChangeEvent {
    pub name: String, // name of the user
    pub changed_event: ChangeEventType, // which event to change
    pub new_event: String, // the new value for the event
}
