extern crate log;
use crate::file_io::{append_json, read_json, write_json};
use crate::user::User;
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
#[post("/api/register/<name>/<pin>/<pronouns>")]
pub fn register_user(name: String, pin: i32, pronouns: String) -> String {
    let mut users: Vec<User> = read_json(); // Create an array of users out of parsed json
    for i in &users {
        // loop through elements of the vector
        if i.name == name.to_lowercase() {
            warn!("Cannot create user {}! User is already in system.", i.name);
            return "User already exists!".to_string();
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
    match append_json(&users) {
        Err(why) => panic!("couldn't append json: {}", why),
        Ok(()) => info!("Succesfully appended to json"),
    };

    info!(
        "succesfully created user {} with pin hash {}",
        users[users.len() - 1].name.to_string(),
        users[users.len() - 1].pin_hashed
    );
    return format!(
        "User {} registered with pin hash: {}",
        users[users.len() - 1].name.to_string().to_lowercase(),
        users[users.len() - 1].pin_hashed
    );
}

// Check if pin matches user
#[get("/api/users/<name>/<pin>")]
pub fn check_pin(name: String, pin: i32) -> String {
    let users: Vec<User> = read_json();
    let hashed_pin_input = sha1::Sha1::from(&pin.to_string()).digest().to_string();
    for i in &users {
        // loop through the vector
        if i.name == name.to_lowercase() {
            if i.pin_hashed == hashed_pin_input {
                info!("pin correct for user {}", i.name);
                return "pin matches".to_string();
            } else {
                warn!("pin incorrect for user {}", i.name);
                return "Incorrect pin".to_string();
            };
        };
    }
    warn!(
        "cannot check pin for user {} as they do not exist",
        name.to_string().to_lowercase()
    );
    return format!("User {} does not exist.", name.to_string().to_lowercase());
}

// Change a users pin/name
#[post("/api/users/change/<name>/<pin>/<new_name>/<new_pin>")]
pub fn change(name: String, pin: i32, new_name: String, new_pin: i32) -> String {
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
                    users[i].pin_hashed =
                        sha1::Sha1::from(&new_pin.to_string()).digest().to_string();
                    match write_json(&users) {
                        Err(why) => panic!("Cannot write to json! {}", why),
                        Ok(()) => info!("succesfully wrote to json file"),
                    }
                    info!("Changed pin of {}", name.to_string().to_lowercase());
                    return format!(
                        "User {}'s new pin hash is {}.",
                        name.to_string().to_lowercase(),
                        users[i].pin_hashed
                    );
                } else {
                    // check if new name already exists
                    for n in &users {
                        if n.name == new_name.to_lowercase() {
                            warn!(
                                "Could not change name of {} to {}, as new name is already taken.",
                                name.to_lowercase(),
                                new_name.to_lowercase()
                            );
                            return format!(
                                "New name {} is already taken!",
                                new_name.to_lowercase()
                            );
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
                    return format!(
                        "User previously known as {} is now {}. Pin hash, if different, is {}",
                        name.to_string(),
                        users[i].name.to_string(),
                        users[i].pin_hashed.to_string()
                    );
                }
            } else {
                warn!("Incorrect pin given for user {}!", name.to_string());
                return format!("Incorrect pin for user {}!", name.to_string());
            }
        }
    }
    warn!(
        "User {} not found, could not change pin and/or name.",
        name.to_string()
    );
    return format!("User {} not found.", name.to_string());
}

#[get("/api/users/<name>")]
pub fn get_user(name: String) -> String {
    let users: Vec<User> = read_json();
    let found_user = users
        .iter()
        .filter(|u| u.name == name.to_lowercase())
        .next();

    match found_user {
        Some(user) => format!("User {}", &user.name),
        None => "User does not exist".to_string(),
    }
}

/* Get data about a user */
#[get("/api/about/name/<name>")]
pub fn get_user_name(name: String) -> String {
    let users: Vec<User> = read_json();
    let found_user = users
        .iter()
        .filter(|u| u.name == name.to_lowercase())
        .next();

    match found_user {
        Some(user) => user.name.to_string(),
        None => "NULL".to_string(),
    }
}

#[get("/api/about/pronouns/<name>")]
pub fn get_user_pronouns(name: String) -> String {
    let users: Vec<User> = read_json();
    let found_user = users
        .iter()
        .filter(|u| u.name == name.to_lowercase())
        .next();

    match found_user {
        Some(user) => user.pronouns.to_string(),
        None => "NULL".to_string(),
    }
}
