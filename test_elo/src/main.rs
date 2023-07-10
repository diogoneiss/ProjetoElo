use elo_compnat::{
    self,
    experimentation::run_config::{RunConfig, RunHyperparameters}
};

fn main() {

    let x = vec![  2.49518433,   2.79995614,   2.65563014 ,  1.80708322 ,  1.28271224, 0.38,   125.16330199, 115.40319719];
    let hyper_params_list = vec![1000, 2003, 8, 100, 1, 1, 1, 1];

    let desired_config = RunConfig::from_python_list(x.clone());

    println!("Generated config: {:?}", desired_config);

    let desired_hyperparams = RunHyperparameters::from_python_list(hyper_params_list.clone());


    let default_param = RunHyperparameters::default();
    let test_config = RunConfig {
        k_factor: 1.5,
        gamma: 1.2,
        home_advantage: 50.0,
        home_field_advantage_weight: 1.2,
        market_value_weight: 1.2,
        tie_frequency: 0.30,
        w_division: vec![20.0, 10.0],

    };
    let fitness = elo_compnat::fitness_function("brasileirao", x, hyper_params_list);
    //elo_compnat::run(default_param, Some(&test_config)).unwrap();

    match fitness {
        Ok(fit) => println!("Fitness: {:?}", fit),
        Err(e) => println!("Error: {}", e),
    }

    elo_compnat::run(desired_hyperparams, Some(&desired_config)).unwrap();

}
