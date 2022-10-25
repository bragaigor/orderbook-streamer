use anyhow::Result;
use orderbook::orderbook_aggregator_client::OrderbookAggregatorClient;
use orderbook::Empty;

pub mod orderbook {
    tonic::include_proto!("orderbook");
}

pub async fn listen() -> Result<()> {
    println!("Hello I'm a gRPC CLient TO BE implemented!");

    let mut client = OrderbookAggregatorClient::connect("http://[::1]:8080").await?;

    let empty = Empty {};

    let mut stream = client.book_summary(empty).await?.into_inner();

    while let Some(feature) = stream.message().await? {
        println!("CLIENT: message from server = {:?}", feature);
    }

    Ok(())
}
