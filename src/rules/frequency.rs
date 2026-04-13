use super::{Rule, RuleScore};

use crate::event::Event;
use crate::state::UserState;

pub struct FrequencyRule {
    pub max_count: u64,
    pub window_size: u64,
}

impl Rule for FrequencyRule {
    fn evaluate(&self, event: &Event, state: &UserState) -> RuleScore {
        let count = state
            .recents
            .iter()
            .filter(|e| event.timestamp.saturating_sub(e.timestamp) <= self.window_size)
            .count();

        let triggered = count as u64 > self.max_count;
        RuleScore {
            triggered,
            score: if triggered { 0.3 } else { 0.0 },
            flag: "frequency",
        }
    }
}
