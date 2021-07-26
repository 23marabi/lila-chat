/* Contains Rocket code for chat/message functionality */
extern crate log;
use crate::file_io::db_read_user;
use crate::message::{Message, MessageInput, MessageType};
use crate::user::User;
use chrono::prelude::*;
use futures_util::StreamExt;
use once_cell::sync::Lazy;
use rocket::http::{Cookie, Cookies};
use rocket_contrib::json::{Json, JsonValue};
use std::sync::Mutex;
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::broadcast;
use uuid::Uuid;

static MESSAGES: Lazy<Mutex<Vec<Message>>> = Lazy::new(|| Mutex::new(Vec::new()));

#[get("/message/messages.json")]
pub fn fetch_messages() -> Json<Vec<Message>> {
    let messages = {
        let messages = MESSAGES.lock().unwrap();
        messages.to_vec()
    };
    Json(messages)
}

async fn send_message_to_websocket() {
    let (tx, mut rx1) = broadcast::channel(16);
    let mut rx2 = tx.subscribe();
    tokio::spawn(async move {
        info!("message: {}", rx1.recv().await.unwrap());
    });
    tx.send(10).unwrap();
}

// Create full message object and write to file
fn create_message(message: Json<MessageInput>, user: &User) -> JsonValue {
    let event_type = match message.body.chars().nth(0).unwrap() {
        '/' => MessageType::Command,
        ':' => MessageType::Emote,
        _ => MessageType::Normal,
    };

    if (message.body == "") | (message.body == " ") {
        warn!("blank message given");
        return json!({
            "status": "fail",
            "reason": "blank message",
        });
    }

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


    std::thread::spawn(|| send_message_to_websocket());
    /*
    // append message to file
    let mut messages = MESSAGES.lock().unwrap();
    messages.push(message_obj.to_owned());
    */
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
        } else if user.session_token == token.value() {
            // if token matches
            info!("user exists and given token matches");
            return create_message(message, &user);
        } else {
            warn!("token does not match!");
            return json!({
                "status": "fail",
                "reason": "token does not match",
            });
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
        }
        Some(token) => token,
    };
    check_token(token, message)
}

// Websocket Stuff //

pub async fn accept_connection(stream: TcpStream) {
    let addr = stream
        .peer_addr()
        .expect("connected streams should have a peer address");
    info!("Peer address: {}", addr);

    let ws_stream = tokio_tungstenite::accept_async(stream)
        .await
        .expect("Error during the websocket handshake occurred");

    info!("New WebSocket connection: {}", addr);

    let (tx, mut rx1) = broadcast::channel::<usize>(16);
    let mut rx2 = tx.subscribe();
    
    loop {
        let message = rx2.recv().await.unwrap();
        info!("message recieved: {}", message);
    }
}
