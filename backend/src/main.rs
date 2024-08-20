#[macro_use] extern crate rocket;

use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::Header;
use rocket::{Request, Response};

mod main_page;

struct Session {
    id : u64,
    username: String,
    // TO DO: WHOLE USER
}

pub struct SessionStorage {
    sessions : Vec<Session>
}

pub struct CORS;

#[rocket::async_trait]
impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "Add CORS headers to responses",
            kind: Kind::Response,
        }
    }

    async fn on_response<'r>(&self, _req: &'r Request<'_>, res: &mut Response<'r>) {
        res.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        res.set_header(Header::new("Access-Control-Allow-Methods", "GET, POST, PUT, DELETE"));
        res.set_header(Header::new("Access-Control-Allow-Headers", "Content-Type, Authorization"));
    }
}

#[launch]
fn rocket() -> _ {
    let session_storage = SessionStorage::new();
    rocket::build()
        .mount("/", routes![main_page::get_home_page])
        .attach(CORS) // CORS :D
}
