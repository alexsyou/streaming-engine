use crate::event::Event;
use crate::event::ScoredEvent;
use tokio::sync::mpsc;

// Individual queue for Event structs
pub struct EventQueue {
    sender: mpsc::Sender<Event>,
}

impl EventQueue {
    pub fn new(buffer: usize) -> (Self, mpsc::Receiver<Event>) {
        let (tx, rx) = mpsc::channel(buffer);

        (Self { sender: tx }, rx)
    }

    pub async fn push(&self, event: Event) {
        let _ = self.sender.send(event).await;
    }
}

// Individual queue for ScoredEvent structs
pub struct ScoredEventQueue {
    sender: mpsc::Sender<ScoredEvent>,
}

impl ScoredEventQueue {
    pub fn new(buffer: usize) -> (Self, mpsc::Receiver<ScoredEvent>) {
        let (tx, rx) = mpsc::channel(buffer);

        (Self { sender: tx }, rx)
    }

    pub async fn push(&self, scored_event: ScoredEvent) {
        let _ = self.sender.send(scored_event).await;
    }
}
