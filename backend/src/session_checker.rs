use rocket::serde::{json::Json, Deserialize, Serialize};
use rocket::{get, State};
use std::collections::HashMap;
use std::sync::Mutex;

use rocket::request::{self, Request, FromRequest};

use crate::user_manager;

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
        let session_box: HashMap<u64, Session> = HashMap::new();
        
        SessionStorage {
            sessions: Mutex::new(session_box),
        }
    }
}

#[get("/check/<session_id>")]
pub fn check(session_id: u64, session_storage: &State<SessionStorage>) -> Json<CheckResponse> {
    // Lock the session storage to safely access the sessions map
    let sessions = session_storage.sessions.lock().unwrap();

    // Check if a session with the given ID exists
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