use futures_util::{SinkExt, StreamExt};
use std::str;
use tokio_tungstenite::{
    connect_async,
    tungstenite::{Error, Result},
};
use url::Url;

const BINANCE_WS_API: &str = "wss://stream.binance.com:9443";

#[tokio::main]
async fn main() {
    // TODO: Make it dynamic
    let symbol = "ethbtc";
    println!("Hello, world!");

    get_data(symbol).await.expect("get data failed!!!!");

    // TODO: Cache incoming data from stream and sort them on the go.
    //       - Use Max Heap for performance
}

async fn get_data(symbol: &str) -> Result<()> {
    let url = Url::parse(&format!("{}/ws/{}@depth20@100ms", symbol, BINANCE_WS_API))
        .expect("Bad URL!! AAAAH");
    let (mut ws_stream, _) = connect_async(url).await?;

    while let Some(msg) = ws_stream.next().await {
        let msg = msg?;
        // TODO: Do something with the data
        println!("Got message: {:?}", msg);
    }

    // TODO: Make it more dynamic since this is unreachable code at the moment
    ws_stream.close(None).await?;

    Ok(())
}
