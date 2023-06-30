use crate::util::game::Game;
use std::error::Error;


pub fn load_csv(path: &str) -> Result<Vec<Game>, Box<dyn Error>> {
    // TODO: colocar gerenciamento de caminhos aqui, ou seja, se ele não encontrar
    // ele tenta prefixar o nome dessa pasta antes do path, de modo que encontre a data
    let mut reader = csv::Reader::from_path(path)?;

    let result: Vec<Game> = reader
        .deserialize()
        .map(|r: Result<Game, csv::Error>| r.unwrap())
        .collect();

    Ok(result)
}

pub fn filter_by_year(games: &[Game], year: u16) -> Vec<Game> {
    games.iter().filter(|p| p.year == year).cloned().collect()
}
