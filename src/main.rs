use anyhow::Result;
use clap::Parser;
use crypto_streamer::{models::stream_service::StreamService, server::grpc_server};

// Command line argument processing config.
#[derive(Parser)]
#[clap(version = "1.0", author = "Igor Braga <higorb1@gmail.com>")]
struct Opts {
    #[clap(subcommand)]
    subcmd: SubCommand,
}

#[derive(Parser)]
enum SubCommand {
    /// gRPC Server. It also spawns threads to listen for Binance and Bitstamp
    #[clap(version = "1.0", author = "Igor Braga <higorb1@gmail.com>")]
    Server(ServerArgs),

    /// gRPC Client
    #[clap(version = "1.0", author = "Igor Braga <higorb1@gmail.com>")]
    Client(ClientArgs),
}

#[derive(Parser)]
pub(crate) struct ServerArgs {
    /// Symbol to which we'll stream
    #[clap(short = 's')]
    symbol: Option<String>,
}

#[derive(Parser)]
pub(crate) struct ClientArgs {}

#[tokio::main]
async fn main() -> Result<()> {
    stackdriver_logger::init_with(
        Some(stackdriver_logger::Service {
            name: "CryptoStream".to_owned(),
            version: "1.0".to_owned(),
        }),
        true,
    );

    let opts: Opts = Opts::parse();

    match opts.subcmd {
        SubCommand::Server(args) => {
            let mut service = StreamService::new(args.symbol);
            service.run().await?;

            // TODO: Above service.run().await?; should be called from withing the gRPC server!
            grpc_server::serve()
                .await
                .expect("Failed to run gRPC server");
        }
        SubCommand::Client(_args) => {
            // TODO: Call gRPC client. Mostly used to test
        }
    }

    // TODO: Cache incoming data from stream and sort them on the go.
    //       - Use Max Heap for performance

    Ok(())
}
