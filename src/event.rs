use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Event {
    pub id: u64,
    pub timestamp: u64,
    pub customer_id: u64,
    pub terminal_id: u64,
    pub amount: f64,
    pub fraud: u8,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ScoredEvent {
    pub event: Event,
    pub score: f64,
}
