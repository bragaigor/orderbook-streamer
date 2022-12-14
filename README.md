# This is the all new Orderbook Streamer

## Requirements

### System requirements
On Mac
```bash
brew install protobuf
```

On Linux
```bash
apt update
apt install protobuf-compiler
```

### Rust requirements
- Rust version used: 1.63.0

## Running the server
Please make sure you have all the above requirements installed. The step-by-step below uses `ethbtc` as the example market. If you'd like to run with any other currency pair just replace `ethbtc` with your choice.

### 1. Clone the repo and `cd` into it
```bash
git clone https://github.com/bragaigor/orderbook-streamer.git
cd orderbook-streamer/
```

### 2. Make sure you have 2 or 3 different terminal opened. In one of them run:
```bash
cargo run -- server -s ethbtc
```
The above will create a gRPC server that will listen for "ethbtc" market from both Binance and Bitstamp exchanges and broadcast it's merged sorted orderbooks.
---
If you want to see warning logs run the following instead:
```bash
RUST_LOG=info cargo run -- server -s ethbtc
```

### 3. [Optional] In the other terminal run:
```bash
RUST_LOG=info cargo run -- client
```
The nice thing about this implementation is that we can have n numbers of clients listening to the same server since we're using multi-producer, multi-consumer broadcast queue.

### 3.1 Try oppening another terminal and run the above command and you'll see the exact same messages coming through 👌