use futures_util::{SinkExt, StreamExt};
use serde_json::json;
use tokio::sync::mpsc::Sender;
use tokio_tungstenite::{
    connect_async,
    tungstenite::{Message, Result},
};
use url::Url;

use crate::models::{
    consts::{BINANCE_WS_API, BITSTAMP_WS_API},
    mapper::{BinanceStreamData, BitstampData},
};

use super::messages::{BidsAsks, CryptoMessage};

/// Binance streamer
pub async fn binance_data_listen(symbol: String, chan_send: Sender<CryptoMessage>) -> Result<()> {
    let url = format!("{}/ws/{}@depth20@100ms", BINANCE_WS_API, &symbol);
    println!("going to listen URL: {}", &url);
    let url = Url::parse(&url).expect("Bad URL!! AAAAH");
    let (mut ws_stream, _) = connect_async(url).await?;

    while let Some(msg) = ws_stream.next().await {
        let msg = msg?;

        let msg_str = msg.into_text()?;

        let parsed: BinanceStreamData =
            serde_json::from_str(&msg_str).expect("Can't parse binance data");
        // TODO: Do something with the data
        // println!("Got Binance message: {:#?}", parsed);

        send_binance_data(parsed, &chan_send).await?;
    }

    // TODO: Make it more dynamic since this is unreachable code at the moment
    ws_stream.close(None).await?;

    Ok(())
}

/// Helper to send binance data to mpsc channel
async fn send_binance_data(
    data: BinanceStreamData,
    chan_send: &Sender<CryptoMessage>,
) -> Result<()> {
    let message = CryptoMessage::BinanceMessage {
        message: Box::new(BidsAsks {
            asks: data.asks,
            bids: data.bids,
        }),
    };

    if let Err(error) = chan_send.try_send(message) {
        log::error!("Failed trying to send Binance message: {:?}", error);
    }

    Ok(())
}

/// Bitstamp streamer
pub async fn bitstamp_data_listen(symbol: String, chan_send: Sender<CryptoMessage>) -> Result<()> {
    println!("going to listen URL: {}", BITSTAMP_WS_API);
    let url = Url::parse(BITSTAMP_WS_API).expect("Bad URL!! AAAAH");
    let (mut ws_stream, _) = connect_async(url).await?;

    let order_book = format!("order_book_{}", &symbol);

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
        // println!("Got bitstamp message: {:#?}", parsed);

        if parsed.data.timestamp.is_some() {
            send_bitstamp_data(parsed, &chan_send).await?;
        }
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

/// Helper to send binance data to mpsc channel
async fn send_bitstamp_data(data: BitstampData, chan_send: &Sender<CryptoMessage>) -> Result<()> {
    let message = CryptoMessage::BitstampMessage {
        message: Box::new(BidsAsks {
            asks: data.data.asks.unwrap(),
            bids: data.data.bids.unwrap(),
        }),
    };

    if let Err(error) = chan_send.try_send(message) {
        log::error!("Failed trying to send Bitstamp message: {:?}", error);
    }

    Ok(())
}
