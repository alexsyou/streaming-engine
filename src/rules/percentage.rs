use super::{Rule, RuleScore};

use crate::event::Event;
use crate::state::UserState;

/// Rule for larger than usual transaction: higher than a certain percentage of the users old events
pub struct PercentageRule {
    pub odd_pct: f64,
}

impl Rule for PercentageRule {
    fn evaluate(&self, event: &Event, state: &UserState) -> RuleScore {
        if state.total_amount == 0.0 {
            return RuleScore {
                triggered: false,
                score: 0.0,
                flag: "percentage",
            };
        };

        let new_pct = event.amount / state.total_amount;

        let triggered = new_pct > self.odd_pct;

        RuleScore {
            triggered,
            score: if triggered { 0.4 } else { 0.0 },
            flag: "percentage",
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
    fn no_trigger_on_empty() {
        let rule = PercentageRule { odd_pct: 2.5 };
        let result = rule.evaluate(&make_event(1000.0), &UserState::default());
        assert!(!result.triggered);
    }

    #[test]
    fn typical_event_no_trigger() {
        let rule = PercentageRule { odd_pct: 2.5 };
        let mut state = UserState::default();
        state.add(&make_event(100.0));
        state.add(&make_event(100.0));

        let result = rule.evaluate(&make_event(100.0), &state);
        assert!(!result.triggered);
    }

    #[test]
    fn large_spike_trigger() {
        let rule = PercentageRule { odd_pct: 2.5 };
        let mut state = UserState::default();
        state.add(&make_event(10.0));
        state.add(&make_event(10.0));

        let result = rule.evaluate(&make_event(1000.0), &state);
        assert!(result.triggered);
    }
}
