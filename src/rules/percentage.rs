use super::{Rule, RuleScore};

use crate::event::Event;
use crate::state::UserState;

pub struct PercentageRule {
    pub odd_pct: f64,
}

impl Rule for PercentageRule {
    fn evaluate(&self, event: &Event, state: &UserState) -> RuleScore {
        let new_pct = event.amount / state.total_amount;

        let triggered = new_pct > self.odd_pct;

        RuleScore {
            triggered,
            score: if triggered { 0.4 } else { 0.0 },
            flag: "percentage",
        }
    }
}
