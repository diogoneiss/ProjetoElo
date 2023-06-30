use pyo3::prelude::*;

mod elo;
pub mod experimentation;
mod util;

use elo::train::construct_elo_table_for_time_series;
use elo::util::league::LeagueTable;

use experimentation::compare_simulation::run_experiments;
use experimentation::run_config;
use std;
use util::game::Game;

use crate::experimentation::run_config::RunHyperparameters;

#[pyfunction]
pub fn run(parameters: RunHyperparameters) -> PyResult<()> {
    println!("\n\nRunning experiments with parameters: {:?}", &parameters);
    //print current directory
    let curr_directory: String = match std::env::current_dir() {
        Ok(path) => path.display().to_string(),
        Err(e) => panic!("Error getting current directory: {}", e),
    };

    println!("Current directory: {}", &curr_directory);

    let mut path = String::from("data/brasileirao.csv");

    // "ProjetoElo" is the last directory in the current_dir path prefix it to path
    if curr_directory.ends_with("ProjetoElo") {
        println!("Current directory ends with ProjetoElo");
        let path2 = String::from("elo_compnat/");
        path = path2 + &path;
    }

    println!("Path to csv: {}", &path);

    let partidas = util::parsing::load_csv(&path)
        .map_err(|e| {
            println!("Erro fazendo parse do csv de partidas: {}", e);
        })
        .unwrap();

    let run_config = run_config::RunConfig::default();
    let experiment_config = run_config::RunHyperparameters::default();

    let errors = run_experiments(&partidas, &run_config, &experiment_config);

    experiment_config.print_errors_by_year(&errors);

    Ok(())
}

/// Use this to get the parsed Vec<game>
#[pyfunction]
pub fn get_data(py: Python) -> PyResult<PyObject> {
    // sim, essa funcao é copiada da run. Ideal seria deixarmos toda a logica de dataset aqui e so passar ele parseado bonitinho pro run
    println!("Getting data..");
    let curr_directory: String = match std::env::current_dir() {
        Ok(path) => path.display().to_string(),
        Err(e) => panic!("Error getting current directory: {}", e),
    };

    println!("Current directory: {}", &curr_directory);

    let mut path = String::from("data/brasileirao.csv");

    // "ProjetoElo" is the last directory in the current_dir path prefix it to path
    // TODO: melhorar esse crime. Se chamamos dentro de elo_compnat, não precisa do prefixo
    // mas se é chamado de dentro de test_elo, precisa do prefixo, pq ele usa a pasta data errada
    if curr_directory.ends_with("ProjetoElo") {
        println!("Current directory ends with ProjetoElo");
        let path2 = String::from("elo_compnat/");
        path = path2 + &path;
    }

    println!("Path to csv: {}", &path);

    let partidas = util::parsing::load_csv(&path)
        .map_err(|e| {
            println!("Erro fazendo parse do csv de partidas: {}", e);
        })
        .unwrap();

    Ok(partidas.into_py(py))
}

#[pyfunction]
/// No momento essa função não faz nada, mas é um exemplo de como chamar uma função rust
/// passando um parâmetro do python (vetor de partidas)
pub fn process_data(py: Python, data: PyObject) -> PyResult<()> {
    let data: Vec<Game> = data.extract(py)?;
    println!("Data: {:?}", data[0]);
    // Process your data here
    Ok(())
}

/// Modulo que vai pro python, necessário adicionar as funções e classes que ele vai usar
#[pymodule]
fn elo_compnat(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(run, m)?)?;
    m.add_function(wrap_pyfunction!(get_data, m)?)?;
    m.add_function(wrap_pyfunction!(process_data, m)?)?;
    m.add_class::<RunHyperparameters>()?;
    Ok(())
}
