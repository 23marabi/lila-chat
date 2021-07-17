/* Contains Rocket code for chat/message functionality */
extern crate log;
extern crate rocket_contrib;
use crate::message::Message;
extern crate serde_derive;
use rocket_contrib::json::{Json, JsonValue};
use rocket::{data::FromData, response::Responder};

#[post("/api/message/send", format = "json", data = "<message>")]
pub fn send_message(message: Json<Message>) -> JsonValue {
    json!({
        "status": "ok",
        "reason": "bruh"
    })
}
