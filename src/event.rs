use serde::{Deserialize, Serialize};

/// A faux finanical transaction event (ex. credit card transaction)
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Event {
    /// ID of event
    pub id: u64,
    /// Timestamp of event
    pub timestamp: u64,
    /// ID of customer who generated the event
    pub customer_id: u64,
    /// ID of terminal where the event was generated
    pub terminal_id: u64,
    /// Amount of value passed through the event
    pub amount: f64,
    /// Truth of whether an event was fraud
    pub fraud: u8,
}

/// A scored version of faux financial events
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ScoredEvent {
    /// The event that is being scoreds
    pub event: Event,
    /// The score for the event from 0.0 - 1.0 where 0.0 means no fraud and 1.0 means fraud
    pub score: f64,
    /// Any rules that were tripped by our rule engine
    pub flags: Vec<String>,
}
