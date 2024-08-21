use rocket::serde::{json::Json, Deserialize, Serialize};
use rocket::{get, Request, State};
use std::collections::HashMap;
use std::sync::Mutex;

pub struct User {
    pub username: String,
    pub passwd_hash: [char; 40], // passwords are sha1 hashed
}

pub struct UserStorage {
    pub users: Mutex<HashMap<String, User>>,
}

#[derive(Serialize, Deserialize)]
pub struct UserReturnable {
    pub username: String,
}

#[derive(Serialize, Deserialize)]
pub struct Response {
    pub user: Option<UserReturnable>,
    pub failed: bool,
}

// register page api idk iam not a real developer
#[get("/register/")]
pub fn register_page(_request: &Request<'_>, _state: &State<UserStorage>) -> Json<Response> {
    // Implement registration logic here
    Json(Response {
        user: None,
        failed: true, // Placeholder for actual implementation
    })
}

// smth i can call logging page
#[get("/logging")]
pub fn login_page(request: &Request<'_>, state: &State<UserStorage>) -> Json<Response> {
    // Extract username from headers
    let username = match request.headers().get_one("username") {
        Some(username) => username,
        None => return Json(Response { user: None, failed: true }),
    };

    // Extract password hash from headers
    let passwd_hash = match request.headers().get_one("password_hash") {
        Some(passwd_hash) => passwd_hash,
        None => return Json(Response { user: None, failed: true }),
    };

    // Lock the users map for thread-safe access
    let users = state.users.lock().unwrap();

    // Look for the user in the storage
    match users.get(username) {
        Some(user) => {
            // Check if the provided password hash matches the stored one
            if &user.passwd_hash == &passwd_hash.chars().collect::<Vec<_>>()[..] {
                let user_return = UserReturnable {
                    username: user.username.clone(),
                };

                Json(Response { user: Some(user_return), failed: false })
            } else {
                Json(Response { user: None, failed: true })
            }
        }
        None => Json(Response { user: None, failed: true }),
    }
}