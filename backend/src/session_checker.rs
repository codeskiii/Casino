use rocket::serde::{json::Json, Deserialize, Serialize};
use rocket::{get, Request, State};
use std::collections::HashMap;
use std::sync::Mutex;

mod user_manager;

#[derive(Deserialize, Serialize)]
pub struct CheckResponse {
    pub test_passed: bool,
    pub user: Option<user_manager::User>,  // `Option` in case user is not found
}

#[derive(Clone)]
pub struct Session {
    pub user: user_manager::User,
}

pub struct SessionStorage {
    pub sessions: Mutex<HashMap<u64, Session>>,
}

impl SessionStorage {
    pub fn new() -> Self {
        SessionStorage {
            sessions: Mutex::new(HashMap::new()),
        }
    }
}

#[get("/check")]
pub fn check(request: &Request<'_>, session_storage: &State<SessionStorage>) -> Json<CheckResponse> {
    let session_id = match request.headers().get_one("Session") {
        Some(id_str) => match id_str.parse::<u64>() {
            Ok(id) => id,
            Err(_) => return Json(CheckResponse { test_passed: false, user: None }),
        },
        None => return Json(CheckResponse { test_passed: false, user: None }),
    };

    let sessions = session_storage.sessions.lock().unwrap();

    match sessions.get(&session_id) {
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
