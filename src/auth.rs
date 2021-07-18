extern crate log;
use crate::file_io::{append_json, read_json, write_json};
use rocket::http::{Cookie, Cookies};
use crate::user::User;
use rocket_contrib::{json::Json, json::JsonValue, serve::StaticFiles};
extern crate sha1;

#[get("/")]
pub fn index() -> &'static str {
    "API Info:

    `POST /api/register/<name>/<pin>/<pronouns>` Register the username with the pin provided if it doesn't already exist

    `GET /api/users/<name>` Check if the user exists

    `GET /api/users/<name>/<pin>` Check if the user exists, and if the pin provided matches

    `POST /api/users/change/<name>/<pin>/<new-name>/<new-pin>` Change a users name and/or pin

    `GET /api/about/name/<name>` Get the name of a user

    `GET /api/about/pronouns/<name>` Get the pronouns of a user"
}

// Post request to register a user and pin
#[post("/register/<name>/<pin>/<pronouns>")]
pub fn register_user(name: String, pin: i32, pronouns: String) -> JsonValue {
    let mut users: Vec<User> = read_json(); // Create an array of users out of parsed json
    for i in &users {
        // loop through elements of the vector
        if i.name == name.to_lowercase() {
            warn!("Cannot create user {}! User is already in system.", i.name);
            return json!({
                "status": "fail",
                "reason": "user already exists",
            });
        };
    }

    let pin_hashed = sha1::Sha1::from(&pin.to_string()).digest().to_string(); // hash the pin

    users.push(User {
        name: name.to_string().to_lowercase(),
        pin_hashed: pin_hashed,
        pronouns: pronouns.to_string().to_lowercase(),
        session_token: "NULL".to_string(),
    }); // append the user to the vec

    // append to the json file
    match append_json(&users[users.len()-1]) {
        Err(why) => panic!("couldn't append json: {}", why),
        Ok(()) => info!("Succesfully appended to json"),
    };

    info!(
        "succesfully created user {} with pin hash {}",
        users[users.len() - 1].name.to_string(),
        users[users.len() - 1].pin_hashed
    );
    return json!({
        "status": "ok",
        "reason": format!("user {} registered", users[users.len()-1].name.to_string().to_lowercase()),
    });
}

fn create_token(name: String, mut users: Vec<User>) -> String {
    for i in 0..users.len() {
        if users[i].name == name {
            users[i].session_token = "token".to_string();
            append_json(&users[i]);
            info!("succesfully created token for user {}", name);
            let token = users[i].session_token.clone();
            return token;
        };
    };
    return "NULL".to_string();
}

// Check if pin matches user
#[get("/users/<name>/<pin>")]
pub fn check_pin(name: String, pin: i32) -> JsonValue {
    let users: Vec<User> = read_json();
    let hashed_pin_input = sha1::Sha1::from(&pin.to_string()).digest().to_string();
    for i in &users {
        // loop through the vector
        if i.name == name.to_lowercase() {
            if i.pin_hashed == hashed_pin_input {
                info!("pin correct for user {}", i.name);
                // Create token for user & set a cookie
                return json!({
                    "status": "ok",
                    "reason": "pin matches",
                });
            } else {
                warn!("pin incorrect for user {}", i.name);
                return json!({
                    "status": "fail",
                    "reason": "incorrect pin",
                });
            };
        };
    }
    warn!(
        "cannot check pin for user {} as they do not exist",
        name.to_string().to_lowercase()
    );
    return json!({
        "status": "fail",
        "reason": format!("user {} doesn't exist", name.to_string().to_lowercase()),
    });
}

// Change a users pin/name
#[post("/users/change/<name>/<pin>/<new_name>/<new_pin>")]
pub fn change(name: String, pin: i32, new_name: String, new_pin: i32) -> JsonValue {
    let mut users: Vec<User> = read_json();

    let hashed_pin_input = sha1::Sha1::from(&pin.to_string()).digest().to_string();

    // Loop over elements in vector
    for i in 0..users.len() {
        if users[i].name == name.to_lowercase() {
            // make sure name exists
            if users[i].pin_hashed == hashed_pin_input {
                // check if pin is correct
                // Check wether to change name or name+pin
                if users[i].name == new_name.to_lowercase() {
                    // check if new name already exists
                    users[i].pin_hashed = sha1::Sha1::from(&new_pin.to_string()).digest().to_string();
                    match write_json(&users) {
                        Err(why) => panic!("Cannot write to json! {}", why),
                        Ok(()) => info!("succesfully wrote to json file"),
                    }
                    info!("Changed pin of {}", name.to_string().to_lowercase());
                    return json!({
                        "status": "ok",
                        "reason": format!("changed {}'s pin", name.to_string().to_lowercase()),
                    });
                } else {
                    // check if new name already exists
                    for n in &users {
                        if n.name == new_name.to_lowercase() {
                            warn!(
                                "Could not change name of {} to {}, as new name is already taken.",
                                name.to_lowercase(),
                                new_name.to_lowercase()
                            );
                            return json!({
                                "status": "fail",
                                "reason": format!("new name {} is already taken", new_name.to_lowercase()),
                            });
                        }
                    }
                    users[i].name = new_name.to_string().to_lowercase();
                    users[i].pin_hashed =
                        sha1::Sha1::from(&new_pin.to_string()).digest().to_string();

                    match write_json(&users) {
                        Err(why) => panic!("couldn't write to json file! {}", why),
                        Ok(()) => info!("succesfully wrote to json file"),
                    }
                    info!(
                        "Changed name of {} to {}. New pin hash is {}",
                        name.to_string(),
                        users[i].name.to_string(),
                        users[i].pin_hashed.to_string()
                    );
                    return json!({
                        "status": "ok",
                        "reason": "successfully changed name and/or pin",
                    });
                }
            } else {
                warn!("Incorrect pin given for user {}!", name.to_string());
                return json!({
                    "status": "fail",
                    "reason": "incorrect pin for user",
                });
            }
        }
    }
    warn!(
        "User {} not found, could not change pin and/or name.",
        name.to_string()
    );
    return json!({
        "status": "fail",
        "reason": format!("user {} not found", name.to_string().to_lowercase()),
    });
}

#[get("/users/<name>")]
pub fn get_user(name: String) -> JsonValue {
    let users: Vec<User> = read_json();
    let found_user = users
        .iter()
        .filter(|u| u.name == name.to_lowercase())
        .next();

    match found_user {
        Some(user) => json!({
            "status":"ok",
            "user": {
                "name": user.name,
                "pronouns": user.pronouns,
            },
        }),
        None => json!({
            "status": "fail",
            "reason": format!("user {} not found", name),
        }),
    }
}
