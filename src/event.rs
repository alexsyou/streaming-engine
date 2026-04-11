use serde::{Serialize, Deserialize}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Event {
    pub id: u64,
    pub timestamp: u64,
    pub customer_id: u64,
    pub terminal_id: u64,
    pub amount: i64,
    pub fraud: u2,
}
