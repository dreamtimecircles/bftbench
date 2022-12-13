use std::env::args;
use std::io::Write;

use anyhow::Result;

use clap::Parser;

use bft_bench_core::{BftBinding, Config};
use bft_bench_shortcircuit::ShortCircuitedBftBinding;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(short, long, value_name = "FILE.toml")]
    config: std::path::PathBuf,
}

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info"))
        .target(env_logger::Target::Stdout)
        .format(|buf, record| {
            let ts = buf.timestamp_micros();
            writeln!(
                buf,
                "[{} {} {:?} {} {}:{}] {}",
                ts,
                buf.default_level_style(record.level())
                    .value(record.level()),
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

    let config = settings.try_deserialize::<Config>()?;

    log::info!(
        "'{}' starting, configuration loaded: {:?}",
        args().next().unwrap(),
        config
    );

    let sc_binding = ShortCircuitedBftBinding::new();

    log::info!("Starting benchmark");

    log::info!(
        "Benchmark completed, stats follow: {}",
        bft_bench_core::run(config, sc_binding).await?
    );

    Ok(())
}
