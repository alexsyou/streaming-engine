# Streaming Engine

A real-time stream processing engine in Rust for scoring financial transactions against configurable fraud detection rules.

## Architecture

```
Python Generator ──TCP──► Ingestion Task
                              │
                          Event (mpsc)
                              │
Shared State ◄──────► Scoring Task ◄──── Rules
Arc<Mutex<HashMap>>        │              ├ Block
                      ScoredEvent (mpsc)  ├ Frequency
                           │              ├ Size
                       Output Task        └ Percentage
```

### Components
#### Python Generator
Quick python generator that generates synthetic payment transaction data

#### Ingestion
Ingestion involves listening to streaming events on a TCP port and parsing into events. These events are sent into a task queue to be scored.

#### Scoring
Scoring takes events as well as some state about the user's previous transactions and sends them through a ruling process.
This ruling process applies multiple rules to decide the likelihood of fraud.
These scores are then sent to a task queue to be output.

#### Output
Sends results of scoring to stdout.


## Rules

| Rule     | Trigger Condition              | Score |
|----------|--------------------------------|-------|
|BLOCK     |Accessing blocked terminal      |1      |
|PERCENTAGE|Larger than usual transaction   |0.4    |
|SIZE      |Large transaction               |0.2    |
|FREQUENCY |Many transactions in a row      |0.3    |

## Getting Started

### Prerequisites

```
- Rust (1.70+)
- Python 3 (for the event generator)
```

### Build

```bash
cargo build --release
```

### Run

```bash
cargo run --release
```

### Test

```bash
cargo test
```

## Potential Updates at Scale

- Tasks queues can be split into a more robust interprocess system where individual sections can be scaled up or down as needed
- Rule-based fraud detection can be enhanced with ML-based techniques
- Analysis of rule efficacy can be conducted to improve rules
- Persistent storage of events outside of memory in databases
- Add metrics to observe trends in events and status of streaming engine over time

## Stack

- Rust, Python
