extern crate log;
use crate::user::User;
use once_cell::sync::Lazy;
use std::sync::Mutex;

type MyErrorType = Box<dyn std::error::Error>;
static DB: Lazy<Mutex<sled::Db>> = Lazy::new(|| Mutex::new(sled::Db::open("users_db").unwrap()));

// add a user to the database
pub fn db_add(user: &User) {
    let db: sled::Db = sled::open("users_db").unwrap();
    let bytes = bincode::serialize(&user).unwrap();
    db.insert(&user.name, bytes).unwrap();
    info!("succesfully appended user {} to database", &user.name);
}

// write all changed users to database
pub fn db_write(users_list: &Vec<User>) {
    let db = DB.lock().unwrap();
    for i in users_list {
        let bytes = bincode::serialize(&i).unwrap();
        db.insert(&i.name, bytes).unwrap();
        info!("wrote user {} to db", &i.name);
    }
    info!("wrote all users to db");
}

// remove a user from the database
pub fn db_remove(user: &User) {
    let db = DB.lock().unwrap();
    db.remove(&user.name);
}

// read all users from the database
pub fn db_read() -> Vec<User> {
    let db = DB.lock().unwrap();
    let mut users: Vec<User> = Vec::new();
    for (_, bytes) in db.iter().filter_map(|r| r.ok()) {
        let read_user: User = bincode::deserialize(&bytes).unwrap();
        info!("read user {} from db", read_user.name);
        users.push(read_user);
    }
    return users;
}

// read one user from the database
pub fn db_read_user(user: &str) -> std::result::Result<Option<User>, MyErrorType> {
    let db = DB.lock().unwrap();
    let entry = db.get(user)?;
    if let Some(user_entry) = entry {
        let read_user: User = bincode::deserialize(&user_entry)?;
        info!("read user {} from db", read_user.name);
        Ok(Some(read_user))
    } else {
        warn!("user {} not found in db!", user);
        Ok(None)
    }
}
