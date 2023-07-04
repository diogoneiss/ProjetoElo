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

use super::{run_all_experiments, run_config};

/// Given an starting elo and matches, simulates the season and compares it to the real season and the real match results, returning the elo difference table
pub fn run_season_experiment(
    season_games: &Vec<Game>,
    starting_elo: &EloTable,
    run_config: &run_config::RunConfig,
    experiment_config: &run_config::RunHyperparameters,
    random_seed: u32,
) -> (f64, EloTable, EloTable) {
    let (elo_simulated, simulated_matches, config_after_run) = simulate_season(
        season_games,
        starting_elo,
        run_config,
        experiment_config,
        random_seed,
    );

    let elo_config = run_config.clone();

    let real_elo =
        construct_elo_table_for_year(season_games, Some(starting_elo.clone()), Some(&elo_config), experiment_config);

    let tabela_fake = LeagueTable::new(&simulated_matches, "Brasileirão", &1);
    let tabela = LeagueTable::new(season_games, "Brasileirão", &1);

    // uncomment this to see the ending table
    //tabela.print_final_table();
    //println!("--------------- Elo simulated ----------- \n");
    // tabela_fake.print_final_table();

    //calculate distance between real and simulated elo
    let elo_diff = compare_elo_tables(&real_elo, &elo_simulated);

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
        let simulated_elo = simulated_elo.get(team);
        let diff = match simulated_elo {
            Some(sim_elo) => elo.rating - sim_elo.rating,
            None => {println!("A zero appeared! team: {:?}", &team ); 0.0},
        };
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

    if n == 0 {
        panic!("n is 0");
    }

    let mean: f64 = sum / n as f64;

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
