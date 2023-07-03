use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};

use crate::elo::train::EloTable;
use crate::util::game::{Game, GameResult};

use super::run_config::{RunConfig, RunHyperparameters, CustomElo, CustomRating, expected_score};

pub fn simulate_season(
    games: &[Game],
    original_elos: &EloTable,
    run_config: &RunConfig,
    experiment_config: &RunHyperparameters,
    random_seed: u32,
) -> (EloTable, Vec<Game>, RunConfig) {
    // For each game, simulate the game and update the elo table accordingly. We will also update the games with the results for debugging purposes, so we can
    // print the estimated league table
    // It's important to note that we use the games for the season only for estimation purposes, the real game outcome is not used in the simulation (maybe the goal difference)

    let mut acc_home_elo_variation: f64 = 0.0;
    let mut acc_away_elo_variation: f64 = 0.0;

    let mut acc_tie_frequency: f64 = 0.0;

    // TODO: extrair a liga do game e retirar o peso w_i
    let mut simulated_games: Vec<Game> = games.to_vec();
    let mut starting_elos = original_elos.clone();
    let mut rng = StdRng::seed_from_u64(random_seed as u64);

    // loop over the games
    for (i, game) in games.iter().enumerate() {
        // get the home and away teams from match
        let home = game.home.clone();
        let away = game.away.clone();

        // get the respective elos from the simulated_elos hashmap

        let mut new_elo = CustomRating::new();
        new_elo.rating = experiment_config.starting_elo.into();

        let home_elo = match starting_elos.get(&home) {
            Some(elo) => &*elo,
            None => &new_elo,
        };

        let away_elo = match starting_elos.get(&away) {
            Some(elo) => &*elo,
            None => &new_elo,
        };

        // calculate expected scores
        let (exp_tie, exp_home, _) = expected_score(&home_elo, &away_elo, run_config);

        //generate two random numbers between 0 and 1, determine the winner (or draw) and update the elos
        let random_result: f64 = rng.gen();

        let tie = random_result < exp_tie;
        let home_wins = random_result > exp_tie && random_result < exp_tie + exp_home;
        let away_wins = !(tie || home_wins);

        let mut simulated_game = game.clone();
        let absolute_goal_diff: f64 = ((game.home_score as i8) - (game.away_score as i8)).abs().into();
        let absolute_market_value_diff: f64 = 10.0; // preencher corrertamente conform tabela

        // assign the result to the simulated game according to home team's perspective
        simulated_game.result = match (tie, home_wins, away_wins) {
            (false, true, false) => GameResult::H,
            (false, false, true) => GameResult::A,
            _ => GameResult::D,
        };

        match simulated_game.result {
            GameResult::D => {acc_tie_frequency += 1.0},
            _ => ()
        };

        // hard coded value as we are not using the real game goal difference
        (simulated_game.home_score, simulated_game.away_score) = match simulated_game.result {
            GameResult::H => (1, 0),
            GameResult::A => (0, 1),
            GameResult::D => (1, 1),
        };

        let custom_elo = CustomElo{config: run_config.clone()};

        let (new_player_home, new_player_away) =
            custom_elo.rate(&home_elo, &away_elo, simulated_game.result, absolute_goal_diff, absolute_market_value_diff);

        let home_diff = new_player_home.rating - home_elo.rating;
        let away_diff = new_player_away.rating - away_elo.rating;

        acc_home_elo_variation += home_diff;
        acc_away_elo_variation += away_diff;

        // update elos
        starting_elos.insert(home, new_player_home);
        starting_elos.insert(away, new_player_away);

        // update the ith game in the simulated_games vector with the simulated result
        simulated_games[i] = simulated_game;
    }

    
    let mut config_copy = run_config.clone();
    config_copy.tie_frequency = acc_tie_frequency / (games.len() as f64);
    config_copy.home_advantage += config_copy.home_field_advantage_weight * (acc_home_elo_variation - acc_away_elo_variation);
    (starting_elos, simulated_games.to_vec(), config_copy)
}
