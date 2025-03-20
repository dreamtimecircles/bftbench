mod conf;

use std::env::args;
use std::error::Error;
use std::io::{ErrorKind, Write};
use tonic::transport::Server;

use anyhow::Result;

use clap::Parser;
use conf::Config as ServerConfig;

use crate::pb::Tx;
use std::net::ToSocketAddrs;
use std::pin::Pin;
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;
use tokio_stream::{Stream, StreamExt};
use tonic::{Request, Response, Status, Streaming};

pub mod pb {
    tonic::include_proto!("grpc.echo");
}

type ResponseStream = Pin<Box<dyn Stream<Item = Result<Tx, Status>> + Send>>;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(short, long, value_name = "FILE.toml")]
    config: std::path::PathBuf,
}

#[derive(Debug)]
pub struct EchoServer {}

#[tonic::async_trait]
impl pb::service_server::Service for EchoServer {
    type EchoStream = ResponseStream;

    async fn echo(
        &self,
        req: Request<Streaming<Tx>>,
    ) -> std::result::Result<Response<Self::EchoStream>, Status> {
        let mut in_stream = req.into_inner();
        let (sender, receiver) = mpsc::channel(128);

        // this spawn here is required if you want to handle connection error.
        // If we just map `in_stream` and write it back as `out_stream` the `out_stream`
        // will be drooped when connection error occurs and error will never be propagated
        // to mapped version of `in_stream`.
        tokio::spawn(async move {
            while let Some(result) = in_stream.next().await {
                match result {
                    Ok(tx) => sender
                        .send(Ok(Tx {
                            tx_id: tx.tx_id,
                            value: tx.value,
                        }))
                        .await
                        .expect("working rx"),
                    Err(err) => {
                        if let Some(io_err) = match_for_io_error(&err) {
                            if io_err.kind() == ErrorKind::BrokenPipe {
                                // here you can handle special case when client
                                // disconnected in unexpected way
                                log::info!("client disconnected: broken pipe");
                                break;
                            }
                        }

                        match sender.send(Err(err)).await {
                            Ok(_) => (),
                            Err(_err) => break, // response was dropped
                        }
                    }
                }
            }
            log::info!("stream ended");
        });

        // echo just write the same data that was received
        let out_stream = ReceiverStream::new(receiver);

        Ok(Response::new(Box::pin(out_stream) as Self::EchoStream))
    }
}

fn match_for_io_error(err_status: &Status) -> Option<&std::io::Error> {
    let mut err: &(dyn Error + 'static) = err_status;

    loop {
        if let Some(io_err) = err.downcast_ref::<std::io::Error>() {
            return Some(io_err);
        }

        // h2::Error do not expose std::io::Error with `source()`
        // https://github.com/hyperium/h2/pull/462
        if let Some(h2_err) = err.downcast_ref::<h2::Error>() {
            if let Some(io_err) = h2_err.get_io() {
                return Some(io_err);
            }
        }

        err = match err.source() {
            Some(err) => err,
            None => return None,
        };
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info"))
        .target(env_logger::Target::Stdout)
        .format(|buf, record| {
            let ts = buf.timestamp_micros();
            writeln!(
                buf,
                "[{} {}{}{:#} {:?} {} {}:{}] {}",
                ts,
                buf.default_level_style(record.level()),
                record.level(),
                buf.default_level_style(record.level()),
                std::thread::current().id(),
                record.target(),
                record.file().unwrap_or("<unknown>"),
                record.line().unwrap_or(0),
                record.args()
            )
        })
        .init();

    let cli = Cli::parse();

    let settings = config::Config::builder()
        .add_source(config::File::from(cli.config))
        .add_source(config::Environment::with_prefix("BENCH"))
        .build()?;

    let config = settings.try_deserialize::<ServerConfig>()?;

    let program_name = args().next().unwrap();
    log::info!(
        "'{}' starting, configuration loaded: {:?}",
        program_name,
        config
    );

    let server = EchoServer {};
    let awaiter = Server::builder()
        .add_service(pb::service_server::ServiceServer::new(server))
        .serve(config.listen.to_socket_addrs().unwrap().next().unwrap());

    log::info!("'{}' open for business", program_name);

    awaiter.await.unwrap();

    Ok(())
}
