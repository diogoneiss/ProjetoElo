use crate::util::game::Game;
use std::error::Error;

pub fn load_csv(path: &str) -> Result<Vec<Game>, Box<dyn Error>> {
    let mut reader = csv::Reader::from_path(path)?;
    let result: Vec<Game> = reader
        .deserialize()
        .map(|r: Result<Game, csv::Error>| r.unwrap())
        .collect();
    Ok(result)
}

pub fn filter_by_year(games: &[Game], year: u16) -> Vec<Game> {
    games.iter().filter(|p| p.year == year).cloned().collect()
}
