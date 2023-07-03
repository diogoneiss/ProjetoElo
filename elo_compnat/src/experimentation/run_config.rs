use pyo3::exceptions::PyValueError;
use pyo3::{prelude::*, types::PyDict};
use serde::{Deserialize, Serialize};
use std::f64::EPSILON;
use crate::util::game::{Game, GameResult};

/// this struct holds the necessary parameters for configuring the runtime of our experiments
/// It is also used as the genotype, as it holds all the experimentation parameters
#[derive(Debug, Clone, Deserialize, Serialize)]
#[pyclass(dict, frozen)]
pub struct RunConfig {
    pub k_factor: f64,
    pub gamma: f64,
    pub home_advantage: f64,
    pub home_field_advantage_weight: f64,
    pub market_value_weight: f64,
    pub tie_frequency: f64,
    pub w_division: Vec<f64>,
}

impl PartialEq for RunConfig {
    fn eq(&self, other: &Self) -> bool {
        (self.k_factor - other.k_factor).abs() < EPSILON
    }
}

impl std::hash::Hash for RunConfig {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        let rounded = (self.k_factor * 1000.0).round() / 1000.0;
        let s = format!("{:.3}", rounded);
        s.hash(state);
    }
}

impl Eq for RunConfig {}

impl Default for RunConfig {
    fn default() -> Self {
        RunConfig {
            k_factor: 20.0,
            gamma: 1.0,
            home_advantage: 0.075,
            home_field_advantage_weight: 0.075,
            market_value_weight: 1.0,
            tie_frequency: 0.5,
            w_division: vec![20.0],
        }
    }
}

#[pymethods]
impl RunConfig {
    #[new]
    fn new(
        k_factor: f64,
        gamma: f64,
        home_advantage: f64,
        home_field_advantage_weight: f64,
        market_value_weight: f64,
        tie_frequency: f64,
        w_division: Vec<f64>,
    ) -> RunConfig {
        RunConfig {
            k_factor,
            gamma,
            home_advantage,
            home_field_advantage_weight,
            market_value_weight,
            tie_frequency,
            w_division,
        }
    }

    #[getter]
    fn __dict__(&self) -> PyResult<PyObject> {
        Python::with_gil(|py| {
            let dict = PyDict::new(py);
            dict.set_item("k_factor", self.k_factor)?;
            dict.set_item("gamma", self.gamma)?;
            dict.set_item("home_advantage", self.home_advantage)?;
            dict.set_item("home_field_advantage_weight", self.home_field_advantage_weight)?;
            dict.set_item("market_value_weight", self.market_value_weight)?;
            dict.set_item("tie_frequency", self.tie_frequency)?;
            dict.set_item("w_division", self.w_division.clone())?;

            Ok(dict.to_object(py))
        })
    }

    #[staticmethod]
    fn from_dict(dict: &PyDict) -> PyResult<Self> {
        let k_factor = dict.get_item("k_factor").unwrap().extract()?;
        let gamma = dict.get_item("gamma").unwrap().extract()?;
        let home_advantage = dict.get_item("home_advantage").unwrap().extract()?;
        let home_field_advantage_weight = dict.get_item("home_field_advantage_weight").unwrap().extract()?;
        let market_value_weight = dict.get_item("market_value_weight").unwrap().extract()?;
        let tie_frequency = dict.get_item("tie_frequency").unwrap().extract()?;
        let w_division = dict.get_item("w_division").unwrap().extract()?;

        Ok(RunConfig::new(
            k_factor,
            gamma,
            home_advantage,
            home_field_advantage_weight,
            market_value_weight,
            tie_frequency,
            w_division,
        ))
    }

    fn to_dict(&self, py: Python) -> PyResult<Py<PyDict>> {
        let dict = PyDict::new(py);
        dict.set_item("k_factor", self.k_factor)?;
        dict.set_item("gamma", self.gamma)?;
        dict.set_item("home_advantage", self.home_advantage)?;
        dict.set_item("home_field_advantage_weight", self.home_field_advantage_weight)?;
        dict.set_item("market_value_weight", self.market_value_weight)?;
        dict.set_item("tie_frequency", self.tie_frequency)?;
        dict.set_item("w_division", self.w_division.clone())?;

        Ok(dict.into())
    }
    #[staticmethod]
    fn from_list(params: Vec<f64>) -> PyResult<Self> {
        if params.len() < 7 {
            return Err(PyValueError::new_err(
                "The input list should have at least 6 elements.",
            ));
        }

        let k_factor = params[0];
        let gamma = params[1];
        let home_advantage = params[2];
        let home_field_advantage_weight = params[3];
        let market_value_weight = params[4];
        let tie_frequency = params[5];
        let w_division: Vec<f64> = params[6..].to_vec();

        Ok(RunConfig::new(
            k_factor,
            gamma,
            home_advantage,
            home_field_advantage_weight,
            market_value_weight,
            tie_frequency,
            w_division,
        ))
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Hash)]
#[pyclass(dict, frozen)]
pub struct RunHyperparameters {
    pub starting_elo: u16,
    pub starting_year: u16,
    pub backtest_years: u16,
    pub random_variations: u16,
    pub use_goals_diff: bool,
    pub use_home_advantage: bool,
    pub use_market_values: bool,
    pub leagues_to_use: u16,
}

impl std::fmt::Display for RunHyperparameters {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} {} {}",
            self.starting_elo, self.starting_year, self.backtest_years
        )
    }
}

impl Default for RunHyperparameters {
    fn default() -> Self {
        RunHyperparameters {
            starting_elo: 1000,
            starting_year: 2003,
            backtest_years: 8,
            use_goals_diff: false,
            use_home_advantage: false,
            use_market_values: false,
            leagues_to_use: 1,
            random_variations: 20,
        }
    }
}

impl RunHyperparameters {
    pub fn print_errors_by_year(&self, errors: &[f64]) {
        let base_year = self.backtest_years + self.starting_year + 1;

        let horizontal_line = format!("{:-<1$}", "", 19);
        println!("{}", &horizontal_line);
        println!("|Errors by year:  |");

        for (i, error) in errors.iter().enumerate() {
            println!("|{}  :  {:.2}   |", base_year + i as u16, error);
        }
        println!("{}", &horizontal_line);
    }
}
#[pymethods]
impl RunHyperparameters {
    #[new]
    fn new(
        starting_elo: u16,
        starting_year: u16,
        backtest_years: u16,
        random_variations: u16,
        use_goals_diff: bool,
        use_home_advantage: bool,
        use_market_values: bool,
        leagues_to_use: u16,
    ) -> RunHyperparameters {
        RunHyperparameters {
            starting_elo,
            starting_year,
            backtest_years,
            random_variations,
            use_goals_diff,
            use_home_advantage,
            use_market_values,
            leagues_to_use,
        }
    }
    #[getter]
    fn __dict__(&self) -> PyResult<PyObject> {
        Python::with_gil(|py| {
            let dict = PyDict::new(py);
            dict.set_item("starting_elo", self.starting_elo)?;
            dict.set_item("starting_year", self.starting_year)?;
            dict.set_item("backtest_years", self.backtest_years)?;
            dict.set_item("random_variations", self.random_variations)?;
            dict.set_item("use_goals_diff", self.use_goals_diff)?;
            dict.set_item("use_home_advantage", self.use_home_advantage)?;
            dict.set_item("use_market_values", self.use_market_values)?;
            dict.set_item("leagues_to_use", self.leagues_to_use)?;
            Ok(dict.to_object(py))
        })
    }
    #[staticmethod]
    fn from_list(params: Vec<u16>) -> PyResult<Self> {
        if params.len() != 8 {
            return Err(PyValueError::new_err(
                "The input list should have exactly 8 elements.",
            ));
        }

        Ok(RunHyperparameters::new(
            params[0],
            params[1],
            params[2],
            params[3],
            params[4] != 0,
            params[5] != 0,
            params[6] != 0,
            params[7],
        ))
    }

    #[staticmethod]
    fn from_dict(dict: &PyDict) -> PyResult<Self> {
        let starting_elo = dict.get_item("starting_elo").unwrap().extract()?;
        let starting_year = dict.get_item("starting_year").unwrap().extract()?;
        let backtest_years = dict.get_item("backtest_years").unwrap().extract()?;
        let random_variations = dict.get_item("random_variations").unwrap().extract()?;
        let use_goals_diff = dict.get_item("use_goals_diff").unwrap().extract()?;
        let use_home_advantage = dict.get_item("use_home_advantage").unwrap().extract()?;
        let use_market_values = dict.get_item("use_market_values").unwrap().extract()?;
        let leagues_to_use = dict.get_item("leagues_to_use").unwrap().extract()?;

        Ok(RunHyperparameters::new(
            starting_elo,
            starting_year,
            backtest_years,
            random_variations,
            use_goals_diff,
            use_home_advantage,
            use_market_values,
            leagues_to_use,
        ))
    }
    fn to_dict(&self, py: Python) -> PyResult<Py<PyDict>> {
        let dict = PyDict::new(py);
        dict.set_item("starting_elo", self.starting_elo)?;
        dict.set_item("starting_year", self.starting_year)?;
        dict.set_item("backtest_years", self.backtest_years)?;
        dict.set_item("random_variations", self.random_variations)?;
        dict.set_item("use_goals_diff", self.use_goals_diff)?;
        dict.set_item("use_home_advantage", self.use_home_advantage)?;
        dict.set_item("use_market_values", self.use_market_values)?;
        dict.set_item("leagues_to_use", self.leagues_to_use)?;
        Ok(dict.into())
    }
}

#[derive(Debug, Clone)]
#[pyclass]
pub struct CustomRating {
    pub rating: f64
}

#[pymethods]
impl CustomRating {
    #[staticmethod]
    pub const fn new() -> Self {
        Self { rating: 1000.0 }
    }
}

impl Default for CustomRating {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Hash)]
#[pyclass]
pub struct CustomElo {
    pub config: RunConfig
}

impl CustomElo {
    
    pub fn rate(
        &self,
        player_one: &CustomRating,
        player_two: &CustomRating,
        outcome: GameResult,
        absolute_goal_diff: f64,
        absolute_market_value_diff: f64
    ) -> (CustomRating, CustomRating) {
        let RunConfig {k_factor, gamma, home_advantage, home_field_advantage_weight, market_value_weight, tie_frequency, w_division} = self.config.clone();
        let (tie_expected, one_expected, two_expected) = expected_score(player_one, player_two, &self.config);
        let real_player_one_score: f64 = match outcome {
            GameResult::H => 1.0,
            GameResult::D => 0.5,
            GameResult::A => 0.0
        };
        let real_player_two_score: f64 = 1.0 - real_player_one_score;
        let player_one_new_rate: f64 = player_one.rating + k_factor * w_division[0] * ((1.0 + absolute_market_value_diff).powf(market_value_weight)) *
            ((1.0 + absolute_goal_diff).powf(gamma)) * (real_player_one_score - one_expected);
        let player_two_new_rate: f64 = player_two.rating + k_factor * w_division[0] * ((1.0 + absolute_market_value_diff).powf(market_value_weight)) *
            ((1.0 + absolute_goal_diff).powf(gamma)) * (real_player_two_score - two_expected);
        (CustomRating{rating: player_one_new_rate}, CustomRating{rating: player_two_new_rate})
    }
}

pub fn expected_score(player_one: &CustomRating, player_two: &CustomRating, config: &RunConfig) -> (f64, f64, f64) {
    let RunConfig {k_factor, gamma, home_advantage, home_field_advantage_weight, market_value_weight, tie_frequency, w_division} = config.clone();
    let exponent: f64 = (player_two.rating - player_one.rating - home_advantage) / 400.0;
    let denominator: f64 = (10 as f64).powf(exponent) + (10 as f64).powf(-1.0 * exponent) + tie_frequency;
    let exp_one: f64 = (10 as f64).powf(exponent) / denominator;
    let exp_two = (10 as f64).powf(-1.0 * exponent) / denominator;
    let exp_tie: f64 = 1.0 - exp_one - exp_two;
    (exp_tie, exp_one, exp_two)
}
