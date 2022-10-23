use anyhow::Result;
use crypto_streamer::models::stream_service::StreamService;

#[tokio::main]
async fn main() -> Result<()> {
    stackdriver_logger::init_with(
        Some(stackdriver_logger::Service {
            name: "CryptoStream".to_owned(),
            version: "1.0".to_owned(),
        }),
        true,
    );

    // TODO: Make it a command line argument
    let symbol = "ethbtc";
    println!("Hello, world!");

    let mut service = StreamService::new(Some(symbol.to_owned()));
    service.run().await?;

    // TODO: Cache incoming data from stream and sort them on the go.
    //       - Use Max Heap for performance

    Ok(())
}
