use serde::de;
use serde::{Deserialize, Deserializer};

#[derive(Debug, Deserialize)]
pub struct OfferData {
    /// Price level to be updated
    #[serde(deserialize_with = "de_float_from_str")]
    pub price: f32,
    /// Quantity
    #[serde(deserialize_with = "de_float_from_str")]
    pub quantity: f32,
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
