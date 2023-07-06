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
    #[serde(rename = "Week")]
    pub week: f32,
    #[serde(rename = "Date")]
    pub date: String,
    #[serde(rename = "HomeTeam")]
    pub home: String,
    #[serde(rename = "AwayTeam")]
    pub away: String,
    #[serde(rename = "HomeGoals")]
    pub home_score: u16,
    #[serde(rename = "AwayGoals")]
    pub away_score: u16,
    #[serde(rename = "Result")]
    pub result: GameResult,
    #[serde(rename = "Season")]
    pub year: u16,
    #[serde(rename = "Divisao")]
    pub division: u8,
    #[serde(rename = "HomeTeam_value_norm2")]
    pub home_value: f64,
    #[serde(rename = "AwayTeam_value_norm2")]
    pub away_value: f64,
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
        week: f32,
        date: String,
        home: String,
        away: String,
        home_score: u16,
        away_score: u16,
        result: GameResult,
        year: u16,
        division: u8,
        home_value: f64,
        away_value: f64,
        home_elo: Option<f64>,
        away_elo: Option<f64>,
    ) -> Game {
        Game {
            week,
            date,
            home,
            away,
            home_score,
            away_score,
            result,
            year,
            division,
            home_value,
            away_value,
            home_elo,
            away_elo,
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
