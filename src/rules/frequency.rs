use super::{Rule, RuleScore};

use crate::event::Event;
use crate::state::UserState;

/// Rule for high frequency events: too many events in a timeframe are suspicious
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::event::Event;
    use crate::state::UserState;

    fn make_event(timestamp: u64) -> Event {
        Event {
            id: 1,
            timestamp,
            customer_id: 1,
            terminal_id: 1,
            amount: 50.0,
            fraud: 0,
        }
    }

    #[test]
    fn under_threshold() {
        let rule = FrequencyRule {
            max_count: 3,
            window_size: 60,
        };
        let mut state = UserState::default();
        state.add(&make_event(10));
        state.add(&make_event(20));

        let result = rule.evaluate(&make_event(30), &state);
        assert!(!result.triggered);
    }

    #[test]
    fn over_threshold() {
        let rule = FrequencyRule {
            max_count: 3,
            window_size: 60,
        };
        let mut state = UserState::default();
        state.add(&make_event(10));
        state.add(&make_event(20));
        state.add(&make_event(30));
        state.add(&make_event(40));

        let result = rule.evaluate(&make_event(50), &state);
        assert!(result.triggered);
    }

    #[test]
    fn old_events_no_trigger() {
        let rule = FrequencyRule {
            max_count: 3,
            window_size: 60,
        };
        let mut state = UserState::default();
        state.add(&make_event(1));
        state.add(&make_event(2));
        state.add(&make_event(3));
        state.add(&make_event(4));

        // New event should not trigger, other events are old
        let result = rule.evaluate(&make_event(500), &state);
        assert!(!result.triggered);
    }
}
