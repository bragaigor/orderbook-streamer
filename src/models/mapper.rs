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
pub struct StreamData {
    pub last_update_id: usize,
    /// Bids to be updated
    pub bids: Vec<OfferData>,
    /// Asks to be updated
    pub asks: Vec<OfferData>,
}

/// Helper to convert the returned floats which are encapsulated between quotes AKA strings to actual floats
pub fn de_float_from_str<'a, D>(deserializer: D) -> Result<f32, D::Error>
where
    D: Deserializer<'a>,
{
    let str_val = String::deserialize(deserializer)?;
    str_val.parse::<f32>().map_err(de::Error::custom)
}
