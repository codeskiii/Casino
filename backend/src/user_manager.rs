use rocket::serde::{json::Json, Deserialize, Serialize};
use rocket::{get, Request, State};
use std::collections::HashMap;

pub struct User {
    username: String,
    passwd_hash: [char; 40] // passwords are sha1 hashed
} 

pub struct UserStorage {
    users: HashMap<String, User>,
}

user UserReturnable {
    username: String,
}

struct response {
    user: Option<User>,
    failed: bool,
}

// register page api idk iam not a real developer
#[get("/register/")]
pub fn register_page(request: &Request<'_>, &State<UserStorage>) -> Json<response> {

}

// smth i can call logging page
#[get("/logging/")]
pub fn login_page(request: &Request<'_>, &State<UserStorage>) -> Json<response> {
    let username = match request.headers().get_one("username") {
        Ok(username) => username,
        _ => return Json(response { user: None, 
                                    failed: true }),
    };

    let passwd_hash = match request.headers().get_one("password_hash") {
        Ok(passwd_hash) => passwd_hash,
        _ => return Json(response { user: None, 
                                    failed: true }),
    };

    match UserStorage.users.get(&username) {

    }
    
}