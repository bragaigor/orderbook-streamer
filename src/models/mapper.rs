use enum_display_derive::Display;
use serde::de;
use serde::{Deserialize, Deserializer, Serialize};
use std::fmt;
use std::fmt::Display;

use crate::client::grpc_client::orderbook::{Level, Summary};

pub mod orderbook {
    tonic::include_proto!("orderbook");
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OfferData {
    /// Price level to be updated
    #[serde(deserialize_with = "de_float_from_str")]
    pub price: f32,
    /// Quantity
    #[serde(deserialize_with = "de_float_from_str")]
    pub quantity: f32,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, Display)]
pub enum Exchange {
    Binance,
    Bitstamp,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BinanceStreamData {
    pub last_update_id: usize,
    /// Bids to be updated
    pub bids: Vec<OfferData>,
    /// Asks to be updated
    pub asks: Vec<OfferData>,
}

#[derive(Debug, Deserialize)]
pub struct BitstampOfferData {
    /// Price level to be updated
    #[serde(deserialize_with = "de_float_from_str")]
    pub price: f32,
    /// Quantity
    #[serde(deserialize_with = "de_float_from_str")]
    pub quantity: f32,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BitstampStreamData {
    #[serde(default, deserialize_with = "de_usize_from_str")]
    pub timestamp: Option<usize>,
    #[serde(default, deserialize_with = "de_usize_from_str")]
    pub microtimestamp: Option<usize>,
    /// Bids to be updated
    pub bids: Option<Vec<OfferData>>,
    /// Asks to be updated
    pub asks: Option<Vec<OfferData>>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BitstampData {
    pub data: BitstampStreamData,
    /// Bids to be updated
    pub channel: String,
    /// Asks to be updated
    pub event: String,
}

/// Helper to convert the returned floats which are encapsulated between quotes AKA strings
/// to actual float types
pub fn de_float_from_str<'a, D>(deserializer: D) -> Result<f32, D::Error>
where
    D: Deserializer<'a>,
{
    let str_val = String::deserialize(deserializer)?;
    str_val.parse::<f32>().map_err(de::Error::custom)
}

/// Helper to convert the returned numbers which are encapsulated between quotes AKA strings
/// to actual usize types
pub fn de_usize_from_str<'a, D>(deserializer: D) -> Result<Option<usize>, D::Error>
where
    D: Deserializer<'a>,
{
    let str_val = String::deserialize(deserializer)?;
    let num = str_val.parse::<usize>().map_err(de::Error::custom)?;
    Ok(Some(num))
}

// These are structs used to beautify Client's output

/// Equivalent of Level struct but used to output data
pub struct LevelOutput {
    exchange: String,
    price: f64,
    amount: f64,
}

impl fmt::Debug for LevelOutput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "\t\texchange: \"{}\", price: {}, amount: {}",
            self.exchange, self.price, self.amount
        )
    }
}

impl From<Level> for LevelOutput {
    /// Convert from a bucket set to a WSBucket for easy database insertion
    fn from(level: Level) -> Self {
        LevelOutput {
            exchange: level.exchange,
            price: level.price,
            amount: level.amount,
        }
    }
}

/// Equivalent of Summary struct but used to output data
pub struct SummaryOutput {
    pub spread: f64,
    pub asks: Vec<LevelOutput>,
    pub bids: Vec<LevelOutput>,
}

impl fmt::Debug for SummaryOutput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{{\n\tspread: \"{}\",\n\tasks: {:#?},\n\tbids: {:#?}\n}}",
            self.spread, self.asks, self.bids
        )
    }
}

impl From<Summary> for SummaryOutput {
    /// Convert from a bucket set to a WSBucket for easy database insertion
    fn from(summary: Summary) -> Self {
        let asks = summary
            .asks
            .into_iter()
            .map(|ask| LevelOutput::from(ask))
            .collect();

        let bids = summary
            .bids
            .into_iter()
            .map(|bid| LevelOutput::from(bid))
            .collect();

        SummaryOutput {
            spread: summary.spread,
            asks,
            bids,
        }
    }
}
