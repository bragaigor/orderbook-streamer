use futures_util::{SinkExt, StreamExt};
use serde_json::json;
use std::str;
use tokio_tungstenite::{
    connect_async,
    tungstenite::{Message, Result},
};
use url::Url;

use crate::models::mapper::{BinanceStreamData, BitstampData};

const BINANCE_WS_API: &str = "wss://stream.binance.com:9443";
const BITSTAMP_WS_API: &str = "wss://ws.bitstamp.net";

/// Binance streamer
pub async fn get_data_binance(symbol: &str) -> Result<()> {
    let url = format!("{}/ws/{}@depth20@100ms", BINANCE_WS_API, symbol);
    println!("going to listen URL: {}", &url);
    let url = Url::parse(&url).expect("Bad URL!! AAAAH");
    let (mut ws_stream, _) = connect_async(url).await?;

    while let Some(msg) = ws_stream.next().await {
        let msg = msg?;

        let msg_str = msg.into_text()?;

        let parsed: BinanceStreamData =
            serde_json::from_str(&msg_str).expect("Can't parse binance data");
        // TODO: Do something with the data
        println!("Got message: {:#?}", parsed);
    }

    // TODO: Make it more dynamic since this is unreachable code at the moment
    ws_stream.close(None).await?;

    Ok(())
}

pub async fn get_data_bitstamp(symbol: &str) -> Result<()> {
    println!("going to listen URL: {}", BITSTAMP_WS_API);
    let url = Url::parse(BITSTAMP_WS_API).expect("Bad URL!! AAAAH");
    let (mut ws_stream, _) = connect_async(url).await?;

    let order_book = format!("order_book_{}", symbol);

    let subscribe_msg = json!({
        "event": "bts:subscribe",
        "data": {
            "channel": order_book
        }
    });

    let message = Message::Text(subscribe_msg.to_string());
    ws_stream.send(message).await?;

    while let Some(msg) = ws_stream.next().await {
        let msg = msg?;

        let msg_str = msg.into_text()?;

        let parsed: BitstampData =
            serde_json::from_str(&msg_str).expect("Can't parse bitstamp data");
        // TODO: Do something with the data
        println!("Got message: {:#?}", parsed);
    }

    let unsubscribe_msg = json!({
        "event": "bts:unsubscribe",
        "data": {
            "channel": order_book
        }
    });
    let message = Message::Text(unsubscribe_msg.to_string());
    ws_stream.send(message).await?;

    // TODO: Make it more dynamic since this is unreachable code at the moment
    ws_stream.close(None).await?;

    Ok(())
}
