use std::f64::EPSILON;
use pyo3::exceptions::PyValueError;
use pyo3::{prelude::*, types::PyDict};
use pyo3::types::IntoPyDict as IntoPyDict;
use serde::{Deserialize, Serialize};

// TODO: incluir parametros novos aqui dentro
/// this struct holds the necessary parameters for configuring the runtime of our experiments
/// It is also used as the genotype, as it holds all the experimentation parameters
#[derive(Debug, Clone, Deserialize, Serialize)]
#[pyclass]
pub struct RunConfig {
    pub k_factor: f64,
}
#[pymethods]
impl RunConfig {
    #[new]
    fn new(k_factor: f64) -> RunConfig {
        RunConfig {
            k_factor
        }
    }
}

// These trait implementations will probably be required to perform the genetic algorithm operations
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
        RunConfig { k_factor: 20.0 }
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
        write!(f, "{} {} {}", self.starting_elo, self.starting_year, self.backtest_years)
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
            return Err(PyValueError::new_err("The input list should have exactly 8 elements."));
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

