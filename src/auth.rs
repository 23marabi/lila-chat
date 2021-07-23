extern crate log;
use crate::file_io::*;
use rocket::http::{Cookie, Cookies};
use crate::user::*;
use rocket_contrib::json::{Json, JsonValue};
use random_string::generate;
extern crate sha1;
use serde::Deserialize;

// Post request to register a user and pin
#[post("/register", format = "json", data = "<data>")]
pub fn register(data: Json<RegisterEvent>) -> JsonValue {
    // check if the user exists
    if let Some(user) = db_read_user(&data.name).ok().flatten() {
        warn!("Cannot create user {}! User is already in system.", data.name);
        return json!({
            "status": "fail",
            "reason": "user already exists",
        });
    } else {
        let pin_hashed = sha1::Sha1::from(&data.pin).digest().to_string(); // hash the pin
    
        let new_user: User = User {
            name: data.name.to_string().to_lowercase(),
            pin_hashed,
            pronouns: data.pronouns.to_string().to_lowercase(),
            session_token: "NULL".to_string(),
            role: UserType::Normal,

        };
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
}

fn create_token(name: String, mut user: User) -> String {
    let charset = "1234567890abcdefghijklmnopqrstuvwxyz";

    if user.name == name {
        user.session_token = generate(12, charset);
        db_add(&user);
        info!("succesfully created token for user {}", name);
        let token = user.session_token.clone();
        return token;
    };
    
    warn!("something bad happened while creating a token and idk what");
    return "NULL".to_string();
}

// Check if user is properly logged in
#[get("/token/<name>")]
pub fn check_token(name: String, mut cookies: Cookies) -> JsonValue {
    // check if the user is in the system
    if let Some(user) = db_read_user(&name).ok().flatten() {
        // get the token from the cookie
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

        // check the token value
        if token.value() == "NULL" {
            warn!("NULL token!");
            return json!({
                "status": "fail",
                "reason": "NULL token",
            });
        } else if token.value() == user.session_token {
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
    } else {
    warn!("user {} not found", name);
    return json!({
        "status": "fail",
        "reason": "user not found",
    });
    }
}

// Logout API
#[post("/logout", format = "json", data = "<info>")]
pub fn logout(info: Json<LogoutEvent>, mut cookies: Cookies) -> JsonValue {
    if let Some(mut user) = db_read_user(&info.name.to_lowercase()).ok().flatten() {
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
        } else if token.value() == user.session_token {
            cookies.remove_private(Cookie::named("token"));
            user.session_token = "NULL".to_string();
            db_add(&user);
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
    } else {
        warn!("failed to log out user {}, user not found", info.name);
        return json!({
            "status": "fail",
            "reason": "user not found",
        });
    }
}

// Check if pin matches user
#[post("/login", format = "json", data = "<data>")]
pub fn login(data: Json<LoginEvent>, mut cookies: Cookies) -> JsonValue {
    if let Some(user) = db_read_user(&data.name.to_lowercase()).ok().flatten() {
        let hashed_pin_input = sha1::Sha1::from(&data.pin.to_string()).digest().to_string();

        if user.pin_hashed == hashed_pin_input { // check if pin hash matches
            info!("pin correct for user {}", &user.name);

            // Create token for user & set a cookie
            let token = create_token(user.name.clone(), user);
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
            warn!("pin incorrect for user {}", user.name);
            return json!({
                "status": "fail",
                "reason": "incorrect pin",
            });
        };
    } else {
        cookies.remove_private(Cookie::named("token"));
        info!("removed private cookie");
        warn!(
            "cannot check pin for user {} as they do not exist",
            data.name.to_string().to_lowercase()
        );
        return json!({
            "status": "fail",
            "reason": format!("user {} doesn't exist", data.name.to_string().to_lowercase()),
        });
    }
}

// Change info about a user
#[post("/change", format = "json", data = "<input>")]
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

    // find the user
    if let Some(mut user) = db_read_user(&input.name).ok().flatten() {
        if token.value() == user.session_token { // & if token matches:
            match input.changed_event {
                ChangeEventType::Name => {
                    // remove the user first
                    db_remove(&user);
                    // change the name
                    user.name = input.new_event.clone();
                    info!("changed name of {} to {}", input.name, input.new_event);
                    db_add(&user);
                    return json!({
                        "status": "ok",
                        "reason": format!("changed name of {} to {}", input.name, input.new_event),
                    });
                },
                ChangeEventType::Pin => {
                    // change the pin
                    let new_hashed_pin = sha1::Sha1::from(&input.new_event).digest().to_string();
                    user.pin_hashed = new_hashed_pin.clone();
                    db_add(&user);
                    info!("changed pin of {}", input.name);
                    return json!({
                        "status": "ok",
                        "reason": "changed pin",
                    });
                },
                ChangeEventType::Pronouns => {
                    // change the pronouns
                    user.pronouns = input.new_event.clone();
                    info!("changed pronouns of {} to {}", input.name, input.new_event);
                    db_add(&user);
                    return json!({
                        "status": "ok",
                        "reason": "successfully changed pronouns",
                    });
                },
            };
        } else {
            warn!("incorrect pin for user {}", input.name);
            return json!({
               "status": "fail",
                "reason": "incorrect pin",
            });
       };
    } else {
        warn!("couldn't change users info, user does not exist");
        return json!({
            "status": "fail",
            "reason": "user doesn't exist",
        });
    }
    return json!({
        "status": "fail",
        "reason": "idk",
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
                "role": user.role,
            },
        }),
        None => json!({
            "status": "fail",
            "reason": format!("user {} not found", name),
        }),
    }
}

/* User Management */
#[post("/mod", format = "json", data = "<data>")]
pub fn moderation_actions(data: Json<ModerationAction>, mut cookies: Cookies) -> JsonValue {
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
    if let Some(user) = db_read_user(&data.name.to_lowercase()).ok().flatten() {
        if token.value() == "NULL" { // fail if token is NULL
            warn!("NULL token!");
            return json!({
                "status": "fail",
                "reason": "NULL token",
            });
        } else if user.session_token == token.value() { // if token matches
            if user.role == UserType::Normal {
            match data.action {
                    ModActions::Kick => {
                        info!("kicked user {}", data.target)
                    },
                    ModActions::Ban => info!("banned user {}", data.target),
                    _ => info!("F"),
                };
                return json!({
                    "status": "ok",
                    "reason": "completed action",
                });
            } else {
                warn!("user does not have sufficient permissions to perform that action!");
                return json!({
                    "status": "fail",
                    "reason": "insufficient permissions",
                });
            };
        } else {
            warn!("token does not match!");
            return json!({
                "status": "fail",
                "reason": "token does not match",
            })
        };
    } else {
        warn!("user not found");
        return json!({
            "status": "fail",
            "reason": "user not found"
        });
    }
}
