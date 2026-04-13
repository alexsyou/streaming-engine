use super::{Rule, RuleScore};

use crate::event::Event;
use crate::state::UserState;

/// Rule for large events: some events with a high value must be suspicious
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::event::Event;
    use crate::state::UserState;

    fn make_event(amount: f64) -> Event {
        Event {
            id: 1,
            timestamp: 100,
            customer_id: 1,
            terminal_id: 1,
            amount,
            fraud: 0,
        }
    }

    #[test]
    fn small_amount_no_trigger() {
        let rule = SizeRule {
            max_size: 250.0,
            suspicion: 0.2,
        };
        let result = rule.evaluate(&make_event(100.0), &UserState::default());
        assert!(!result.triggered);
    }

    #[test]
    fn large_amount_triggers() {
        let rule = SizeRule {
            max_size: 250.0,
            suspicion: 0.2,
        };
        let result = rule.evaluate(&make_event(500.0), &UserState::default());
        assert!(result.triggered);
        assert_eq!(result.score, 0.2);
    }
}
