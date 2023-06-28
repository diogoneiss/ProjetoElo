use pyo3::prelude::*;


mod elo;
mod experimentation;
mod util;

use elo::train::construct_elo_table_for_time_series;
use elo::util::league::LeagueTable;

use experimentation::compare_simulation::run_experiments;
use experimentation::run_config;
use util::game::Game;
use std;

#[pyfunction]
pub fn run() -> PyResult<()>{
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
    println!("Calling get_data");
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

    Ok(partidas.into_py(py))
}

#[pyfunction]
pub fn process_data(py: Python, data: PyObject) -> PyResult<()> {
    let data: Vec<Game> = data.extract(py)?;
    println!("Data: {:?}", data[0]);
    // Process your data here
    Ok(())
}



/// Formats the sum of two numbers as string.
#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}

/// A Python module implemented in Rust.
#[pymodule]
fn elo_compnat(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    m.add_function(wrap_pyfunction!(run, m)?)?;
    m.add_function(wrap_pyfunction!(get_data, m)?)?;
    m.add_function(wrap_pyfunction!(process_data, m)?)?;
    Ok(())
}