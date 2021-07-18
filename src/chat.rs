/* Contains Rocket code for chat/message functionality */
extern crate log;
use crate::message::Message;
use rocket_contrib::json::{Json, JsonValue};

#[post("/api/message/send", format = "json", data = "<message>")]
pub fn send_message(message: Json<Message<'_>>) -> JsonValue {
    json!({
        "status": "ok",
        "reason": "bruh"
    })
}
