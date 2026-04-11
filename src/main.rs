mod event;

use crate::event::Event;
use serde_json;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::net::TcpListener;
use tokio_stream::{StreamExt, wrappers::TcpListenerStream};

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();
    println!("Listener started on 127.0.0.1:8080!");

    loop {
        let (socket, _) = listener.accept().await.unwrap();

        tokio::spawn(async move {
            let reader = BufReader::new(socket);
            let mut lines = reader.lines();

            while let Ok(Some(line)) = lines.next_line().await {
                println!("Got: {}", line);
                let c: Event = serde_json::from_str(&line).unwrap();
                println!("Got: {:?}", c);
            }
        });
    }
}
