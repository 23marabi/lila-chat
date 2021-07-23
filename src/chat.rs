/* Contains Rocket code for chat/message functionality */
extern crate log;
use once_cell::sync::Lazy;
use std::sync::Mutex;
use crate::file_io::db_read_user;
use rocket::http::{Cookie, Cookies};
use crate::message::{Message, MessageInput, MessageType};
use rocket_contrib::json::{Json, JsonValue};
use chrono::prelude::*;
use uuid::Uuid;
use crate::user::User;

static MESSAGES: Lazy<Mutex<Vec<Message>>> = Lazy::new(|| Mutex::new(Vec::new()));

#[get("/message/messages.json")]
pub fn fetch_messages() -> Json<Vec<Message>> {
    let messages = {
        let messages = MESSAGES.lock().unwrap();
        messages.to_vec()
    };
    Json(messages)
}

// Create full message object and write to file
fn create_message(message: Json<MessageInput>, user: &User) -> JsonValue {
    let event_type = match message.body.chars().nth(0).unwrap() {
        '/' => MessageType::Command,
        ':' => MessageType::Emote,
        _ => MessageType::Normal,
    };

    // create full message object
    let message_obj: Message = Message {
        id: Uuid::new_v4(),
        event_type,
        user: user.name.to_lowercase().to_owned(),
        body: message.body.to_string(),
        created_at: Utc::now(),
    };
    info!("created mesage: {:?}", message_obj);
    info!("Date is: {}", message_obj.created_at.to_rfc2822());

    // append message to file
    let mut messages = MESSAGES.lock().unwrap();
    messages.push(message_obj.to_owned());
    return json!({
        "status": "ok",
        "reason": "message created",
    });
}

// Check if user can create the message, and then create more info about the message
fn check_token(token: Cookie, message: Json<MessageInput<'_>>) -> JsonValue {
    if let Some(user) = db_read_user(&message.name.to_lowercase()).ok().flatten() {
        // check if token is correct for name given
        if token.value() == "NULL" {
            warn!("NULL token!");
            return json!({
                "status": "fail",
                "reason": "NULL token",
            });
        } else if user.session_token == token.value() { // if token matches
            info!("user exists and given token matches");
            return create_message(message, &user);
        } else {
            warn!("token does not match!");
            return json!({
                "status": "fail",
                "reason": "token does not match",
            })
        };
    };
    warn!("user not found");
    json!({
        "status": "fail",
        "reason": "user not found"
    })
}

// Receive a basic message
#[post("/message/send", format = "json", data = "<message>")]
pub fn send_message(message: Json<MessageInput<'_>>, mut cookies: Cookies) -> JsonValue {
    let token = match cookies.get_private("token") {
        None => {
            warn!("couldn't get token cookie!");
            return json!({
                "status": "fail",
                "reason": "could not read cookie",
            });
        },
        Some(token) => token,
    };
    check_token(token, message)
}

// Delete a message
/*
#[post("/message/delete", format = "json", data = "<message>")]
pub fn delete_message(message: Json<MessageInput<'_>>, mut cookies: Cookies) -> JsonValue {
    let token = match cookies.get_private("token") {
        None => {
            warn!("couldn't get token cookie!");
            return json!({
                "status": "fail",
                "reason": "could not read cookie",
            });
        },
        Some(token) => token,
    };
}
*/
