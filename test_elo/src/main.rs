use elo_compnat;

fn main() {
    let param = elo_compnat::experimentation::run_config::RunHyperparameters::default();
    elo_compnat::run(param);
}
