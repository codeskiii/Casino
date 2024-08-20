
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
pub fn get_home_page() -> Json<MainPage> {
    let file = File::open("../datastuff/data/mainpage.json").expect("Failed to open file");
    println!("File read");
    let reader = BufReader::new(file);
    
    let main_page: MainPage = serde_json::from_reader(reader).expect("Failed to deserialize JSON");

    Json(main_page)
}