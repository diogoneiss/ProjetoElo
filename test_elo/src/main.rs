use elo_compnat::{self, experimentation::run_config::{RunConfig, RunHyperparameters}};

fn main() {
    let param = RunHyperparameters::default();
    let test_config = RunConfig { k_factor: 16.0, gamma: 0.5898910102683956, home_advantage: 0.13010318597055903, home_field_advantage_weight: 0.9488855372533332, market_value_weight: 0.9656320330745594, tie_frequency: 0.8083973481164611, w_division: vec![31.322963842135948, 16.83704798044687] };
    elo_compnat::run(param, Some(&test_config));
}
