use crate::event::Event;
use std::collections::VecDeque;

#[derive(Default)]
pub struct UserState {
    pub recents: VecDeque<Event>,
    pub total_amount: f64,
    pub count: u64,
}

impl UserState {
    pub fn add(&mut self, event: &Event) {
        self.recents.push_back(event.clone());
        self.total_amount += event.amount;
        self.count += 1;
    }
}
