use rocket::serde::{json::Json, Deserialize, Serialize};
use std::fs::File;
use std::io::BufReader;

#[derive(Deserialize)]
struct CheckResponse {
    test_passed: bool,
    username: Option<String>,
}


#[get("/check/")]
fn check(request: &Request<'_>) -> Json<CheckResponse> {
    let session_id = request.headers().get_one("Session");

    /*
    TU BY BYLA LOGIKA GDYBYM UMIAŁ SIĘ NIĄ POSLUGIWAC |
                                                      \/
    */

    // TO DO

    
    Json(CheckResponse {
        test_passed: false,
        username: None,
    })
}