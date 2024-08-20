#[macro_use] extern crate rocket;

use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::Header;
use rocket::{Request, Response};

use rocket::serde::{json::Json, Deserialize, Serialize};
use std::fs::File;
use std::io::BufReader;

#[derive(Debug, Deserialize, Serialize)]
struct Lottery {
    id: u16,
    name: String,
    description: String,
    prize: u64,
    prize_currency: String,
    can_enter: bool,
    last_draw: String,
    prize_withdraw_time: String,
    cryptos: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize)]
struct Game {
    id: u16,
    name: String,
    description: String,
    entry_cost: u8,
    cost_currency: u8,
    prize_multiplier: u8,
    cryptos: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize)]
struct MainPage {
    games: Vec<Game>,
    lotteries: Vec<Lottery>,
}

#[get("/")]
fn get_home_page() -> Json<MainPage> {
    let file = File::open("../datastuff/data/mainpage.json").expect("Failed to open file");
    println!("File read");
    let reader = BufReader::new(file);
    
    let main_page: MainPage = serde_json::from_reader(reader).expect("Failed to deserialize JSON");

    Json(main_page)
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
    rocket::build()
        .mount("/", routes![get_home_page])
        .attach(CORS) // Attach the CORS fairing
}
