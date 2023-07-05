use crate::{elo::{
    train::{
        construct_elo_table_for_time_series,
        EloTable,
    },
    util::{league::LeagueTable, season},
}, util::math::{mean, transpose_matrix}};

use crate::{util::game::Game};

use super::{run_config::{RunConfig, RunHyperparameters}};

/// Performs the backtesting for t years and experiments with the elo metric for n-t remaining years.
/// Note that the next year is based on the real year, not the simulated one.
pub fn run_experiments(
    all_games: &[Game],
    run_config: &RunConfig,
    experiment_config: &RunHyperparameters,
) -> Vec<f64> {
    // Setup: Configure the required structs
    let elo_config = run_config.clone();
    // Pre processing: split the games into seasons, determine start and end years of backtesting
    let end_year = experiment_config.starting_year + experiment_config.backtest_years;

    let seasons_map = season::construct_seasons(all_games);

    // 1st stage: do the elo training with the desired years of data. this is the backtesting
    let elo_table_at_start = construct_elo_table_for_time_series(
        all_games,
        Some(&elo_config),
        &experiment_config,
        experiment_config.starting_year,
        end_year,
    );

    //Sanity check: assert the correct range. Later this will be refactored outside the experiment run itself
    let min_year = all_games.iter().map(|game| game.year).min().unwrap();
    let max_year = all_games.iter().map(|game| game.year).max().unwrap();
    assert!(experiment_config.starting_year >= min_year);
    assert!(end_year < max_year);

    // 2nd stage: simulate the seasons after the training period, until the end of the dataset
    let start_t = end_year + 1;
    let end_t = *seasons_map.keys().max().unwrap();

    let mut errors_for_each_run: Vec<Vec<f64>> = Vec::new();
    let mut draw_frequency: Vec<Vec<f64>> = Vec::new();

    for i in 0..experiment_config.random_variations {
        // we set the seed for the random number generator at the simulation function, with i as its seed
        
        let mut errors_per_season: Vec<f64> = Vec::new();
        let mut last_season_config = elo_config.clone();

        let mut elo_table = elo_table_at_start.clone();
        let mut tie_frequency = vec![];

        for s_year in start_t..=end_t {
            //TODO: perform n random variations, with unique seeds

            let season = seasons_map.get(&s_year).unwrap();
            let season_games = &season.matches;
            /*
            OUTPUT / elo comparison decisions
            2 options on how to measure the error between expected and simulated elo

            1. ORACLE: Use previous loop simulated elo and apply correct data from the current loop. We will take the t-1 table and apply the respective t real match results, implying that the
            elos were perfectly predicted, but using the elo values generated from previous simulations. Note that this will be used *only* for the error calculation, this elo table
            will be discarded after the error is calculated, such that the correct match results are used only in desired eval season, the other ones are simulated.
            This works as a oracle, which would be capable of predicting the results perfectly, even with bad elo values. Minimizing the error in this case would be equivalent to correctly
            estimating the elo values.


            2. REAL: Real elo table from t-1 period, updated with t period match results.

            Currently we are using option 2, but we should test both. this will require a refactor of the individual experiment function
            */

            /* INPUT decisions

            How to deal with "starting elo"
            2 options on how to feed the "simulated" elo table

            1. PROPAGATED: Use previous loop simulated elo. Feed as the starting elo for the next loop. Will require some sort of exponential moving average to deal with the propagation
            Conceptually is the best approach.
            2. SYNTHETIC: Take the real elo table from time t-1 as input, meaning we recreate this simulated table for every experiment based on real data,
            such that elo errors do not propagate between different seasons

            Currently we are using option 2, but we should test both. This will require a code refactor to deal with the update and refeeding.
            */

            // use elo table as the starting elo for the next season, using it to measure the error as well.
            let (rmse, _, real_elo, season_config) = super::run_single_experiment::run_season_experiment(
                season_games,
                &elo_table,
                &last_season_config,
                experiment_config,
                i as u32,
            );

            // save the season config for the next iteration, with the updated weigths
            last_season_config = season_config;
            tie_frequency.push(last_season_config.tie_frequency);
            // Update the elo tables for the next iteration
            // As we are using option 2, we will use the real elo table for the next season, so it needs to be updated.
            elo_table = real_elo;

            errors_per_season.push(rmse);

        }
        // Save the errors for this experiment in the vector, we will later calculate the mean for this experiment
        errors_for_each_run.push(errors_per_season);
        draw_frequency.push(tie_frequency);

    }

    println!("Finished experiments");
    println!("Draw frequency: ");
    for i in 0..draw_frequency.len() {
        println!("Draw frequency for experiment {}: {:?}", i, draw_frequency[i]);
    }

    //print_elo_table(&elo_table, true);
    // Tivemos que escolher estratégia de "seleção" do valor representativo da run. Será o melhor valor? O mediano? o médio?
    // interessante plottarmos isso pra estudar no python. Acabamos optando por usar o valor médio


    // iterate over the random variations. We need to calculate the mean 

    let season_errors = transpose_matrix(errors_for_each_run);

    let mean_errors_for_each_season = season_errors
        .iter()
        .map(|errors| mean(errors).unwrap_or_else(|| {10000.0}))
        .collect::<Vec<f64>>();
 
    mean_errors_for_each_season
}



fn changed_elos(elo_table: &EloTable, elo_table_after_season: &EloTable) -> u32 {
    let mut changed_elos: u32 = 0;

    for (team, elo) in elo_table.iter() {
        let elo_after_season = elo_table_after_season.get(team).unwrap();
        let diff = elo_after_season.rating - elo.rating;
        if diff != 0.0 {
            changed_elos += 1;
        }
    }

    changed_elos
}
