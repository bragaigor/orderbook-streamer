use anyhow::Result;
use futures_util::{SinkExt, StreamExt};
use serde_json::json;
use tokio::sync::broadcast::Sender;
use tokio_tungstenite::{connect_async, tungstenite::Message};
use url::Url;

use crate::models::{
    consts::{
        BINANCE_WS_API, BITSTAMP_WS_API, DEPTH_LEVEL_BINANCE, ERR_COUNT_LOG, UPDATE_SPEED_BINANCE,
    },
    mapper::{BinanceStreamData, BitstampData},
};

use super::{
    mapper::Exchange,
    messages::{OrderbookMessage, Orders},
};

/// Binance streamer
/// 1. Connects to the Binance Web Socket which already contains the orderbook that we want to subscribe to
/// 2. Indefinitely listens for bitstamp orderbooks
/// 2.1 For each orderbook received it sends over the broadcast channel to be agregated and ordered by our server
pub async fn binance_data_listen(
    symbol: String,
    chan_send: Sender<OrderbookMessage>,
) -> Result<()> {
    let url = format!(
        "{}/ws/{}@{}@{}",
        BINANCE_WS_API, &symbol, DEPTH_LEVEL_BINANCE, UPDATE_SPEED_BINANCE
    );
    log::info!("Listening for Binance orderbooks at: {}", &url);
    let url = Url::parse(&url).expect("Bad Binance URL!");
    let (mut ws_stream, _) = connect_async(url).await?;

    let mut err_count = 0;

    while let Some(msg) = ws_stream.next().await {
        let msg = msg?;

        let msg_str = msg.into_text()?;

        let parsed: BinanceStreamData =
            serde_json::from_str(&msg_str).expect("Can't parse binance data");

        if send_binance_data(parsed, &chan_send).await.is_err() {
            err_count += 1;
        }

        if err_count > ERR_COUNT_LOG {
            log::warn!(
                "Failed to send {} binance orderbooks. No receivers found.",
                ERR_COUNT_LOG
            );
            err_count = 0;
        }
    }

    // Unreachable code but we like to do the right thing and close the stream eventually :)
    ws_stream.close(None).await?;

    Ok(())
}

/// Helper to send binance data to broadcast channel
async fn send_binance_data(
    data: BinanceStreamData,
    chan_send: &Sender<OrderbookMessage>,
) -> Result<()> {
    let message = OrderbookMessage::Message {
        message: Box::new(Orders {
            exchange: Exchange::Binance,
            asks: data.asks,
            bids: data.bids,
        }),
    };

    chan_send.send(message)?;

    Ok(())
}

/// Bitstamp streamer.
/// 1. Connects to the bitstamp Web Socket
/// 2. Subscribes to an orderbook
/// 3. Indefinitely listens for bitstamp orderbooks
/// 3.1 For each orderbook received it sends over the broadcast channel to be agregated and ordered by our server
pub async fn bitstamp_data_listen(
    symbol: String,
    chan_send: Sender<OrderbookMessage>,
) -> Result<()> {
    log::info!("Listening for Bitstamp orderbooks at: {}", BITSTAMP_WS_API);
    let url = Url::parse(BITSTAMP_WS_API).expect("Bad Bitstamp URL!");
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

    let mut err_count = 0;

    while let Some(msg) = ws_stream.next().await {
        let msg = msg?;

        let msg_str = msg.into_text()?;

        let parsed: BitstampData =
            serde_json::from_str(&msg_str).expect("Can't parse bitstamp data");

        if parsed.data.timestamp.is_some() && send_bitstamp_data(parsed, &chan_send).await.is_err()
        {
            err_count += 1;
        }

        if err_count > ERR_COUNT_LOG {
            log::warn!(
                "Failed to send {} bitstamp orderbooks. No receivers found.",
                ERR_COUNT_LOG
            );
            err_count = 0;
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

    // Unreachable code but we like to do the right thing and close the stream eventually :)
    ws_stream.close(None).await?;

    Ok(())
}

/// Helper to send binance data to mpsc channel
async fn send_bitstamp_data(
    data: BitstampData,
    chan_send: &Sender<OrderbookMessage>,
) -> Result<()> {
    let message = OrderbookMessage::Message {
        message: Box::new(Orders {
            exchange: Exchange::Bitstamp,
            asks: data.data.asks.unwrap(),
            bids: data.data.bids.unwrap(),
        }),
    };

    chan_send.send(message)?;

    Ok(())
}
