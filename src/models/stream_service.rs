use anyhow::Result;
use tokio::sync::mpsc::{channel, Receiver, Sender};

use super::{
    consts::CHANNEL_BUFFER_LIMIT,
    errors::CryptError,
    messages::CryptoMessage,
    stream::{binance_data_listen, bitstamp_data_listen},
};

pub struct StreamService {
    pub symbol: String,
    /// Private sender that sends message to channel
    chan_send: Sender<CryptoMessage>,
    /// Private reciever that gets the messages sent by send
    chan_recv: Receiver<CryptoMessage>,
}

impl StreamService {
    pub fn new(symbol: Option<String>) -> Self {
        let symbol = if let Some(symbol) = symbol {
            symbol
        } else {
            dotenv::var("CRYPT_SYMBOL").expect("could not find env var CRYPT_SYMBOL")
        };

        StreamService::init_service(symbol)
    }

    fn init_service(symbol: String) -> StreamService {
        let (chan_send, chan_recv) = channel::<CryptoMessage>(CHANNEL_BUFFER_LIMIT);
        StreamService {
            symbol,
            chan_send,
            chan_recv,
        }
    }

    pub async fn run(&mut self) -> Result<()> {
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

        // TODO: Should we spin another Tokio task here?
        self.listen().await?;

        // Wait for all
        let results = futures::future::join_all(tasks).await;
        for res in results {
            res??;
        }

        Ok(())
    }

    /// Receiver loop. Always listens and waits for messages and call handle_message to process messages accordingly
    async fn listen(&mut self) -> Result<()> {
        // Wait for messages and then process them
        loop {
            if let Some(msg) = self.chan_recv.recv().await {
                self.handle_message(&msg).await?;
            } else {
                // Break out of the loop once all clients are destroyed
                break;
            }
        }

        log::info!("Strem Service is now shutdown");

        Ok(())
    }

    async fn handle_message(&mut self, msg: &CryptoMessage) -> Result<(), CryptError> {
        match msg {
            CryptoMessage::BinanceMessage { message } => {
                let asklen = message.asks.len();
                let bidlen = message.bids.len();
                println!(
                    "Received message for Binance with {} asks and {} bids",
                    asklen, bidlen
                );
            }
            CryptoMessage::BitstampMessage { message } => {
                let asklen = message.asks.len();
                let bidlen = message.bids.len();
                println!(
                    "Received message for Bitstamp with {} asks and {} bids",
                    asklen, bidlen
                );
            }
        }
        Ok(())
    }
}
