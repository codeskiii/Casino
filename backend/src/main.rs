#[macro_use] extern crate rocket;

use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::Header;
use rocket::{Request, Response};

#[derive(rocket::serde::Serialize)]
struct GameSnippet {
    id: u16,
    name: String,
    description: String,
    cryptos: Vec<String>,
}

#[derive(rocket::serde::Serialize)]
struct LotterySnippet {
    id: u16,
    name: String,
    description: String,
    cost: u8,
    prize_multiplier: u8,
    cryptos: Vec<String>,
}

#[derive(rocket::serde::Serialize)]
struct IndexPage {
    games: Vec<GameSnippet>,
    lotteries: Vec<LotterySnippet>,
}

#[get("/")]
fn get_home_page() -> rocket::serde::json::Json<IndexPage> {
    rocket::serde::json::Json(IndexPage {
        games: vec![GameSnippet {
            id: 1,
            name: "Game One".to_string(),
            description: "Description of game one".to_string(),
            cryptos: vec!["Crypto1".to_string(), "Crypto2".to_string()],
        }],
        lotteries: vec![LotterySnippet {
            id: 1,
            name: "Lottery One".to_string(),
            description: "Description of lottery one".to_string(),
            cost: 10,
            prize_multiplier: 2,
            cryptos: vec!["Crypto1".to_string(), "Crypto2".to_string()],
        }],
    })
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
