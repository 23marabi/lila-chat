#[macro_use]
extern crate log;
#[macro_use] extern crate rocket;
use rocket::fairing::AdHoc;

mod auth;
mod user;
mod message;

#[launch]
fn rocket() -> _ {
    env_logger::init();
    info!("Started up rocket");
    let cors_fairing = AdHoc::on_response("CORS", |_, res| {
        Box::pin(async move {
            res.set_raw_header("Access-Control-Allow-Origin", "*");
        })
    });
    info!("Built CORS fairing");

    rocket::build()
        .mount(
            "/",
            routes![auth::index, auth::get_user, auth::register_user, auth::check_pin, auth::change],
        )
        .attach(cors_fairing)
}
