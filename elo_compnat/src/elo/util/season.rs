use crate::util::game::Game;
use std::collections::HashMap;
use std::collections::HashSet;

#[derive(Debug, Clone)]
pub struct Season {
    pub year: u16,
    pub division: u8,
    pub league: String,
    pub matches: Vec<Game>,
}

impl Season {
    pub fn new(year: u16, name: &str, division: &u8) -> Self {
        Season {
            year,
            division: *division,
            league: String::from(name),
            matches: Vec::new(),
        }
    }
}

pub type SeasonMap = HashMap<u16, Season>;

//TODO: Make this use a borrow instead of taking ownership
pub fn construct_seasons(games: &[Game]) -> SeasonMap {
    let mut seasons: SeasonMap = HashMap::new();

    for game in games {
        let year = game.year;
        let season = seasons
            .entry(year)
            .or_insert(Season::new(year, "Brasileirão", &1));
        season.matches.push(game.clone());
    }

    seasons
}

pub fn get_seasons_in_season_map(seasons: &SeasonMap) -> Vec<u16> {
    let mut seasons_vec: Vec<u16> = Vec::new();

    for year in seasons.keys() {
        seasons_vec.push(*year);
    }

    seasons_vec.sort_unstable();
    seasons_vec
}

pub fn get_all_teams_in_season_map(seasons: &SeasonMap) -> Vec<String> {
    let mut teams = HashSet::new();

    for season in seasons.values() {
        for game in &season.matches {
            teams.insert(game.home.clone());
            teams.insert(game.away.clone());
        }
    }

    // Converter o hashset para um vetor e ordenar lexicograficamente
    let mut teams_vec: Vec<String> = teams.into_iter().collect();
    teams_vec.sort_unstable();
    teams_vec
}
