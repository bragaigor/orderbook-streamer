use futures_util::StreamExt;
use std::str;
use tokio_tungstenite::{connect_async, tungstenite::Result};
use url::Url;

use crate::models::mapper::StreamData;

const BINANCE_WS_API: &str = "wss://stream.binance.com:9443";

/// Binance streamer
pub async fn get_data_binance(symbol: &str) -> Result<()> {
    let url = format!("{}/ws/{}@depth20@100ms", BINANCE_WS_API, symbol);
    println!("going to listen URL: {}", &url);
    let url = Url::parse(&url).expect("Bad URL!! AAAAH");
    let (mut ws_stream, _) = connect_async(url).await?;

    while let Some(msg) = ws_stream.next().await {
        let msg = msg?;

        let msg_str = msg.into_text()?;

        let parsed: StreamData = serde_json::from_str(&msg_str).expect("Can't parse");
        // TODO: Do something with the data
        println!("Got message: {:#?}", parsed);
    }

    // TODO: Make it more dynamic since this is unreachable code at the moment
    ws_stream.close(None).await?;

    Ok(())
}

