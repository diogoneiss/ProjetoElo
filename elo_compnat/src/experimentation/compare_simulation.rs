use std::collections::HashMap;

use skillratings::elo::EloConfig;

use crate::elo::{
    train::{
        construct_elo_table_for_time_series, construct_elo_table_for_year, print_elo_table,
        EloTable,
    },
    util::{league::LeagueTable, season},
};

use crate::{experimentation::simulate_season::simulate_season, util::game::Game};

use super::{compare_simulation, run_config};

/// Performs the backtesting for t years and experiments with the elo metric for n-t remaining years.
/// Note that the next year is based on the real year, not the simulated one.
pub fn run_experiments(
    all_games: &[Game],
    run_config: &run_config::RunConfig,
    experiment_config: &run_config::RunHyperparameters,
) -> Vec<f64> {
    // Setup: Configure the required structs
    let elo_config = EloConfig {
        k: run_config.k_factor,
    };

    // Pre processing: split the games into seasons, determine start and end years of backtesting
    let end_year = experiment_config.starting_year + experiment_config.backtest_years;

    let seasons_map = season::construct_seasons(all_games);

    // 1st stage: do the elo training with the desired years of data. this is the backtesting
    let mut elo_table = construct_elo_table_for_time_series(
        all_games,
        Some(&elo_config),
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

    let experimentation_range = start_t..=end_t;

    let mut errors_per_season: Vec<f64> = Vec::new();

    for s_year in experimentation_range.into_iter() {
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
        let (rmse, _, real_elo) =
            run_season_experiment(season_games, &elo_table, run_config, experiment_config, 42);

        // Update the elo tables for the next iteration
        // As we are using option 2, we will use the real elo table for the next season, so it needs to be updated.
        elo_table = real_elo;

        errors_per_season.push(rmse);
    }

    println!("Finished experiments");

    //print_elo_table(&elo_table, true);

    errors_per_season
}

/// Given an starting elo and matches, simulates the season and compares it to the real season and the real match results, returning the elo difference table
/// TODO: include a flag to determine if we should use the real or simulated elo for the next season. Maybe create an enum? This is noted in the code as "NEXT OPTION"
fn run_season_experiment(
    season_games: &Vec<Game>,
    starting_elo: &EloTable,
    run_config: &run_config::RunConfig,
    experiment_config: &run_config::RunHyperparameters,
    random_seed: u32,
) -> (f64, EloTable, EloTable) {
    let (elo_simulated, simulated_matches) = simulate_season(
        season_games,
        starting_elo,
        run_config,
        experiment_config,
        random_seed,
    );

    let elo_config = EloConfig {
        k: run_config.k_factor,
    };

    let real_elo =
        construct_elo_table_for_year(season_games, Some(starting_elo.clone()), Some(&elo_config));

    let tabela_fake = LeagueTable::new(&simulated_matches, "Brasileirão", &1);
    let tabela = LeagueTable::new(season_games, "Brasileirão", &1);

    // uncomment this to see the ending table
    //tabela.print_final_table();
    //println!("--------------- Elo simulated ----------- \n");
    // tabela_fake.print_final_table();

    //calculate distance between real and simulated elo
    let elo_diff = compare_simulation::compare_elo_tables(&real_elo, &elo_simulated);

    /*
        println!("--------------- Elo diff ----------- \n");
        for (team, diff) in elo_diff.iter() {
            println!("{}: {}", team, diff);
        }
    */
    let games_count = changed_elos(starting_elo, &elo_simulated);

    let rmse_correct_mean = calculate_rmse(&elo_diff, Some(games_count));

    //println!("RMSE with games: {}", rmse_correct_mean);
    //println!("RMSE: {}", rmse_all_teams);

    (rmse_correct_mean, elo_simulated, real_elo)
}

fn compare_elo_tables(real_elo: &EloTable, simulated_elo: &EloTable) -> HashMap<String, f64> {
    let mut elo_diff: HashMap<String, f64> = HashMap::new();

    for (team, elo) in real_elo.iter() {
        let simulated_elo = simulated_elo.get(team).unwrap();
        let diff = elo.rating - simulated_elo.rating;
        elo_diff.insert(team.clone(), diff);
    }

    elo_diff
}

fn calculate_rmse(elo_diffs: &HashMap<String, f64>, season_match_count: Option<u32>) -> f64 {
    let mut sum = 0.0;

    let n = match season_match_count {
        Some(n) => n,
        None => elo_diffs.len() as u32,
    };

    for (_, diff) in elo_diffs.iter() {
        sum += diff.powi(2);
    }

    let mean = sum / n as f64;

    mean.sqrt()
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
