use orderbook::orderbook_aggregator_server::{OrderbookAggregator, OrderbookAggregatorServer};
use orderbook::{Empty, Summary};
use tokio::sync::mpsc::channel;
use tokio_stream::wrappers::ReceiverStream;
use tonic::{transport::Server, Request, Response, Status};

pub mod orderbook {
    tonic::include_proto!("orderbook");
}

#[derive(Debug, Default)]
pub struct CryptService {}

#[tonic::async_trait]
impl OrderbookAggregator for CryptService {
    type BookSummaryStream = ReceiverStream<Result<Summary, Status>>;

    async fn book_summary(
        &self,
        empty: Request<Empty>,
    ) -> Result<Response<Self::BookSummaryStream>, Status> {
        let (tx, rx) = channel(4);

        tokio::spawn(async move {
            for i in 0..10 {
                let summary = Summary {
                    spread: i as f64,
                    bids: vec![],
                    asks: vec![],
                };
                tx.send(Ok(summary)).await.unwrap();
            }

            println!(" /// done sending");
        });

        // let summary = Summary {
        //     spread: 10.0,
        //     bids: vec![],
        //     asks: vec![]
        // };

        Ok(Response::new(ReceiverStream::new(rx)))
    }
}

pub async fn serve() -> Result<(), Box<dyn std::error::Error>> {
    // defining address for our service
    let addr = "[::1]:8080".parse().unwrap();
    // creating a service
    let crypt = CryptService::default();
    println!("Server listening on {}", addr);
    // adding our service to our server.
    Server::builder()
        .add_service(OrderbookAggregatorServer::new(crypt))
        .serve(addr)
        .await?;
    Ok(())
}
