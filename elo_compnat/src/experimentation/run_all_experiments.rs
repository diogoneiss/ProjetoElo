use crate::{elo::{
    train::{
        construct_elo_table_for_time_series,
        EloTable,
        print_elo_table
    },
    util::{season},
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
    //let mut draw_frequency: Vec<Vec<f64>> = Vec::new();

    for i in 0..experiment_config.random_variations {
        // we set the seed for the random number generator at the simulation function, with i as its seed
        
        let mut errors_per_season: Vec<f64> = Vec::new();
        let mut last_season_config = elo_config.clone();

        let mut elo_table = elo_table_at_start.clone();
        //let mut tie_frequency = vec![];

        for s_year in start_t..=end_t {
            //TODO: perform n random variations, with unique seeds

            let season = seasons_map.get(&s_year).unwrap();
            let season_games = &season.matches;
            let (rmse, simulated_elo, real_elo, season_config) = super::run_single_experiment::run_season_experiment(
                season_games,
                &elo_table,
                &last_season_config,
                experiment_config,
                i as u32,
            );

            last_season_config = season_config;
            //tie_frequency.push(last_season_config.tie_frequency);

            elo_table = real_elo;

            errors_per_season.push(rmse);
        }
        errors_for_each_run.push(errors_per_season);
        //draw_frequency.push(tie_frequency);

    }


    let season_errors = transpose_matrix(errors_for_each_run);

    let mean_errors_for_each_season = season_errors
        .iter()
        .map(|errors| mean(errors).unwrap_or_else(|| {1000.0}))
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
