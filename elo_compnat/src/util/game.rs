use serde::{Deserialize, Serialize};
use skillratings::Outcomes;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum GameResult {
    H,
    A,
    D,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Game {
    pub id: u64,
    #[serde(rename = "HomeTeam")]
    pub home: String,
    #[serde(rename = "AwayTeam")]
    pub away: String,
    #[serde(rename = "FTHG")]
    pub home_score: u16,
    #[serde(rename = "FTAG")]
    pub away_score: u16,
    #[serde(rename = "FTR")]
    pub result: GameResult,
    #[serde(rename = "Season")]
    pub year: u16,
}

impl Game {
    pub fn get_match_outcome(&self) -> (Outcomes, Outcomes) {
        match self.result {
            GameResult::H => (Outcomes::WIN, Outcomes::LOSS),
            GameResult::A => (Outcomes::LOSS, Outcomes::WIN),
            GameResult::D => (Outcomes::DRAW, Outcomes::DRAW),
        }
    }
}
