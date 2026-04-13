pub mod block;
pub mod frequency;
pub mod percentage;
pub mod size;

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

pub fn score_event(event: &Event, state: &UserState) -> (f64, Vec<String>) {
    let rules: Vec<Box<dyn Rule>> = vec![
        Box::new(block::BlockRule { illegal: vec![2] }),
        Box::new(frequency::FrequencyRule {
            max_count: 3,
            window_size: 60,
        }),
        Box::new(size::SizeRule {
            max_size: 250.0,
            suspicion: 0.2,
        }),
        Box::new(percentage::PercentageRule { odd_pct: 2.5 }),
    ];

    let mut total_score = 0.0;
    let mut flags = Vec::new();

    for rule in rules {
        let result = rule.evaluate(event, state);
        if result.triggered {
            total_score += result.score;
            flags.push(result.flag.to_string());
        }
    }

    total_score = total_score.min(1.0);

    (total_score, flags)
}
