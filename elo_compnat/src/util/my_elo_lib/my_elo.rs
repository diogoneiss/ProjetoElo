use crate::skillRatings::Outcomes;
use std::collections::HashMap;

/* definições da struct MyRating (renomear dps) */
pub struct MyRating {
    pub rating: f64
}

impl MyRating {
    pub const fn new() -> Self {
        Self { rating: 1000.0}
    }
}

impl Default for MyRating {
    fn default() -> Self {
        Self::new()
    }
}

impl From<f64> for MyRating {
    fn from(r: f64) -> Self {
        Self { rating: r }
    }
}

/* definicoes da struct de config */
pub struct MyConfig {

    pub config_values: HashMap<&str, f64>,
    pub k0: f64,
    pub wi: f64,
    pub gamma: f64,
    pub c: f64,
    pub d: f64,
    pub hfa: f64
}

impl MyConfig {

    pub const fn new() -> Self {
        Self {
            k0: 1.0,
            importance: 1.0,
            gamma: 0.0,
            c: 10.0,
            d: 400.0
        }
    }
}

impl Default for MyConfig {
    fn default() -> Self {
        Self::new()
    }
}


/* struct para calculos com MyElo */

pub struct MyElo {
    config: MyConfig
}

impl RatingSystem for MyElo {
    fn new(config: MyConfig) -> Self {
        Self { config }
    }

    fn rate(
        &self,
        player_one: &MyRating,
        player_two: &MyRating,
        outcome: &Outcomes,
        config: &MyConfig,
        home_field_advantage: Option<f64>,
        absolute_goal_diff: Option<f64>,
        absolute_market_value_diff: Option<f64>
    ) -> (MyRating, MyRating) {

        let hfa: f64 = match home_field_advantage {
            Some(n) => n,
            None => 0,
        };
        
        /* absolute goal diff check */
        let abd: f64 = match absolute_goal_diff {
            Some(d) => d,
            None => 0.0
        };

        /* absolute market value check */
        let amvd: f64 = match absolute_market_value_diff {
            Some(d) => d,
            None => 0.0
        };

        let (one_expected, two_expected) = expected_score(player_one, player_two, config, hfa);
        let real_player_one_score: f64 = match outcome {
            &Outcomes::WIN => 1.0,
            &Outcomes::DRAW => 0.5,
            &Outcomes::LOSS => 0.0
        };
        let real_player_two_score: f64 = 1.0 - real_player_one_score;
        let player_one_new_rate: f64 = player_one.rate + k0 * importance * ((1 + amvd).powf(zeta)) *
            ((1 + agd).powf(gamma)) * (real_player_one_score - one_expected);
        let player_two_new_rate: f64 = player_two.rate + k0 * importance * ((1 + amvd).powf(zeta)) *
            ((1 + agd).powf(gamma)) * (real_player_two_score - two_expected);
        (MyRating{rate: player_one_new_rate}, MyRating{rate: player_two_new_rate})
    }
}


pub fn expected_score(player_one: &FifaRating, player_two: &FifaRating, config: &MyConfig, home_field_advantage: f64) -> (f64, f64) {
    let MyConfig {k0, importance, gamma, c, d} = config;
    let exponent: f64 = (player_two.rate - player_one.rate - home_field_advantage) / d;
    let exp_one: f64 = 1.0 / (1.0 + c.powf(exponent));
    let exp_two = 1.0 - exp_one;
    (exp_one, exp_two)
}