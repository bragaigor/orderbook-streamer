use serde::{Deserialize, Serialize};

use super::mapper::OfferData;

///
#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum CryptoMessage {
    ///
    BinanceMessage {
        /// Generic Stats metric
        message: Box<BidsAsks>,
    },
    BitstampMessage {
        /// Generic Stats metric
        message: Box<BidsAsks>,
    },
}

///
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct BidsAsks {
    /// Bids to be updated
    pub bids: Vec<OfferData>,
    /// Asks to be updated
    pub asks: Vec<OfferData>,
}
