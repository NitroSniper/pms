use std::{collections::HashMap, sync::Mutex};
use uuid::Uuid;

use lazy_static::lazy_static;

lazy_static! {
    static ref DATABASE: Mutex<Database> = Mutex::new(Database::new());
}

struct Database {
    users: HashMap<Uuid, User>,
    spaces: HashMap<Uuid, ParkingSpace>,
    messages: Vec<Message>,
    // add payment model
}

impl Database {
    fn new() -> Self {
        Database {
            users: HashMap::new(),
            spaces: HashMap::new(),
            messages: Vec::new(),
        }
    }
}

pub struct PublicBackend;

impl PublicBackend {
    pub fn login(username: String, password: String) -> Option<Uuid> {
        let db = DATABASE.lock().expect("Database to be free"); // [database access required]

        let (uuid, _) = db
            .users
            .clone()
            .into_iter()
            .find(|(_uuid, user)| user.username == username && user.password == password)?;
        Some(uuid)
    }
    pub fn signup(username: String, password: String) -> Option<Uuid> {
        // create a unique user id
        let user_id = Uuid::new_v4();

        let mut db = DATABASE.lock().expect("Database to be free"); // [database access required]        // check if username exist, will be easier cause database...
        if db
            .users
            .values()
            .find(|user| user.username == username)
            .is_some()
        {
            return None; // can't accept this transaction
        }
        db.users.insert(user_id, User { username, password });
        Some(user_id)
    }
    // can't do other functions cause wtf is status?
}

struct ElevatedBackend;

impl ElevatedBackend {
    fn calculate_parking_charge() -> u32 {
        // TODO! should be uint since we are doing a credit
        // system
        todo!()
    }
}

#[derive(Clone)]
struct Message {
    message: String,
    sender: String,
    recipient: String,
}

#[derive(Clone)]
struct User {
    // id: used in Hashmap Search
    username: String,
    password: String,
    // Other detail should be implemented here like contact info
}

// all methods defined for User
trait UserTrait {
    // Getters and Setters
    fn send_message(&self, message: String, recipient: String);
    fn receive_message(&self) -> Vec<Message>;
}

impl UserTrait for User {
    fn send_message(&self, message: String, recipient: String) { // username of recipient, could be
        let mut db = DATABASE.lock().expect("Database to be free"); // [database access required]
        db.messages.push(Message { message, sender: self.username.clone(), recipient })
    }
    fn receive_message(&self) -> Vec<Message> {
        let db = DATABASE.lock().expect("Database to be free"); // [database access required]
        db.messages.clone().into_iter().filter(|msg| msg.recipient == self.username).collect()
    }
}

struct ParkingSpace {
    // id: used in Hashmap Search
    status: bool,     // TODO! more context
    location: String, // TODO! isn't this float?
}




fn main() {
    // Check if Sign in works
    let n_username = "Nitro".to_owned();
    let n_password = "I am Dead".to_owned();
    let nitro_id =
        PublicBackend::signup(n_username.clone(), n_password.clone()).expect("Nitro to not exist");
    assert!(
        PublicBackend::signup(n_username.clone(), n_password.clone()).is_none(),
        "Nitro already exists: Same Password"
    );
    assert!(
        PublicBackend::signup(
            n_username.to_owned(),
            "I am a different password".to_owned()
        )
        .is_none(),
        "Nitro already exists: Different Password"
    );
    let sniper_id =
        PublicBackend::signup("Sniper".to_owned(), "I am a different password".to_owned())
            .expect("Sniper to not exist");
    assert_ne!(nitro_id, sniper_id, "Id Should be different");

    // Check if Log in works
    let other_id = PublicBackend::login(n_username.clone(), n_password.clone())
        .expect("Log in to be successful");
    assert_eq!(nitro_id, other_id, "Both uuid to be equal");
    assert!(PublicBackend::login(n_username, "wut".to_owned()).is_none());
}
