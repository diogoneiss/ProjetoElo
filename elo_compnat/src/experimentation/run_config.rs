use std::f64::EPSILON;

use serde::{Deserialize, Serialize};

// this struct holds the necessary parameters for configuring the runtime of our experiments
// It is also used as the genotype, as it holds all the experimentation parameters
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RunConfig {
    pub k_factor: f64,
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
pub struct RunHyperparameters {
    pub starting_elo: u32,
    pub starting_year: u16,
    pub backtest_years: u16,
    pub random_variations: u16,
    pub use_goals_diff: bool,
    pub use_home_advantage: bool,
    pub use_market_values: bool,
    pub leagues_to_use: u16,
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
