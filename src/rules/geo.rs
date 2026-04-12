use super::{Rule, RuleScore};

use crate::event::Event;
use crate::state::UserState;

pub struct GeoRule {
    pub illegal: Vec<u64>,
}

impl Rule for GeoRule {
    fn evaluate(&self, event: &Event, _state: &UserState) -> RuleScore {
        let triggered = self.illegal.iter().any(|&i| i == event.terminal_id);
        RuleScore {
            triggered,
            score: if triggered { 1.0 } else { 0.0 },
            flag: "geography",
        }
    }
}
