use anyhow::Result;
use grpc_server::orderbook::{Level, Summary};
use tokio::sync::{
    broadcast::{self, Receiver, Sender},
    mpsc,
};

use crate::server::grpc_server::{self, ResultSummary};

use super::{
    consts::CHANNEL_BUFFER_LIMIT,
    errors::OrderbookError,
    messages::OrderbookMessage,
    stream::{binance_data_listen, bitstamp_data_listen},
};

pub mod orderbook {
    tonic::include_proto!("orderbook");
}

pub struct StreamService {
    pub symbol: String,
    /// Private sender that sends message to channel
    chan_send: Sender<OrderbookMessage>,
    /// Private reciever that gets the messages sent by send
    chan_recv: Receiver<OrderbookMessage>,
}

impl StreamService {
    pub fn new(symbol: Option<String>) -> Self {
        let symbol = if let Some(symbol) = symbol {
            symbol
        } else {
            dotenv::var("ORDERBOOK_SYMBOL").expect("could not find env var ORDERBOOK_SYMBOL")
        };

        StreamService::init_service(symbol)
    }

    /// Initializes the service that spawns orderbook threads
    fn init_service(symbol: String) -> StreamService {
        let (chan_send, chan_recv) = broadcast::channel::<OrderbookMessage>(CHANNEL_BUFFER_LIMIT);
        StreamService {
            symbol,
            chan_send,
            chan_recv,
        }
    }

    pub async fn run(self) -> Result<Sender<OrderbookMessage>> {
        let mut tasks = vec![];

        let cloned_symbol = self.symbol.clone();
        let chan_send_cloned = self.chan_send.clone();

        tasks.push(tokio::spawn(async move {
            binance_data_listen(cloned_symbol, chan_send_cloned).await
        }));

        // We need to clone them again as the above ones got moved
        let cloned_symbol = self.symbol.clone();
        let chan_send_cloned = self.chan_send.clone();
        tasks.push(tokio::spawn(async move {
            bitstamp_data_listen(cloned_symbol, chan_send_cloned).await
        }));

        // TODO: Should we wait for tasks somewhere?
        // // Wait for all
        // let results = futures::future::join_all(tasks).await;
        // for res in results {
        //     res??;
        // }

        Ok(self.chan_send)
    }

    /// Receiver loop. Always listens and waits for messages and call handle_message to process messages accordingly
    pub async fn mpsc_handle(
        mut chan_recv: Receiver<OrderbookMessage>,
        chan_send: mpsc::Sender<ResultSummary>,
    ) -> Result<()> {
        // let mut recv = chan_recv.lock().await;
        // Wait for messages and then process them

        loop {
            if let Ok(msg) = chan_recv.recv().await {
                let summary = StreamService::handle_message(&msg).await?;

                // TODO: Merge and sort asks and bids. Should we use an internal cache or should we sort each order book as they come?
                chan_send.send(Ok(summary)).await.unwrap();
            } else {
                // Break out of the loop once all clients are destroyed
                break;
            }
        }

        log::info!("Strem Service is now shutdown");

        Ok(())
    }

    async fn handle_message(msg: &OrderbookMessage) -> Result<Summary, OrderbookError> {
        let (asks, bids) = match msg {
            OrderbookMessage::BinanceMessage { message } => {
                let asklen = message.asks.len();
                let bidlen = message.bids.len();
                log::warn!(
                    "Received message for Binance with {} asks and {} bids",
                    asklen,
                    bidlen
                );

                (message.asks.clone(), message.bids.clone())
            }
            OrderbookMessage::BitstampMessage { message } => {
                let asklen = message.asks.len();
                let bidlen = message.bids.len();
                log::warn!(
                    "Received message for Bitstamp with {} asks and {} bids",
                    asklen,
                    bidlen
                );

                (message.asks.clone(), message.bids.clone())
            }
        };

        // TODO: Clean this for sorting
        let askss = asks
            .into_iter()
            .map(|ask| Level {
                amount: ask.quantity as f64,
                exchange: "".to_owned(),
                price: ask.price as f64,
            })
            .collect();

        let bidss = bids
            .into_iter()
            .map(|bid| Level {
                amount: bid.quantity as f64,
                exchange: "".to_owned(),
                price: bid.price as f64,
            })
            .collect();

        Ok(Summary {
            spread: 10 as f64,
            bids: bidss,
            asks: askss,
        })
    }
}
