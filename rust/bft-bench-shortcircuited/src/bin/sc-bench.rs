use std::env::args;

use anyhow::Result;

use bft_bench_core::{BftBinding, Config};
use bft_bench_shortcircuited::ShortCircuitedBftBinding;

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();

    tracing::subscriber::set_global_default(
        tracing_subscriber::fmt::Subscriber::builder()
            // subscriber configuration
            .with_max_level(tracing::Level::DEBUG)
            .finish(),
    )
    .expect("Unable to set global tracing subscriber");

    let metrics_handle = opinionated_metrics::initialize_cli()?;

    let settings = config::Config::builder()
        .add_source(config::File::with_name("Benchmark.toml"))
        .add_source(config::Environment::with_prefix("BENCH"))
        .build()
        .unwrap();

    let config = settings.try_deserialize::<Config>()?;

    log::info!(
        "'{}' starting, configuration loaded: {:?}",
        args().next().unwrap(),
        config
    );

    let sc_binding = ShortCircuitedBftBinding::new();

    log::info!("Starting benchmark");

    bft_bench_core::run(config, sc_binding).await?;

    log::info!("Benchmark completed, stats follow");

    metrics_handle.report().await?;

    Ok(())
}
