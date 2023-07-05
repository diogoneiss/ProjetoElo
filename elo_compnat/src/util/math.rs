use std::collections::HashMap;

pub fn mean(numbers: &Vec<f64>) -> Option<f64> {
    if numbers.is_empty() {
        None
    } else {
        let sum: f64 = numbers.iter().sum();
        let count = numbers.len() as f64;
        Some(sum / count)
    }
}
pub fn transpose_matrix<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>> {
    assert!(!v.is_empty());
    let len = v[0].len();
    let mut iters: Vec<_> = v.into_iter().map(|n| n.into_iter()).collect();
    (0..len)
        .map(|_| {
            iters
                .iter_mut()
                .map(|n| n.next().unwrap())
                .collect::<Vec<T>>()
        })
        .collect()
}



pub fn calculate_rmse(elo_diffs: &HashMap<String, f64>, season_match_count: Option<u32>) -> f64 {
    let mut sum = 0.0;

    let n = match season_match_count {
        Some(n) => n,
        None => elo_diffs.len() as u32,
    };

    for (_, diff) in elo_diffs.iter() {
        let diff_squared = diff.powi(2);
        
        if diff_squared.is_infinite() {
            sum = f64::MAX;
            break;
        }
        sum += diff_squared
    }

    let mean = sum / n as f64;

    mean.sqrt()
}