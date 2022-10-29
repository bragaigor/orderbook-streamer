use anyhow::Result;
use grpc_server::orderbook::{Level, Summary};
use tokio::sync::{
    broadcast::{self, Receiver, Sender},
    mpsc,
};

use crate::server::grpc_server::{self, ResultSummary};

use super::{
    consts::{CHANNEL_BUFFER_LIMIT, MAX_PAIR_EXCHANGE},
    errors::OrderbookError,
    mapper::{Exchange, OfferData},
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
    _chan_recv: Receiver<OrderbookMessage>,
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
            _chan_recv: chan_recv,
        }
    }

    /// Spawns two threads that will be listening for orders in 2 different exchanges:
    /// - Binance
    /// - Bitstamp
    /// Additionaly they'll be sending orderbooks through a multi-producer, multi-consumer
    /// broadcast queue so that we can combine and order the data.
    pub async fn run(self) -> Result<Sender<OrderbookMessage>> {
        let cloned_symbol = self.symbol.clone();
        let chan_send_cloned = self.chan_send.clone();

        tokio::spawn(async move { binance_data_listen(cloned_symbol, chan_send_cloned).await });

        // We need to clone them again as the above ones got moved
        let cloned_symbol = self.symbol.clone();
        let chan_send_cloned = self.chan_send.clone();
        tokio::spawn(async move { bitstamp_data_listen(cloned_symbol, chan_send_cloned).await });

        Ok(self.chan_send)
    }

    /// Receiver loop. Always listens and waits for messages and call handle_message to process messages accordingly
    pub async fn broadcast_handle(
        client_id: String,
        mut chan_recv: Receiver<OrderbookMessage>,
        chan_send: mpsc::Sender<ResultSummary>,
    ) -> Result<()> {
        log::info!(
            "Stream Server ready to stream. Connected to client: {}",
            &client_id
        );

        while let Ok(msg) = chan_recv.recv().await {
            let summary = StreamService::handle_message(&msg)?;

            if let Err(error) = chan_send.send(Ok(summary)).await {
                log::debug!(
                    "Failed to send broadcast message. No clients available. Error: {:?}",
                    error
                );
                break;
            }
        }

        log::info!("Stream Server closed connection to client: {}", &client_id);

        Ok(())
    }

    pub(crate) fn handle_message(msg: &OrderbookMessage) -> Result<Summary, OrderbookError> {
        let (mut asks, mut bids, exchange) = match msg {
            OrderbookMessage::Message { message } => {
                let asklen = message.asks.len();
                let bidlen = message.bids.len();
                log::debug!(
                    "Received message for {:?} with {} asks and {} bids",
                    message.exchange,
                    asklen,
                    bidlen
                );

                (message.asks.clone(), message.bids.clone(), message.exchange)
            }
        };

        let (converted_asks, converted_bids) =
            StreamService::sort_and_convert(&mut asks, &mut bids, &exchange);

        let spread = converted_asks[0].price - converted_bids[0].price;

        Ok(Summary {
            spread,
            bids: converted_bids,
            asks: converted_asks,
        })
    }

    /// Helper to sort the asks based on price and convert them to a Vec of Levels to be send to a client
    pub(crate) fn sort_and_convert(
        asks: &mut [OfferData],
        bids: &mut [OfferData],
        exchange: &Exchange,
    ) -> (Vec<Level>, Vec<Level>) {
        asks.sort_by(|order_l, order_r| order_l.price.partial_cmp(&order_r.price).unwrap());
        bids.sort_by(|order_l, order_r| order_r.price.partial_cmp(&order_l.price).unwrap());

        let converted_asks = StreamService::convert_to_levels(asks, exchange);
        let converted_bids = StreamService::convert_to_levels(bids, exchange);

        (converted_asks, converted_bids)
    }

    /// Helper to convert asks and prices to Level Struct to be sent via gRPC
    fn convert_to_levels(securities: &mut [OfferData], exchange: &Exchange) -> Vec<Level> {
        securities
            .iter_mut()
            .take(MAX_PAIR_EXCHANGE)
            .map(|bid| Level {
                amount: bid.quantity as f64,
                exchange: exchange.to_string(),
                price: bid.price as f64,
            })
            .collect()
    }
}
