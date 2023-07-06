use std::collections::HashMap;

use crate::util::game::{Game, GameResult};



pub fn calculate_points(games: &[Game]) -> HashMap<String, i32> {
    let mut scores: HashMap<String, i32> = HashMap::new();
    
    for game in games {
        let home = game.home.to_string();
        let away = game.away.to_string();
        match game.result {
            GameResult::H => {
                *scores.entry(home).or_insert(0) += 3;
            }
            GameResult::A => {
                *scores.entry(away).or_insert(0) += 3;
            }
            GameResult::D => {
                *scores.entry(home).or_insert(0) += 1;
                *scores.entry(away).or_insert(0) += 1;
            }
        }
    }
    
    scores
}