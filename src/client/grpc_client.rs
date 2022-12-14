use anyhow::Result;
use orderbook::orderbook_aggregator_client::OrderbookAggregatorClient;
use orderbook::Empty;

use crate::models::consts::{IP_ADDRESS, SERVER_PORT};

pub mod orderbook {
    tonic::include_proto!("orderbook");
}

pub async fn listen() -> Result<()> {
    println!("Hello I'm a gRPC CLient TO BE implemented!");

    let server_url = format!("http://{}:{}", IP_ADDRESS, SERVER_PORT);
    let mut client = OrderbookAggregatorClient::connect(server_url).await?;

    let empty = Empty {};

    let mut stream = client.book_summary(empty).await?.into_inner();

    while let Some(summary) = stream.message().await? {
        // Uncomment me to beautify output. Note: it does add some latency to the client, which is why it's commented by default
        // let summary = SummaryOutput::from(summary);
        log::info!("\n{:#?}", summary);
    }

    Ok(())
}
