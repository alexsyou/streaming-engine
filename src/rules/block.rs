use super::{Rule, RuleScore};

use crate::event::Event;
use crate::state::UserState;

/// Rule for blocked sites: some sites have no valid transactions
pub struct BlockRule {
    pub illegal: Vec<u64>,
}

impl Rule for BlockRule {
    fn evaluate(&self, event: &Event, _state: &UserState) -> RuleScore {
        let triggered = self.illegal.iter().any(|&i| i == event.terminal_id);
        RuleScore {
            triggered,
            score: if triggered { 1.0 } else { 0.0 },
            flag: "block",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::event::Event;
    use crate::state::UserState;

    fn make_event(terminal_id: u64) -> Event {
        Event {
            id: 1,
            timestamp: 100,
            customer_id: 1,
            terminal_id,
            amount: 50.0,
            fraud: 0,
        }
    }

    #[test]
    fn safe_terminal_no_trigger() {
        let rule = BlockRule {
            illegal: vec![2, 5],
        };
        let result = rule.evaluate(&make_event(1), &UserState::default());
        assert!(!result.triggered);
    }

    #[test]
    fn blocked_terminal_triggers() {
        let rule = BlockRule {
            illegal: vec![2, 5],
        };
        let result = rule.evaluate(&make_event(2), &UserState::default());
        assert!(result.triggered);
    }
}
