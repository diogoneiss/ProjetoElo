use crate::skillRatings::Outcomes;
use std::collections::HashMap;

/* definições da struct CustomRating (renomear dps) */
pub struct CustomRating {
    pub rating: f64
}

impl CustomRating {
    pub const fn new() -> Self {
        Self { rating: 1000.0}
    }
}

impl Default for CustomRating {
    fn default() -> Self {
        Self::new()
    }
}

/* definicoes da struct de config */
pub struct CustomConfig {
    pub k0: f64,
    pub importance: f64,
    pub gamma: f64,
    pub zeta: f64,
    pub c: f64,
    pub d: f64
}

impl CustomConfig {

    pub const fn new(config_params: HashMap<&str, f64>) -> Self {
        Self {
            k0: match config_params.get("k0"){
                Some(v) => v,
                None => 1.0
            },

            importance: match config_params.get("importance"){
                Some(v) => v,
                None => 1.0
            },
            gamma: match config_params.get("gamma"){
                Some(v) => v,
                None => 0.0
            },
            zeta: match config_params.get("zeta"){
                Some(v) => v,
                None => 0.0
            },
            c: match config_params.get("c"){
                Some(v) => v,
                None => 10.0
            },
            d: match config_params.get("d"){
                Some(v) => v,
                None => 400.0
            }
        }
    }
}

impl Default for CustomConfig {
    fn default() -> Self {
        let base_config: HashMap<&str, f64> = HashMap.from([
            ("k", 1.0),
            ("importance", 1.0),
            ("gamma", 0.0),
            ("zeta", 0.0),
            ("c", 10.0),
            ("d", 400.0)
        ]);
        Self::new(base_config)
    }
}


/* struct para calculos com CustomElo */

pub struct CustomElo {
    config: CustomConfig
}

impl RatingSystem for CustomElo {
    fn new(config: CustomConfig) -> Self {
        Self { config }
    }

    fn rate(
        &self,
        player_one: &CustomRating,
        player_two: &CustomRating,
        outcome: &Outcomes,
        config: &CustomConfig,
        home_field_advantage: Option<f64>,
        absolute_goal_diff: Option<f64>,
        absolute_market_value_diff: Option<f64>
    ) -> (CustomRating, CustomRating) {

        let hfa: f64 = match home_field_advantage {
            Some(n) => n,
            None => 0.0,
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
        (CustomRating{rate: player_one_new_rate}, CustomRating{rate: player_two_new_rate})
    }
}


pub fn expected_score(player_one: &FifaRating, player_two: &FifaRating, config: &CustomConfig, home_field_advantage: f64) -> (f64, f64) {
    let CustomConfig {k0, importance, gamma, c, d} = config;
    let exponent: f64 = (player_two.rate - player_one.rate - home_field_advantage) / d;
    let exp_one: f64 = 1.0 / (1.0 + c.powf(exponent));
    let exp_two = 1.0 - exp_one;
    (exp_one, exp_two)
}