use super::{Rule, RuleScore};

use crate::event::Event;
use crate::state::UserState;

pub struct SizeRule {
    pub max_size: f64,
    pub suspicion: f64,
}

impl Rule for SizeRule {
    fn evaluate(&self, event: &Event, _state: &UserState) -> RuleScore {
        let triggered = event.amount > self.max_size;

        RuleScore {
            triggered,
            score: if triggered { self.suspicion } else { 0.0 },
            flag: "size",
        }
    }
}
