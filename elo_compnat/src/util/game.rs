use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
use skillratings::Outcomes;

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
#[pyclass]
pub enum GameResult {
    H,
    A,
    D,
}

// TODO: adicionar as colunas de home_elo e away_elo no csv, com valor 0
#[derive(Serialize, Deserialize, Debug, Clone)]
#[pyclass]
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
    pub home_elo: Option<f64>,
    pub away_elo: Option<f64>,
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

#[pymethods]
impl Game {
    #[new]
    fn new(
        id: u64,
        home: String,
        away: String,
        home_score: u16,
        away_score: u16,
        result: GameResult,
        year: u16,
        home_elo: Option<f64>,
        away_elo: Option<f64>
    ) -> Game {
        Game {
            id,
            home,
            away,
            home_score,
            away_score,
            result,
            year,
            home_elo,
            away_elo
        }
    }
}

#[pymethods]
impl GameResult {
    // add constructors for each variant
    #[staticmethod]
    fn h() -> GameResult {
        GameResult::H
    }

    #[staticmethod]
    fn a() -> GameResult {
        GameResult::A
    }

    #[staticmethod]
    fn d() -> GameResult {
        GameResult::D
    }
}
