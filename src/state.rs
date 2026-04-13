use crate::event::Event;
use std::collections::VecDeque;

/// Values related to a specific user and their events
#[derive(Default)]
pub struct UserState {
    /// All recent events (within 300 time)
    pub recents: VecDeque<Event>,
    /// Total amount the user has processed within 300 time
    pub total_amount: f64,
    /// Count of how many events the user has within 300 time
    pub count: u64,
}

impl UserState {
    pub fn add(&mut self, event: &Event) {
        self.recents.push_back(event.clone());

        while let Some(front) = self.recents.front() {
            if event.timestamp.saturating_sub(front.timestamp) > 300 {
                let old = self.recents.pop_front().unwrap();
                self.total_amount -= old.amount;
                self.count -= 1;
            } else {
                break;
            }
        }

        self.total_amount += event.amount;
        self.count += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::event::Event;

    fn make_event(timestamp: u64, amount: f64) -> Event {
        Event {
            id: 1,
            timestamp,
            customer_id: 1,
            terminal_id: 1,
            amount,
            fraud: 0,
        }
    }

    #[test]
    fn evicts_old_events() {
        let mut state = UserState::default();
        state.add(&make_event(100, 50.0));
        state.add(&make_event(200, 75.0));
        assert_eq!(state.recents.len(), 2);

        // this should evict the first event (500 - 100 > 300)
        state.add(&make_event(500, 25.0));
        assert_eq!(state.recents.len(), 2); // 200 and 500
        assert_eq!(state.count, 2);
    }

    #[test]
    fn keeps_recent_events() {
        let mut state = UserState::default();
        state.add(&make_event(100, 50.0));
        state.add(&make_event(150, 75.0));
        state.add(&make_event(200, 25.0));
        assert_eq!(state.recents.len(), 3);
        assert_eq!(state.count, 3);
    }
}
