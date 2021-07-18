/* Contains Rocket code for chat/message functionality */
extern crate log;
use crate::file_io::{read_json};
use crate::message::{Message, MessageInput};
use rocket_contrib::json::{Json, JsonValue};
use chrono::prelude::*;
use uuid::Uuid;
use crate::user::User;

// Check if user can create the message, and then create more info about the message
fn create_message(message: Json<MessageInput>, file: &str) -> JsonValue {
    // check if token is correct for name given
    let users: Vec<User> = read_json(); // create vector out of users in json file

    for i in &users {
        // loop through elements
        if i.name == message.name.to_lowercase() { // if it finds the user in the file
            if i.session_token == message.token { // if token matches
                info!("user exists and given token matches");
                return json!({
                    "status": "ok",
                    "reason": "token matches"
                })
            } else {
                warn!("token does not match!");
                return json!({
                    "status": "fail",
                    "reason": "token does not match"
                })
            };
        };
    };
    json!({
        "status": "fail",
        "reason": "user not found"
    })
/*
    // create full message object
    // append message to file

    let date_split: Vec<&str> = message.date.split("-").collect();
    let year: i32 = match date_split[0].trim().parse() {
        Err(why) => panic!("could not extract year from given date: {}", why),
        Ok(year) => year,
    };
    let date: DateTime<Utc> = Utc.ymd(year, 7, 7).and_hms(9, 10, 11);
    let message_obj: Message = Message {
        id: Uuid::new_v4(),
        user: User { name: message.name.to_string(), pin_hashed: "no".to_string(), pronouns: "she/her".to_string(), session_token: "NULL".to_string() },
        body: message.body.to_string(),
        created_at: Utc.ymd(2005, 7, 8).and_hms(9, 10, 11),
    };
    println!("{:?}", message_obj);*/
}

// Receive a basic message
#[post("/api/message/send", format = "json", data = "<message>")]
pub fn send_message(message: Json<MessageInput<'_>>) -> JsonValue {
    create_message(message, "messages.json")
}
