use std::borrow::BorrowMut;
use chrono::{DateTime, Duration, Local, NaiveDateTime, Utc};
use serde::{Deserialize, Deserializer, Serialize};
use serde_json::{Map, Value, Result, from_value};

#[derive(PartialEq)]
enum Side {
    Radiant,
    Dire
}

#[derive(Debug, Serialize, Deserialize)]
struct MatchResponse {
    match_id: u64,
    start_time: u64,
    radiant_win: bool,
    duration: u32,
    hero_id: u32,
    kills: u32,
    deaths: u32,
    assists: u32,
    gold_per_min: u32,
    hero_damage: u32,
    tower_damage: u32,
    last_hits: u32,
    player_slot: u8
}

struct Match {
    match_id: u64,
    start_time: DateTime<Utc>,
    side: Side,
    won: bool,
    duration: Duration,
    hero: String,
    kills: u32,
    deaths: u32,
    assists: u32,
    gold_per_min: u32,
    hero_damage: u32,
    tower_damage: u32,
    last_hits: u32,
}

impl Match {
    fn from_match_response(response: &MatchResponse) -> Match {
        let naive_start_time_date_time = NaiveDateTime::from_timestamp(response.start_time as i64, 0);
        let start_time = DateTime::from_utc(naive_start_time_date_time, Utc);
        let side = if response.player_slot <= 127 { Side::Radiant } else { Side::Dire };
        
        Match {
            match_id: response.match_id,
            start_time,
            won: (side == Side::Radiant && response.radiant_win) || (side == Side::Dire && !response.radiant_win),
            side,
            duration: Duration::seconds(response.duration as i64),
            hero: "".to_string(),
            kills: 0,
            deaths: 0,
            assists: 0,
            gold_per_min: 0,
            hero_damage: 0,
            tower_damage: 0,
            last_hits: 0
        }
    }
}

fn main() {
    println!("Hello, world!");

    let matches: Vec<MatchResponse> = reqwest::blocking::get(" https://api.opendota.com/api/players/371012563/recentMatches")
        .unwrap()
        .json()
        .unwrap();

    for single_match in matches.iter().take(10) {
        println!("Found match: {:?}", single_match);
    }
}
