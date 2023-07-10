use crate::{util::game::{Game, GameResult}, experimentation::run_config::CustomRating};
use std::collections::HashMap;

use skillratings::elo::EloRating;

#[derive(Default, Debug)]
pub struct TeamStats {
    pub goals_scored: u16,
    pub goals_conceded: u16,
    pub wins: u16,
    pub draws: u16,
    pub losses: u16,
    pub played: u16,
    pub points: u16,
}

pub struct LeagueTable<'a> {
    pub table: HashMap<&'a str, TeamStats>,
    pub year: u16,
    pub division: u8,
    pub league: String,
}

impl<'a> LeagueTable<'a> {
    pub fn new(games: &'a Vec<Game>, name: &str, division: &u8) -> Self {
        let mut table = HashMap::new();

        for game in games {
            let home_name = game.home.as_str();
            let away_name = game.away.as_str();

            let home_stats = table.entry(home_name).or_insert(TeamStats::default());
            home_stats.played += 1;
            home_stats.goals_scored += game.home_score;
            home_stats.goals_conceded += game.away_score;

            let away_stats = table.entry(away_name).or_insert(TeamStats::default());
            away_stats.played += 1;
            away_stats.goals_scored += game.away_score;
            away_stats.goals_conceded += game.home_score;

            match game.result {
                GameResult::H => {
                    let home_stats = table.get_mut(home_name).unwrap();
                    home_stats.wins += 1;
                    home_stats.points += 3;

                    let away_stats = table.get_mut(away_name).unwrap();
                    away_stats.losses += 1;
                }
                GameResult::A => {
                    let home_stats = table.get_mut(home_name).unwrap();
                    home_stats.losses += 1;

                    let away_stats = table.get_mut(away_name).unwrap();
                    away_stats.wins += 1;
                    away_stats.points += 3;
                }
                GameResult::D => {
                    let home_stats = table.get_mut(home_name).unwrap();
                    home_stats.draws += 1;
                    home_stats.points += 1;

                    let away_stats = table.get_mut(away_name).unwrap();
                    away_stats.draws += 1;
                    away_stats.points += 1;
                }
            }
        }

        LeagueTable {
            year: games[0].year,
            division: *division,
            league: String::from(name),
            table,
        }
    }

    pub fn rank(&self) -> Vec<(&'a str, &TeamStats)> {
        let mut teams: Vec<_> = self.table.iter().map(|(a, b)| (*a, b)).collect();

        teams.sort_by(|(_, a_stats), (_, b_stats)| {
            let a_goal_diff = a_stats.goals_scored as i32 - a_stats.goals_conceded as i32;
            let b_goal_diff = b_stats.goals_scored as i32 - b_stats.goals_conceded as i32;

            // Compare teams based on points, then goal difference, then goals scored.
            // Reverse because we want to sort in descending order.
            b_stats
                .points
                .cmp(&a_stats.points)
                .then_with(|| b_goal_diff.cmp(&a_goal_diff))
                .then_with(|| b_stats.goals_scored.cmp(&a_stats.goals_scored))
        });

        teams
    }

    pub fn print_final_table(&self) {
        let ranked_teams = self.rank();

        println!(
            "League Table for the year {}: Division {} of {}",
            self.year, self.division, self.league
        );

        println!(
            "{:<5} {:<20} {:<7} {:<5} {:<5} {:<7} {:<14} {:<14} {:<8}",
            "Rank",
            "Team",
            "Points",
            "Wins",
            "Draws",
            "Losses",
            "Goals Scored",
            "Goals Conceded",
            "Played"
        );

        for (index, (team_name, team_stats)) in ranked_teams.iter().enumerate() {
            println!(
                "{:<5} {:<20} {:<7} {:<5} {:<5} {:<7} {:<14} {:<14} {:<8}",
                index + 1, // Add 1 to the index because enumerate starts at 0
                team_name,
                team_stats.points,
                team_stats.wins,
                team_stats.draws,
                team_stats.losses,
                team_stats.goals_scored,
                team_stats.goals_conceded,
                team_stats.played
            );
        }
    }

    pub fn print_final_table_with_elo(&self, elo_ratings: &HashMap<String, CustomRating>) {
        let ranked_teams = self.rank();

        println!(
            "League Table for the year {}: Division {} of {}",
            self.year, self.division, self.league
        );

        println!(
            "{:<5} {:<8} {:<20} {:<7} {:<5} {:<5} {:<7} {:<14} {:<14} {:<8} ",
            "Rank",
            "Elo",
            "Team",
            "Points",
            "Wins",
            "Draws",
            "Losses",
            "Goals Scored",
            "Goals Conceded",
            "Played",
        );

        for (index, (team_name, team_stats)) in ranked_teams.iter().enumerate() {
            let elo_rating = elo_ratings
                .get(*team_name)
                .cloned()
                .unwrap_or(CustomRating::new());

            let elo_value = elo_rating.rating;
            println!(
                "{:<5} {:<8} {:<20} {:<7} {:<5} {:<5} {:<7} {:<14} {:<14} {:<8} ",
                index + 1, // Add 1 to the index because enumerate starts at 0
                format!("{:.2}", elo_value),
                team_name,
                team_stats.points,
                team_stats.wins,
                team_stats.draws,
                team_stats.losses,
                team_stats.goals_scored,
                team_stats.goals_conceded,
                team_stats.played,
            );
        }
    }
}
