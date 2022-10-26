use serde::{Deserialize, Serialize};

use super::mapper::OfferData;

/// Message that will be sent to our agregator. We use a multi-producer,
/// multi-consumer broadcast queue to send messages since we need to merge
/// data from different order books.
#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum OrderbookMessage {
    ///
    BinanceMessage {
        message: Box<BidsAsks>,
    },
    BitstampMessage {
        message: Box<BidsAsks>,
    },
}

/// Struct to hold the "buy" and "sell"s of a certain orderbook
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct BidsAsks {
    /// Bids to be updated
    pub bids: Vec<OfferData>,
    /// Asks to be updated
    pub asks: Vec<OfferData>,
}
