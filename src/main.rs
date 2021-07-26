#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate log;
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
use chat::accept_connection;
use rocket::fairing::AdHoc;
use rocket_contrib::serve::StaticFiles;
use std::io::Error;
use tokio::net::{TcpListener, TcpStream};

mod auth;
mod chat;
mod file_io;
mod message;
mod user;

#[tokio::main]
async fn startup_websocket() -> Result<(), Error> {
    let addr = "127.0.0.1:1312".to_string();
    // Create the event loop and TCP listener we'll accept connections on.
    let try_socket = TcpListener::bind(&addr).await;
    let listener = try_socket.expect("Failed to bind");
    info!("Listening on: {}", addr);

    while let Ok((stream, _)) = listener.accept().await {
        tokio::spawn(accept_connection(stream));
    }

    Ok(())
}

fn main() {
    env_logger::init();

    // Start thread for tungstenite websockets
    std::thread::spawn(|| startup_websocket());

    let cors_fairing = AdHoc::on_response("CORS", |_, res| {
        res.set_raw_header("Access-Control-Allow-Origin", "http://localhost:8000");
    });
    info!("Built CORS fairing");
    info!("Started up rocket");
    rocket::ignite()
        .mount(
            "/api",
            routes![
                auth::get_user,
                auth::register,
                auth::login,
                chat::send_message,
                chat::fetch_messages,
                auth::change_info,
                auth::check_token,
                auth::logout,
                auth::moderation_actions
            ],
        )
        .mount("/", StaticFiles::from("frontend"))
        .attach(cors_fairing)
        .launch();
}
