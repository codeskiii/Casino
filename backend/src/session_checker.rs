use rocket::serde::{json::Json, Deserialize, Serialize};
use rocket::{get, Request, State};
use std::collections::HashMap;

mod user_manager;

#[derive(Deserialize, Serialize)]
struct CheckResponse {
    test_passed: bool,
    user: Option<user_manager::User>,  // `Option` in case user is not found
}

struct Session {
    user: user_manager::User,
}

struct SessionStorage {
    sessions: HashMap<u64, Session>,
}

#[get("/check/")]
pub fn check(request: &Request<'_>, session_storage: &State<SessionStorage>) -> Json<CheckResponse> {
    let session_id = match request.headers().get_one("Session") {
        Some(id_str) => match id_str.parse::<u64>() {
            Ok(id) => id,
            Err(_) => return Json(CheckResponse { test_passed: false, user: None }),
        },
        None => return Json(CheckResponse { test_passed: false, user: None }),
    };

    match session_storage.sessions.get(&session_id) {
        Some(session) => Json(CheckResponse { 
            test_passed: true, 
            user: Some(session.user.clone()), 
        }),
        None => Json(CheckResponse { 
            test_passed: false, 
            user: None,
        }),
    }
}