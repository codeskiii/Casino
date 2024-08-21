use rocket::serde::{json::Json, Deserialize, Serialize};
use rocket::{get, Request, State};

pub struct UserStorage {
    users: Vec<User>
}

pub struct User {
    username: String,
    passwd_hash: [char; 40] // passwords are sha1 hashed
} 

struct response {

}

// register page api idk iam not a real developer
#[get("/register/")]
pub fn (request: &Request<'_>) -> Json<> {

}

// smth i can call logging page
#[get("/log/")]
pub fn (request: &Request<'_>) -> Json<> {

}