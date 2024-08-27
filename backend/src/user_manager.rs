use rocket::serde::{json::Json, Deserialize, Serialize};
use rocket::{get, State};
use std::collections::HashMap;
use std::sync::Mutex;

use std::io::{self, BufReader};
use rocket::request::{self, Request, FromRequest};
use std::fs::File;

use std::path::Path;

#[derive(Clone, Deserialize, Serialize)]
pub struct User {
    pub username: Option<String>,
    pub passwd_hash: Option<String>, // passwords are sha1 hashed
}

#[derive(Deserialize, Serialize)]
pub struct UserStorage {
    pub users: Mutex<HashMap<String, User>>,
}

impl UserStorage {
    pub fn new() -> Self {
        let user_box: HashMap<String, User> = HashMap::new();
        
        UserStorage {
            users: Mutex::new(user_box),
        }
    }
}

#[derive(Clone, Deserialize, Serialize)]
pub struct UserReturnable {
    pub username: String,
}

#[derive(Clone, Deserialize, Serialize)]
pub struct Response {
    pub user: Option<UserReturnable>,
    pub failed: bool,
}

// register page api idk iam not a real developer
#[get("/register")]
pub fn register_page(_state: &State<UserStorage>) -> Json<Response> {
    // Implement registration logic here
    Json(Response {
        user: None,
        failed: true, // Placeholder for actual implementation
    })
}

pub fn load_box() -> Result<UserStorage, io::Error>  {
    let path = Path::new("../datastuff/data/mainpage.json");

    if !path.exists() {
        eprintln!("Error: File {:?} does not exist", path);
        return Err(io::Error::new(io::ErrorKind::NotFound, "File not found"));
    }

    let file = File::open(&path)?;
    println!("File read successfully");

    let reader = BufReader::new(file);

    let boxed_users: UserStorage = match serde_json::from_reader(reader) {
        Ok(data) => data,
        Err(e) => {
            eprintln!("Failed to deserialize JSON: {}", e);
            return Err(io::Error::new(io::ErrorKind::InvalidData, "Failed to deserialize JSON"));
        }
    };

    Ok(boxed_users)
}

// smth i can call logging page
#[get("/logging/<username>/<passwd_hash>")]
pub fn login_page(username: String, passwd_hash: String, state: &State<UserStorage>) -> Json<Response> {
    let users = state.users.lock().unwrap();

    match users.get(&username) {
        Some(user) => {
            if user.passwd_hash.as_ref().map(|s| s == &passwd_hash).unwrap_or(false) {
                let user_return = UserReturnable {
                    username: user.username.clone().unwrap_or_default(),
                };
                Json(Response { user: Some(user_return), failed: false })
            } else {
                Json(Response { user: None, failed: true })
            }
        }
        None => Json(Response { user: None, failed: true }),
    }
}