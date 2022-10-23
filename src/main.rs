use crypto_streamer::models::stream::get_data_binance;

#[tokio::main]
async fn main() {
    // TODO: Make it dynamic
    let symbol = "ethbtc";
    println!("Hello, world!");

    get_data_binance(symbol).await.expect("get data failed!!!!");

    // TODO: Cache incoming data from stream and sort them on the go.
    //       - Use Max Heap for performance
}
