#![allow(dead_code)]

use pyo3::prelude::*;

mod elo;
pub mod experimentation;
mod util;

use elo::train::construct_elo_table_for_time_series;
use elo::util::league::LeagueTable;

use experimentation::run_config::{self, CustomElo};
use experimentation::{run_all_experiments::run_experiments, run_config::CustomRating};

use util::game::Game;

//TODO: extrair essas duas structs para arquivos separados
use crate::experimentation::run_config::{RunConfig, RunHyperparameters};

#[pyfunction]
pub fn run(parameters: RunHyperparameters, config: Option<&RunConfig>) -> PyResult<()> {
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

    let run_config = match config {
        Some(config) => config.clone(),
        None => run_config::RunConfig::default(),
    };

    
    let errors = run_experiments(&partidas, &run_config, &parameters);

    parameters.print_errors_by_year(&errors);

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
    // Ideal seria resolver dentro do da função de leitura de maneira invisível, deixei um TODO lá
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
/// Wrapper for the run_experiments function, so that it can be called from python and the
/// data parsed
///
pub fn fitness_function(
    py: Python,
    partidas_py: PyObject,
    run_config_py: PyObject,
    hyperparameters_py: PyObject,
) -> PyResult<Vec<f64>> {
    let partidas: Vec<Game> = partidas_py.extract(py)?;
    let run_config: RunConfig = run_config_py.extract(py)?;
    let hyperparameters: RunHyperparameters = hyperparameters_py.extract(py)?;

    println!(
        "Running experiments with hyperparameters: {:?}",
        &hyperparameters
    );
    println!("Genotypes for this run: {:?}", &run_config);
    //println!("1a partida: {:?}", partidas[0]);

    let errors = run_experiments(&partidas, &run_config, &hyperparameters);

    println!("Errors: {:?}", &errors);
    // aqui sairia o erro
    Ok(errors)
}

/// This is the python module definition, everything that you want to use
/// inside python must be declared here
/// Modulo que vai pro python, necessário adicionar as funções e classes que ele vai usar
#[pymodule]
fn elo_compnat(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(run, m)?)?;
    m.add_function(wrap_pyfunction!(get_data, m)?)?;
    m.add_function(wrap_pyfunction!(fitness_function, m)?)?;
    m.add_class::<RunHyperparameters>()?;
    m.add_class::<RunConfig>()?;
    m.add_class::<CustomElo>()?;
    m.add_class::<CustomRating>()?;

    Ok(())
}
