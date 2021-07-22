extern crate log;
use crate::file_io::{db_add, db_write, db_read};
use rocket::http::{Cookie, Cookies, SameSite};
use crate::user::User;
use rocket_contrib::json::{Json, JsonValue};
use random_string::generate;
extern crate sha1;
use serde::Deserialize;

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
    let mut users: Vec<User> = db_read(); // Create an array of users out of parsed json
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

    let new_user: User = User {
        name: name.to_string().to_lowercase(),
        pin_hashed: pin_hashed,
        pronouns: pronouns.to_string().to_lowercase(),
        session_token: "NULL".to_string(),
    }; // append the user to the vec

    /*
    // append to the json file
    match append_json(&new_user) {
        Err(why) => panic!("couldn't append json: {}", why),
        Ok(()) => info!("Succesfully appended to json"),
    };*/
    db_add(&new_user);

    info!(
        "succesfully created user {} with pin hash {}",
        new_user.name.to_string(),
        new_user.pin_hashed
    );
    return json!({
        "status": "ok",
        "reason": format!("user {} registered", new_user.name.to_string().to_lowercase()),
    });
}

fn create_token(name: String, mut users: Vec<User>) -> String {
    let charset = "1234567890abcdefghijklmnopqrstuvwxyz";

    for i in 0..users.len() {
        if users[i].name == name {
            users[i].session_token = generate(12, charset);
            /*
            match write_json(&users) {
                Err(why) => panic!("coudln't write to file: {}", why),
                Ok(()) => info!("succesfully wrote to file"),
            };*/
            db_write(&users);
            info!("succesfully created token for user {}", name);
            let token = users[i].session_token.clone();
            return token;
        };
    };
    warn!("something bad happened while creating a token and idk what");
    return "NULL".to_string();
}

// Check if user is properly logged in
#[get("/token/<name>")]
pub fn check_token(name: String, mut cookies: Cookies) -> JsonValue {
    let users: Vec<User> = db_read();
    for i in &users {
        if i.name == name.to_lowercase() {
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
            if token.value() == "NULL" {
                warn!("NULL token!");
                return json!({
                    "status": "fail",
                    "reason": "NULL token",
                });
            } else if token.value() == i.session_token {
                info!("user {} has correct session token", name);
                return json!({
                    "status": "ok",
                    "reason": "correct token",
                });
            } else {
                info!("user {} has incorrect token!", name);
                return json!({
                    "status": "fail",
                    "reason": "incorrect token",
                });
            }
        }
    }
    warn!("user {} not found", name);
    return json!({
        "status": "fail",
        "reason": "user not found",
    });
}

// logout event struct
#[derive(Deserialize, Debug)]
pub struct LogoutEvent {
    pub name: String,
}

// Logout API
#[post("/logout", format = "json", data = "<info>")]
pub fn logout(info: Json<LogoutEvent>, mut cookies: Cookies) -> JsonValue {
    let mut users: Vec<User> = db_read();
    for i in 0..users.len() {
        if info.name.to_lowercase() == users[i].name {
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
            if token.value() == "NULL" {
                warn!("NULL token!");
                return json!({
                    "status": "fail",
                    "reason": "NULL token",
                });
            } else if token.value() == users[i].session_token {
                cookies.remove_private(Cookie::named("token"));
                users[i].session_token = "NULL".to_string();
                info!("logged out user {}", info.name);
                return json!({
                    "status": "ok",
                    "reason": "logged out",
                });
            } else {
                warn!("token does not match! cannot logout");
                return json!({
                    "status": "fail",
                    "reason": "token does not match",
                });
            }
        }
    }
    warn!("logged out user {}, user not found", info.name);
    return json!({
        "status": "fail",
        "reason": "user not found",
    });
}

// Check if pin matches user
#[get("/users/<name>/<pin>")]
pub fn check_pin(mut cookies: Cookies, name: String, pin: i32) -> JsonValue {
    let users: Vec<User> = db_read();
    let hashed_pin_input = sha1::Sha1::from(&pin.to_string()).digest().to_string();
    for i in &users {
        // loop through the vector
        if i.name == name.to_lowercase() {
            if i.pin_hashed == hashed_pin_input {
                info!("pin correct for user {}", i.name);

                // Create token for user & set a cookie
                let token = create_token(i.name.clone(), users);
                let cookie = Cookie::build("token", token)
                    .path("/")
                    .finish();
                cookies.remove_private(Cookie::named("token"));
                cookies.add_private(cookie);
                info!("set the token cookie");

                return json!({
                    "status": "ok",
                    "reason": "pin matches",
                });
            } else {
                cookies.remove_private(Cookie::named("token"));
                info!("removed private cookie");
                warn!("pin incorrect for user {}", i.name);
                return json!({
                    "status": "fail",
                    "reason": "incorrect pin",
                });
            };
        };
    }
    cookies.remove_private(Cookie::named("token"));
    info!("removed private cookie");
    warn!(
        "cannot check pin for user {} as they do not exist",
        name.to_string().to_lowercase()
    );
    return json!({
        "status": "fail",
        "reason": format!("user {} doesn't exist", name.to_string().to_lowercase()),
    });
}

#[derive(Deserialize, Debug)]
pub struct ChangeEvent {
    pub name: String,
    pub pin: String,
    pub changed_event: String,
    pub new_event: String,
}

// Change info about a user
#[post("/users/change", format = "json", data = "<input>")]
pub fn change_info(input: Json<ChangeEvent>, mut cookies: Cookies) -> JsonValue {
    // read in the users & hash the pin
    let mut users: Vec<User> = db_read();
    
    // get token from cookie
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
    if token.value() == "NULL" {
        warn!("NULL token!");
        return json!({
            "status": "fail",
            "reason": "NULL token",
        });
    }

    // loop through the users
    for i in 0..users.len() {
        if input.name.to_lowercase() == users[i].name { // if user found...
            if token.value() == users[i].session_token { // & if token matches:
                if input.changed_event == "name" {
                    // change the name
                    users[i].name = input.new_event.clone();
                    info!("changed name of {} to {}", input.name, input.new_event);
                    db_write(&users);
                    return json!({
                        "status": "ok",
                        "reason": format!("changed name of {} to {}", input.name, input.new_event),
                    });
                } else if input.changed_event == "pin" {
                    // change the pin
                    let new_hashed_pin = sha1::Sha1::from(&input.new_event).digest().to_string();
                    users[i].pin_hashed = new_hashed_pin.clone();
                    db_write(&users);
                    info!("changed pin of {}", input.name);
                    return json!({
                        "status": "ok",
                        "reason": "changed pin",
                    });
                } else if input.changed_event == "pronouns" {
                    // change the pronouns
                    users[i].pronouns = input.new_event.clone();
                    info!("changed pronouns of {} to {}", input.name, input.new_event);
                    db_write(&users);
                    return json!({
                        "status": "ok",
                        "reason": "successfully changed pronouns",
                    });
                };
            } else {
                warn!("incorrect pin for user {}", input.name);
                return json!({
                    "status": "fail",
                    "reason": "incorrect pin",
                });
            };
        };
    };
    warn!("couldn't change users info, user does not exist");
    return json!({
        "status": "fail",
        "reason": "user doesn't exist",
    });
}

// Change a users pin/name
#[post("/users/change/<name>/<pin>/<new_name>/<new_pin>")]
pub fn change(name: String, pin: i32, new_name: String, new_pin: i32) -> JsonValue {
    let mut users: Vec<User> = db_read();

    let hashed_pin_input = sha1::Sha1::from(&pin.to_string()).digest().to_string();

    // Loop over elements in vector
    for i in 0..users.len() {
        if users[i].name == name.to_lowercase() {
            // make sure name exists
            if hashed_pin_input == users[i].pin_hashed {
                // check if token is correct
                // Check wether to change name or name+pin
                if users[i].name == new_name.to_lowercase() {
                    // check if new name already exists
                    users[i].pin_hashed = sha1::Sha1::from(&new_pin.to_string()).digest().to_string();
                    /*
                    match write_json(&users) {
                        Err(why) => panic!("Cannot write to json! {}", why),
                        Ok(()) => info!("succesfully wrote to json file"),
                    }*/
                    db_write(&users);
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
                    /*
                    match write_json(&users) {
                        Err(why) => panic!("couldn't write to json file! {}", why),
                        Ok(()) => info!("succesfully wrote to json file"),
                    }*/
                    db_write(&users);
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
                warn!("Incorrect token for user {}!", name.to_string());
                return json!({
                    "status": "fail",
                    "reason": "incorrect token for user",
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
    let users: Vec<User> = db_read();
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
