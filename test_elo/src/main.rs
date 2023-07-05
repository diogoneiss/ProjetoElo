use elo_compnat::{
    self,
    experimentation::run_config::{RunConfig, RunHyperparameters},
};

fn main() {
    let param = RunHyperparameters::default();
    let test_config = RunConfig {
        k_factor: 2.0,
        gamma: 1.0,
        home_advantage: 1.0,
        home_field_advantage_weight: 1.0,
        market_value_weight: 1.0,
        tie_frequency: 1.0,
        w_division: vec![31.322963842135948, 16.83704798044687],
    };
    elo_compnat::run(param, Some(&test_config));
}
