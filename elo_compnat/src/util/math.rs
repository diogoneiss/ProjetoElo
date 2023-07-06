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
    

    let n = match season_match_count {
        Some(n) => n,
        None => elo_diffs.len() as u32,
    };

    let mut sum = 0.0;
    let mut sum_squared = 0.0;

    for (_, diff) in elo_diffs {
        sum += diff;
    }

    let mean = sum / n as f64;

    for (_, diff) in elo_diffs {
        let diff_squared = (diff - mean).powi(2);
        if diff_squared.is_infinite() || diff_squared.is_nan() {
            return f64::MAX;
        }
        sum_squared += diff_squared;
    }

    let variance = sum_squared / n as f64 - mean.powi(2);

    let std = variance.sqrt();
    //println!("mean: {}, variance: {} std: {}", mean, variance, std);

    if std.is_nan() || std < 0.2 {
        return 10000.0;
    } 

    let log_std = std.log(2.0);

    if log_std.is_nan() || log_std.is_infinite() {
        return 1000.0;
    }
    let mse = sum_squared / n as f64;
    // regularize the error using the log of the standard deviation
    //println!("mse: {}, std: {}, log_std: {}", mse, std, log_std);

    if std < 1.0 {
        return mse.sqrt()/log_std;
    }
    mse.sqrt()
}