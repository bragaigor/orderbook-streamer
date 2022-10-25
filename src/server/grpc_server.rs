use std::sync::Arc;

use orderbook::orderbook_aggregator_server::{OrderbookAggregator, OrderbookAggregatorServer};
use orderbook::{Empty, Summary};
use tokio::sync::mpsc::{channel, Receiver};
use tokio::sync::Mutex;
use tokio_stream::wrappers::ReceiverStream;
use tonic::{transport::Server, Request, Response, Status};

use crate::models::messages::OrderbookMessage;
use crate::models::stream_service::StreamService;

pub mod orderbook {
    tonic::include_proto!("orderbook");
}

#[derive(Debug)]
pub struct OrderbookService {
    pub chan_recv: Arc<Mutex<Receiver<OrderbookMessage>>>,
}

pub type ResultSummary = Result<Summary, Status>;

#[tonic::async_trait]
impl OrderbookAggregator for OrderbookService {
    type BookSummaryStream = ReceiverStream<ResultSummary>;

    async fn book_summary(
        &self,
        _empty: Request<Empty>,
    ) -> Result<Response<Self::BookSummaryStream>, Status> {
        // TODO: Make the channel buffer limit here dynamic and larger
        let (tx, rx) = channel(4);

        // TODO: The problem with this implementation is that we can only have one client listening
        //       to the gRPC server at a time, and that's because we're using mpsc channel, which
        //       means we only have one receiver for many servers. To fix this we would have to update
        //       the Server to use broadcast instead.
        let chan_recv_cloned = self.chan_recv.clone();
        tokio::spawn(async move { StreamService::mpsc_handle(chan_recv_cloned, tx.clone()).await });

        println!("DONE with book_summary!!!");

        Ok(Response::new(ReceiverStream::new(rx)))
    }
}

pub async fn serve(symbol: Option<String>) -> Result<(), Box<dyn std::error::Error>> {
    let service = StreamService::new(symbol);
    let chan_recv = service.run().await?;

    // defining address for our service
    let addr = "[::1]:8080".parse().unwrap();
    // creating a service
    let crypt = OrderbookService {
        chan_recv: Arc::new(Mutex::new(chan_recv)),
    };

    println!("Server listening on {}", addr);
    // adding our service to our server.
    Server::builder()
        .add_service(OrderbookAggregatorServer::new(crypt))
        .serve(addr)
        .await?;
    Ok(())
}
