/* Contains Rocket code for chat/message functionality */
extern crate log;
use once_cell::sync::Lazy;
use std::sync::Mutex;
use crate::file_io::db_read;
use rocket::http::{Cookie, Cookies};
use crate::message::{Message, MessageInput};
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
fn create_message(message: Json<MessageInput>, file: &str, user: &User) -> JsonValue {
    // create full message object
    // append message to file

    // Create proper datetime format out of string
    let date_split: Vec<&str> = message.date.split("-").collect();

    let year: i32 = match date_split[0].trim().parse() { // extract year
        Err(why) => panic!("could not extract year from given date: {}", why),
        Ok(year) => year,
    };

    let month: u32 = match date_split[1].trim().parse() { // extract month
        Err(why) => panic!("could not extract month from given date: {}", why),
        Ok(month) => month,
    };

    let day: u32 = match date_split[2].trim().parse() { // extract day
        Err(why) => panic!("could not extract day from given date: {}", why),
        Ok(month) => month,
    };

    let date: DateTime<Utc> = Utc.ymd(year, month, day).and_hms(9, 10, 11);
    let message_obj: Message = Message {
        id: Uuid::new_v4(),
        user: user.name.to_owned(),
        body: message.body.to_string(),
        created_at: date,
    };
    info!("created mesage: {:?}", message_obj);
    let mut messages = MESSAGES.lock().unwrap();
    messages.push(message_obj.to_owned());
    return json!({
        "status": "ok",
        "reason": "message created",
    });
}

// Check if user can create the message, and then create more info about the message
fn check_token(token: Cookie, message: Json<MessageInput<'_>>) -> JsonValue {
    // check if token is correct for name given
    let users: Vec<User> = db_read(); // create vector out of users in json file
    for i in &users {
        // loop through elements
        if i.name == message.name.to_lowercase() { // if it finds the user in the file
            if i.session_token == token.value() { // if token matches
                info!("user exists and given token matches");
                return create_message(message, "messages.json", i);
            } else {
                warn!("token does not match!");
                return json!({
                    "status": "fail",
                    "reason": "token does not match"
                })
            };
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
    let token = cookies.get_private("token").unwrap();
    check_token(token, message)
}
