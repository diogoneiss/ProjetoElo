use std::collections::{HashMap, HashSet};

use crate::{
    elo::train::{construct_elo_table_for_year, EloTable},
    util::math::calculate_rmse,
};

use crate::{experimentation::simulate_season::simulate_season, util::game::Game};

use super::{
    run_config::{self, RunConfig},
    season_standings::calculate_points,
};

/// Given an starting elo and matches, simulates the season and compares it to the real season and the real match results, returning the elo difference table
pub fn run_season_experiment(
    season_games: &Vec<Game>,
    starting_elo: &EloTable,
    run_config: &run_config::RunConfig,
    experiment_config: &run_config::RunHyperparameters,
    random_seed: u32,
) -> (f64, EloTable, EloTable, RunConfig) {
    let (elo_simulated, simulated_matches, config_after_run) = simulate_season(
        season_games,
        starting_elo,
        run_config,
        experiment_config,
        random_seed,
    );

    let elo_config = config_after_run.clone();

    //TODO: retornar as novas partidas nessa função para usar no python, mas nao vai ser pra usar aqui
    let real_elo = construct_elo_table_for_year(
        season_games,
        Some(starting_elo.clone()),
        Some(&elo_config),
        experiment_config,
    );

    //let tabela_fake = LeagueTable::new(&simulated_matches, "Brasileirão", &1);
    //let tabela = LeagueTable::new(season_games, "Brasileirão", &1);

    // uncomment this to see the ending table
    //tabela.print_final_table();
    //println!("--------------- Elo simulated ----------- \n");
    // tabela_fake.print_final_table();

    //calculate distance between real and simulated elo
    let elo_diff = compare_elo_tables(&real_elo, &elo_simulated);
    let points_diff = compare_standing_tables(&season_games, &simulated_matches, false);
    /*
        println!("--------------- Elo diff ----------- \n");
        for (team, diff) in elo_diff.iter() {
            println!("{}: {}", team, diff);
        }
    */
    //let games_count = changed_elos(starting_elo, &elo_simulated);
    let games_count = count_unique_teams(&season_games);
    let rmse_elo_mean = calculate_rmse(&elo_diff, Some(games_count));
    let rmse_points = calculate_rmse(&points_diff, Some(games_count));
    //println!("RMSE with games: {}", rmse_correct_mean);
    //println!("RMSE: {}", rmse_all_teams);

    (rmse_points, elo_simulated, real_elo, config_after_run)
}

fn compare_elo_tables(real_elo: &EloTable, simulated_elo: &EloTable) -> HashMap<String, f64> {
    let mut elo_diff: HashMap<String, f64> = HashMap::new();

    for (team, elo) in real_elo.iter() {
        let simulated_elo = simulated_elo.get(team);
        let diff = match simulated_elo {
            Some(sim_elo) => elo.rating - sim_elo.rating,
            None => {
                println!("A zero appeared! team: {:?}", &team);
                0.0
            }
        };
        elo_diff.insert(team.clone(), diff);
    }

    elo_diff
}

/// Compares two given standing tables (what is displayed at the end of
/// each soccer season, with points and position)
fn compare_standing_tables(real_games: &[Game], simulated_games: &[Game], show_standings: bool) -> HashMap<String, f64> {
    let mut elo_diff: HashMap<String, f64> = HashMap::new();
    let real_table = calculate_points(real_games);
    let simulated_table = calculate_points(simulated_games);

    let filter_games_by_division = |games: &Vec<Game>, division: u8| {
        games.iter().filter(|game| game.division == division).cloned().collect::<Vec<Game>>()
    };

    let print_standings_for_2_divisions = |games: &[Game], title: &str| {
        // Separate games by division
        let division1_games = games.iter().filter(|game| game.division == 1).cloned().collect::<Vec<Game>>();
        let division2_games = games.iter().filter(|game| game.division == 2).cloned().collect::<Vec<Game>>();
    
        // Calculate points
        let division1_table = calculate_points(&division1_games);
        let division2_table = calculate_points(&division2_games);
    
        // Closure to print the table
        let print_table = |table: &HashMap<String, i32>, name: &str| {
            println!("--------------- {}\n", name);
            let mut table_vec: Vec<_> = table.iter().collect();
            table_vec.sort_by(|a, b| b.1.cmp(a.1));
            for (team, points) in table_vec {
                println!("{}: {}", team, points);
            }
            println!("--------------- End of {}\n", name)
        };
    
        // Print tables
        println!("============= {} =============  \n", title);
        print_table(&division1_table, "Division 1");
        print_table(&division2_table, "Division 2");
        println!("============= End of {} =============  \n", title);
    };

    if show_standings {
        print_standings_for_2_divisions(real_games, "Real standings");
        print_standings_for_2_divisions(simulated_games, "Simulated standings");
    }
    



    for (team, points) in real_table.iter() {
        let simulated_points = simulated_table.get(team);
        let diff: i32 = match simulated_points {
            Some(sim_points) => points - sim_points,
            None => {
                //println!("A zero appeared! team: {:?}", &team);
                0
            }
        };
        elo_diff.insert(team.clone(), diff.into());
    }

    elo_diff
}

fn count_unique_teams(games: &[Game]) -> u32 {
    let mut unique_teams = HashSet::new();

    for game in games.iter() {
        let name = game.home.as_str();
        unique_teams.insert(name);
    }

    unique_teams.len() as u32
}

/// Kept for legacy reasons, this function is not used anymore
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
