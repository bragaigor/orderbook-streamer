use crypto_streamer::models::stream::{get_data_binance, get_data_bitstamp};

#[tokio::main]
async fn main() {
    // TODO: Make it dynamic
    let symbol = "ethbtc";
    println!("Hello, world!");

    // get_data_binance(symbol).await.expect("get data for binance failed!!!!");
    get_data_bitstamp(symbol)
        .await
        .expect("get data for bitstamp failed!!!!");

    // TODO: Cache incoming data from stream and sort them on the go.
    //       - Use Max Heap for performance
}
