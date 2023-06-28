use crate::util::game::Game;
use std::error::Error;


pub fn load_csv(path: &str) -> Result<Vec<Game>, Box<dyn Error>> {
    //TODO: essa função tem um gargalo imenso que é a leitura e parsing do csv, que é feita para todos os 
    //individuos da população. Precisamos refatorar para essa função só ser usada uma vez, salvarmos os dados
    //de retorno no python e passarmos os dados para as funções de treinamento e simulação por referencia.
    // Pycell faz isso
    
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
