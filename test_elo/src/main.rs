use elo_compnat::{
    self,
    experimentation::run_config::{RunConfig, RunHyperparameters},
};

fn main() {
    let param = RunHyperparameters::default();
    let test_config = RunConfig {
        k_factor: 1.5,
        gamma: 1.2,
        home_advantage: 100.0,
        home_field_advantage_weight: 1.2,
        market_value_weight: 1.2,
        tie_frequency: 0.30,
        w_division: vec![1.2, 1.05],
    };
    elo_compnat::run(param, Some(&test_config));
}
