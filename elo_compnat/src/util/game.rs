use pyo3::prelude::*;
use pyo3::types::IntoPyDict;
use serde::{Deserialize, Serialize};
use skillratings::Outcomes;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[pyclass]
pub enum GameResult {
    H,
    A,
    D,
}

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
    ) -> Game {
        Game {
            id,
            home,
            away,
            home_score,
            away_score,
            result,
            year,
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
