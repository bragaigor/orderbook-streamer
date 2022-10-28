use std::thread;

use orderbook::orderbook_aggregator_server::{OrderbookAggregator, OrderbookAggregatorServer};
use orderbook::{Empty, Summary};
use tokio::sync::broadcast::Sender;
use tokio::sync::mpsc::channel;
use tokio_stream::wrappers::ReceiverStream;
use tonic::{transport::Server, Request, Response, Status};

use crate::models::consts::{IP_ADDRESS, SERVER_PORT};
use crate::models::messages::OrderbookMessage;
use crate::models::stream_service::StreamService;

pub mod orderbook {
    tonic::include_proto!("orderbook");
}

#[derive(Debug)]
pub struct OrderbookService {
    pub chan_send: Sender<OrderbookMessage>,
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
        let (tx, rx) = channel(100);

        let machine_name = &uname::uname()?.nodename;
        let id = thread::current().id();
        let client_id = format!("{}:tid:{:?}", machine_name, id);
        log::info!("Starting client with id: {}", &client_id);

        // The nice thing about this implementation is that we can have n numbers of clients listening to
        // the same server since we're using multi-producer, multi-consumer broadcast queue
        let chan_recv = self.chan_send.subscribe();
        tokio::spawn(async move {
            StreamService::broadcast_handle(client_id, chan_recv, tx.clone()).await
        });

        Ok(Response::new(ReceiverStream::new(rx)))
    }
}

pub async fn serve(symbol: Option<String>) -> Result<(), Box<dyn std::error::Error>> {
    let service = StreamService::new(symbol);
    let chan_send = service.run().await?;

    // Defining address for our service.
    let addr = format!("{}:{}", IP_ADDRESS, SERVER_PORT).parse().unwrap();
    // Create an orderbook service instance.
    let orderbook = OrderbookService { chan_send };

    log::info!("Server listening on {}", addr);
    // Add orderbook service to the server.
    Server::builder()
        .add_service(OrderbookAggregatorServer::new(orderbook))
        .serve(addr)
        .await?;
    Ok(())
}
