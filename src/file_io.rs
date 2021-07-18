use std::fs::{File, OpenOptions};
use std::io::prelude::*;
use std::io::{self, BufRead};
use std::path::Path;
extern crate log;
use crate::user::User;
use serde_json::Result;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

// Function to read json from file into the vector
pub fn read_json() -> Vec<User> {
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
pub fn append_json(user: &User) -> Result<()> {
    // Create a file to write to
    let path = Path::new("users.json");
    let display = path.display();

    let mut file = match OpenOptions::new()
        .write(true)
        .create(true)
        .append(true)
        .open(&path)
    {
        Err(why) => panic!("couldn't create {}: {}", display, why),
        Ok(file) => file,
    };

    // Serialize the last user value
    let user_json = serde_json::to_string(&user)?;

    // Write to the file
    match file.write_all(user_json.as_bytes()) {
        Err(why) => panic!("couldn't write to {}: {}", display, why),
        Ok(_) => info!("succesfully wrote to {}", display),
    };
    // Add newline
    match file.write_all("\n".as_bytes()) {
        Err(why) => panic!("couldn't write to {}: {}", display, why),
        Ok(_) => info!("succesfully wrote newline to {}", display),
    };
    Ok(())
}

// Function to write whole vector of users to file
pub fn write_json(users_list: &Vec<User>) -> Result<()> {
    // Create a file to write to
    let path = Path::new("users.json");
    let display = path.display();

    let mut file = match OpenOptions::new().write(true).create(true).open(&path) {
        Err(why) => panic!("couldn't create {}: {}", display, why),
        Ok(file) => file,
    };

    let mut users_json = String::new();
    for i in 0..users_list.len() {
        // Serialize the users
        users_json += &serde_json::to_string(&users_list[i])?;
        //if i != users_list.len() - 1 {
            // don't append newline if it's the last element
            users_json += "\n";
        //}
    }

    // Write to the file
    match file.write_all(users_json.as_bytes()) {
        Err(why) => panic!("couldn't write to {}: {}", display, why),
        Ok(_) => info!("succesfully wrote to {}", display),
    };
    Ok(())
}
