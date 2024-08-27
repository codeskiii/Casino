use rocket::serde::{json::Json, Deserialize, Serialize};
use std::fs::File;
use std::io::{self, BufReader};
use rocket::request::{self, Request, FromRequest};
use std::path::Path;

#[derive(Debug, Deserialize, Serialize)]
pub struct Lottery {
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
pub struct Game {
    id: u16,
    name: String,
    description: String,
    entry_cost: u8,
    cost_currency: u8,
    prize_multiplier: u8,
    cryptos: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct MainPage {
    games: Vec<Game>,
    lotteries: Vec<Lottery>,
}

#[get("/")]
pub fn get_home_page() -> Result<Json<MainPage>, io::Error> {
    let path = Path::new("../datastuff/data/mainpage.json");

    if !path.exists() {
        eprintln!("Error: File {:?} does not exist", path);
        return Err(io::Error::new(io::ErrorKind::NotFound, "File not found"));
    }

    let file = File::open(&path)?;
    println!("File read successfully");

    let reader = BufReader::new(file);

    let main_page: MainPage = match serde_json::from_reader(reader) {
        Ok(data) => data,
        Err(e) => {
            eprintln!("Failed to deserialize JSON: {}", e);
            return Err(io::Error::new(io::ErrorKind::InvalidData, "Failed to deserialize JSON"));
        }
    };

    Ok(Json(main_page))
}
