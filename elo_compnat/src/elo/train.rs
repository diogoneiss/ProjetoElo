use std::collections::HashMap;

use crate::experimentation::run_config;
use crate::util::game::Game;

use super::util::season::{construct_seasons, get_seasons_in_season_map, SeasonMap};

use super::super::{CustomElo, CustomRating, RunConfig};

pub type RankedMatch = (CustomRating, skillratings::Outcomes);
pub type EloTable = HashMap<String, CustomRating>;

const DEBUG_INFO: bool = false;

pub fn construct_elo_table_for_year(
    partidas: &Vec<Game>,
    starting_elos: Option<EloTable>,
    elo_config: Option<&RunConfig>,
    run_hyperparameters: &run_config::RunHyperparameters,
) -> EloTable {
    // Construir tabela de elo se vier vazia
    let mut elo_table = match starting_elos {
        Some(elos) => elos,
        None => HashMap::new(),
    };

    let default_config = RunConfig::default();
    let elo_config = match elo_config {
        Some(config) => config,
        None => &default_config,
    };

    let mut results_table: HashMap<String, Vec<RankedMatch>> = HashMap::new();

    // Salvar histórico de elo desses times
    for partida in partidas {
        let home_team = partida.home.clone();
        let away_team = partida.away.clone();

        let (home_outcome, away_outcome) = partida.get_match_outcome();

        let current_elo = |team_name: &String| {
            elo_table
                .get(team_name)
                .cloned()
                .unwrap_or_else(|| CustomRating {rating: run_hyperparameters.starting_elo as f64})
        };

        let home_team_elo = current_elo(&home_team);
        let away_team_elo = current_elo(&away_team);


        if home_team_elo.rating.is_nan()  {
            println!("Elo is NaN for home team: {} at match {}", &home_team, partida.week);
            //let results_home = results_table.get(&home_team).unwrap();
           // println!("Results: {:?}", results_home);
        }
        if away_team_elo.rating.is_nan() {
            println!("Elo is NaN for away_team: {} at match {}", &away_team, partida.week);
            //let results_home = results_table.get(&away_team).unwrap();
           // println!("Results: {:?}", results_home);
        }

        // Salvar histórico de resultados desses times e elos
        let mut insert_result = |team_name: &String, current_elo: &CustomRating, outcome| {
            results_table
                .entry(team_name.clone())
                .or_insert(Vec::new())
                .push((*current_elo, outcome));
        };

        insert_result(&home_team, &home_team_elo, home_outcome);
        insert_result(&away_team, &away_team_elo, away_outcome);

        let custom_elo = CustomElo {
            config: elo_config.clone(),
        };

        let absolute_goal_diff: f64 = ((partida.home_score as i16) - (partida.away_score as i16))
            .abs()
            .into();

        let absolute_market_value_diff: f64 = (partida.home_value - partida.away_value)
        .abs();

        let (new_player_home, new_player_away) = custom_elo.rate(
            &home_team_elo,
            &away_team_elo,
            partida.result,
            absolute_goal_diff,
            absolute_market_value_diff,
            partida.division as usize
        );

        elo_table.insert(home_team, new_player_home);
        elo_table.insert(away_team, new_player_away);
    }
    elo_table
}

fn check_time_series_interval(
    match_years_vector: &Vec<u16>,
    desired_range: &std::ops::RangeInclusive<u16>,
) {
    for year in desired_range.clone() {
        if !match_years_vector.contains(&year) {
            let error_msg = format!("Year {} not found in season map. The range was {:?} and the years present are {:?}", year, &desired_range, match_years_vector);

            panic!("{}", error_msg);
        }
    }
}

pub fn construct_elo_table_for_time_series(
    all_matches: &[Game],
    elo_config: Option<&RunConfig>,
    run_hyperparameters: &run_config::RunHyperparameters,
    start_year: u16,
    end_year: u16,
) -> EloTable {
    let default_config = RunConfig::default();
    let elo_config = match elo_config {
        Some(config) => config,
        None => &default_config,
    };

    let seasons_map: SeasonMap = construct_seasons(all_matches);

    let years_in_season_map = get_seasons_in_season_map(&seasons_map);

    //verificar se o vetor é contíguo
    let desired_range: std::ops::RangeInclusive<u16> = start_year..=end_year;

    check_time_series_interval(&years_in_season_map, &desired_range);

    let mut starting_elo_table: Option<EloTable> = None;
    for year in desired_range.into_iter() {
        let season = seasons_map.get(&year).unwrap();
        let partidas = &season.matches;
        let elo_table =
            construct_elo_table_for_year(partidas, starting_elo_table, Some(elo_config), &run_hyperparameters );
        starting_elo_table = Some(elo_table.clone());

        if DEBUG_INFO {
            println!("Elo table for year {}", year);
            print_elo_table(&elo_table, false);
        }
    }

    starting_elo_table.unwrap()
}

pub fn print_elo_table(elo_table: &EloTable, order_elos: bool) {
    let max_team_length = elo_table.keys().map(|team| team.len()).max().unwrap_or(0);

    let max_elo_length = elo_table
        .values()
        .map(|elo| format!("{:.2}", elo.rating).len())
        .max()
        .unwrap_or(0);

    let divider_length = max_team_length + max_elo_length + 5; // Adding 5 to account for the extra characters in the format

    let divider = "-".repeat(divider_length);

    println!("{}", divider);

    // Convert the elo_table to a vector of tuples so it can be sorted.
    let mut table: Vec<_> = elo_table.iter().collect();

    if order_elos {
        // Sort by elo in descending order.
        table.sort_by(|a, b| {
            b.1.rating
                .partial_cmp(&a.1.rating)
                .unwrap_or(std::cmp::Ordering::Equal)
        });
    } else {
        // Sort alphabetically by team name.
        table.sort_by_key(|a| a.0.clone());
    }

    for (team, elo) in table {
        let elo_string = format!("{:.2}", elo.rating);
        println!(
            "| {:<max_team_width$} : {:<max_elo_width$} |",
            team,
            elo_string,
            max_team_width = max_team_length,
            max_elo_width = max_elo_length
        );
    }
    println!("{}", divider);
}