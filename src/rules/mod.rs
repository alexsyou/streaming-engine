pub mod frequency;
pub mod geo;

use crate::event::Event;
use crate::state::UserState;

pub struct RuleScore {
    pub triggered: bool,
    pub score: f64,
    pub flag: &'static str,
}

pub trait Rule {
    fn evaluate(&self, event: &Event, state: &UserState) -> RuleScore;
}

pub fn score_event(event: &Event, state: &UserState) -> (f64, Vec<&'static str>) {
    let rules: Vec<Box<dyn Rule>> = vec![
        Box::new(geo::GeoRule { illegal: vec![2] }),
        Box::new(frequency::FrequencyRule {
            max_count: 3,
            window_size: 60,
        }),
    ];

    let mut total_score = 0.0;
    let mut flags = Vec::new();

    for rule in rules {
        let result = rule.evaluate(event, state);
        if result.triggered {
            total_score += result.score;
            flags.push(result.flag);
        }
    }

    total_score = total_score.min(1.0);

    (total_score, flags)
}
