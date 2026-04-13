mod channel;
mod event;
mod rules;
mod state;

use crate::channel::{EventQueue, ScoredEventQueue};
use crate::event::{Event, ScoredEvent};
use crate::rules::score_event;
use crate::state::UserState;

use serde_json;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::net::TcpListener;
use tokio::sync::Mutex;

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();
    let state: Arc<Mutex<HashMap<u64, UserState>>> = Arc::new(Mutex::new(HashMap::new()));
    println!("Listener started on 127.0.0.1:8080!");

    loop {
        let (socket, _) = listener.accept().await.unwrap();
        let state = Arc::clone(&state);

        let (eq, mut eq_recv) = EventQueue::new(1000);
        let (seq, mut seq_recv) = ScoredEventQueue::new(1000);

        tokio::spawn(async move {
            let reader = BufReader::new(socket);
            let mut lines = reader.lines();

            while let Ok(Some(line)) = lines.next_line().await {
                println!("Got: {}", line);
                let c: Event = serde_json::from_str(&line).unwrap();
                println!("Got: {:?}", c);
                eq.push(c).await;
            }
        });

        tokio::spawn(async move {
            while let Some(event) = eq_recv.recv().await {
                let mut state = state.lock().await;
                println!("Got: {:?}", event);
                let user_state = state.entry(event.customer_id).or_default();

                let (scored_val, flags) = score_event(&event, &user_state);
                println!("I got a score of: {}", scored_val);
                println!("My flags were: {:?}", flags);

                user_state.add(&event);

                let scored_event = ScoredEvent {
                    event: event.clone(),
                    score: scored_val,
                    flags: flags,
                };
                seq.push(scored_event).await;
            }
        });

        tokio::spawn(async move {
            while let Some(scored) = seq_recv.recv().await {
                println!(
                    "I got a scored event: {}",
                    serde_json::to_string(&scored).unwrap()
                );
            }
        });
    }
}
