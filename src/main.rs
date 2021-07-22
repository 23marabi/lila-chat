#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate log;
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
use rocket_contrib::serve::StaticFiles;

use rocket::fairing::AdHoc;

mod auth;
mod chat;
mod file_io;
mod message;
mod user;

fn main() {
    env_logger::init();
    info!("Started up rocket");
    let cors_fairing = AdHoc::on_response("CORS", |_, res| {
        res.set_raw_header("Access-Control-Allow-Origin", "*");
    });
    info!("Built CORS fairing");

    rocket::ignite()
        .mount(
            "/api",
            routes![
                auth::index,
                auth::get_user,
                auth::register_user,
                auth::check_pin,
                auth::change,
                chat::send_message,
                chat::fetch_messages,
                auth::change_info,
                auth::check_token
            ],
        )
        .mount("/", StaticFiles::from("frontend"))
        .attach(cors_fairing)
        .launch();
}
