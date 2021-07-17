extern crate log;
use crate::user::User;
use crate::file_io::{read_json, append_json, write_json};
extern crate sha1;
use serde_json::Result;

#[get("/")]
pub fn index() -> &'static str {
    "API Info:

    `POST /api/register/<name>/<pin>` Register the username with the pin provided if it doesn't already exist

    `GET /api/users/<name>` Check if the user exists

    `GET /api/users/<name>/<pin>` Check if the user exists, and if the pin provided matches

    `POST /api/users/change/<name>/<pin>/<new-name>/<new-pin>` Change a users name and/or pin"
}
/*
// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

// Function to read json from file into the vector
fn read_json() -> Vec<User> {
    // Create path to file
    let path = Path::new("users.json");
    let display = path.display();

    let mut users: Vec<User> = Vec::new(); // Create an empty vector of users

    // Read through the lines and append them to the array
    if let Ok(lines) = read_lines(&path) {
        for line in lines {
            if let Ok(user) = line {
                info!("read {} from json file {}", display, &user);
                // Parse line from file into a data structure
                let user: User = serde_json::from_str(&user).unwrap();
                users.push(user);
            }
        }
    }
    return users;
}

// Function to append the last value of the users vector to the file
fn append_json(users_list: &Vec<User>)  -> Result<()> {
    // Create a file to write to
    let path = Path::new("users.json");
    let display = path.display();

    let mut file = match OpenOptions::new()
    .write(true)
    .create(true)
    .append(true)
    .open(&path){
        Err(why) => panic!("couldn't create {}: {}", display, why),
        Ok(file) => file,
    };

    // Serialize the last user value
    let users_json = serde_json::to_string(&users_list[users_list.len()-1])?;

    // Write to the file
    match file.write_all(users_json.as_bytes()) {
        Err(why) => panic!("couldn't write to {}: {}", display, why),
        Ok(_) => info!("succesfully wrote to {}", display),
    };
    // Add newline
    match file.write_all("\n".as_bytes()) {
        Err(why) => panic!("couldn't write to {}: {}", display, why),
        Ok(_) => info!("succesfully wrote to {}", display),
    };
    Ok(())
}

// Function to write whole vector of users to file
fn write_json(users_list: &Vec<User>) -> Result<()> {
    // Create a file to write to
    let path = Path::new("users.json");
    let display = path.display();

    let mut file = match OpenOptions::new()
    .write(true)
    .create(true)
    .open(&path){
        Err(why) => panic!("couldn't create {}: {}", display, why),
        Ok(file) => file,
    };
    
    let mut users_json = String::new();
    for i in 0..users_list.len() {
        // Serialize the users
        users_json += &serde_json::to_string(&users_list[i])?;
        if i <= users_list.len()-2 { // don't append newline if it's the last element
            users_json += "\n";
        }
    }

    // Write to the file
    match file.write_all(users_json.as_bytes()) {
        Err(why) => panic!("couldn't write to {}: {}", display, why),
        Ok(_) => info!("succesfully wrote to {}", display),
    };
    Ok(())
}
*/
// Post request to register a user and pin
#[post("/api/register/<name>/<pin>/<pronouns>")]
pub fn register_user(name: &str, pin: i32, pronouns: &str) -> String {
    let mut users: Vec<User> = read_json(); // Create an array of users out of parsed json
    for i in &users { // loop through elements of the vector
        if i.name == name.to_lowercase() {
            warn!("Cannot create user {}! User is already in system.", i.name);
            return "User already exists!".to_string();
        };
    };

    let pin_hashed = sha1::Sha1::from(&pin.to_string()).digest().to_string(); // hash the pin

    users.push( User {
        name: name.to_string().to_lowercase(),
        pin_hashed: pin_hashed,
        pronouns: pronouns.to_string().to_lowercase(),
        sessionToken: "NULL".to_string()
    }); // append the user to the vec

    // append to the json file
    match append_json(&users) {
        Err(why) => panic!("couldn't append json: {}", why),
        Ok(()) => info!("Succesfully appended to json"),
    };

    info!("succesfully created user {} with pin hash {}", users[users.len()-1].name.to_string(), users[users.len()-1].pin_hashed);
    return format!("User {} registered with pin hash: {}", users[users.len()-1].name.to_string().to_lowercase(), users[users.len()-1].pin_hashed);
}

// Check if pin matches user
#[get("/api/users/<name>/<pin>")]
pub fn check_pin(name: &str, pin: i32) -> String {
    let users: Vec<User> = read_json();
    let hashed_pin_input = sha1::Sha1::from(&pin.to_string()).digest().to_string();
    for i in &users { // loop through the vector
        if i.name == name.to_lowercase() {
            if i.pin_hashed == hashed_pin_input {
                info!("pin correct for user {}", i.name);
                return "pin matches".to_string();
            } else {
                warn!("pin incorrect for user {}", i.name);
                return "Incorrect pin".to_string();
            };
        };
    };
    warn!("cannot check pin for user {} as they do not exist", name.to_string().to_lowercase());
    return format!("User {} does not exist.", name.to_string().to_lowercase());
}

// Change a users pin/name
#[post("/api/users/change/<name>/<pin>/<new_name>/<new_pin>")]
pub fn change(name: &str, pin: i32, new_name: &str, new_pin: i32) -> String {
    let mut users: Vec<User> = read_json();

    let hashed_pin_input = sha1::Sha1::from(&pin.to_string()).digest().to_string();

    // Loop over elements in vector
    for i in 0..users.len() {
        if users[i].name == name.to_lowercase() { // make sure name exists
            if users[i].pin_hashed == hashed_pin_input { // check if pin is correct
                // Check wether to change name or name+pin
                if users[i].name == new_name.to_lowercase() {
                    // check if new name already exists
                        users[i].pin_hashed = sha1::Sha1::from(&new_pin.to_string()).digest().to_string();
                        match write_json(&users) {
                            Err(why) => panic!("Cannot write to json! {}", why),
                            Ok(()) => info!("succesfully wrote to json file"),
                        }
                        info!("Changed pin of {}", name.to_string().to_lowercase());
                        return format!("User {}'s new pin hash is {}.", name.to_string().to_lowercase(), users[i].pin_hashed);
                } else {
                    // check if new name already exists
                    for n in &users {
                        if n.name == new_name.to_lowercase() {
                            warn!("Could not change name of {} to {}, as new name is already taken.", name.to_lowercase(), new_name.to_lowercase());
                            return format!("New name {} is already taken!", new_name.to_lowercase());
                        }
                    }
                    users[i].name = new_name.to_string().to_lowercase();
                    users[i].pin_hashed = sha1::Sha1::from(&new_pin.to_string()).digest().to_string();
                            
                    match write_json(&users) {
                        Err(why) => panic!("couldn't write to json file! {}", why),
                        Ok(()) => info!("succesfully wrote to json file"),
                    }
                    info!("Changed name of {} to {}. New pin hash is {}", name.to_string(), users[i].name.to_string(), users[i].pin_hashed.to_string());
                    return format!("User previously known as {} is now {}. Pin hash, if different, is {}", name.to_string(), users[i].name.to_string(), users[i].pin_hashed.to_string());
                }
            } else {
                warn!("Incorrect pin given for user {}!", name.to_string());
                return format!("Incorrect pin for user {}!", name.to_string());
            }
        }
    }
    warn!("User {} not found, could not change pin and/or name.", name.to_string());
    return format!("User {} not found.", name.to_string());
}

#[get("/api/users/<name>")]
pub fn get_user(name: &str) -> String {
    let users: Vec<User> = read_json();
    let found_user = users.iter().filter(|u| u.name == name.to_lowercase()).next();

    match found_user {
        Some(user) => format!("User {}", &user.name),
        None => "User does not exist".to_string(),
    }
}

/* Get data about a user */
#[get("/api/about/name/<name>")]
pub fn get_user_name(name: &str) -> String {
    let users: Vec<User> = read_json();
    let found_user = users.iter().filter(|u| u.name == name.to_lowercase()).next();

    match found_user {
        Some(user) => user.name.to_string(),
        None => "NULL".to_string(),
    }
}

#[get("/api/about/pronouns/<name>")]
pub fn get_user_pronouns(name: &str) -> String {
    let users: Vec<User> = read_json();
    let found_user = users.iter().filter(|u| u.name == name.to_lowercase()).next();

    match found_user {
        Some(user) => user.pronouns.to_string(),
        None => "NULL".to_string(),
    }
}
