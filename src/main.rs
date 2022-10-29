use anyhow::Result;
use clap::Parser;
use crypto_streamer::{client::grpc_client, server::grpc_server};

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

    /// gRPC Client. Listens for gRPC streaming Server sending sorted orderbooks
    #[clap(version = "1.0", author = "Igor Braga <higorb1@gmail.com>")]
    Client(ClientArgs),
}

#[derive(Parser)]
pub(crate) struct ServerArgs {
    /// Symbol (currency pair) to which we'll stream
    #[clap(short = 's')]
    symbol: Option<String>,
}

#[derive(Parser)]
pub(crate) struct ClientArgs {}

#[tokio::main]
async fn main() -> Result<()> {
    pretty_env_logger::init_timed();

    let opts: Opts = Opts::parse();

    match opts.subcmd {
        SubCommand::Server(args) => {
            grpc_server::serve(args.symbol)
                .await
                .expect("Failed to run gRPC server");
        }
        SubCommand::Client(_args) => {
            grpc_client::listen().await?;
        }
    }

    Ok(())
}
