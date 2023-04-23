use clap::Parser;
use dirs::home_dir;
use tracing::info;
use tracing_subscriber::EnvFilter;

use std::time;

use sequencer_relayer::{
    da::CelestiaClient,
    relayer::{Relayer, ValidatorPrivateKeyFile},
    sequencer::SequencerClient,
    server,
};

pub const DEFAULT_SEQUENCER_ENDPOINT: &str = "http://localhost:1317";
pub const DEFAULT_CELESTIA_ENDPOINT: &str = "http://localhost:26659";

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Sequencer node RPC endpoint. Default: http://localhost:1317
    #[arg(short, long, default_value = DEFAULT_SEQUENCER_ENDPOINT)]
    sequencer_endpoint: String,

    /// Celestia node RPC endpoint. Default: http://localhost:26659
    #[arg(short, long, default_value = DEFAULT_CELESTIA_ENDPOINT)]
    celestia_endpoint: String,

    /// Expected block time of the sequencer in milliseconds;
    /// ie. how often we should poll the sequencer.
    #[arg(short, long, default_value = "1000")]
    block_time: u64,

    /// Path to validator private key file.
    #[arg(short, long, default_value = ".metro/config/priv_validator_key.json")]
    validator_key_file: String,

    /// RPC port to listen on. Default: 2450
    #[arg(short, long, default_value = "2450")]
    rpc_port: u16,

    /// Log level. One of debug, info, warn, or error
    #[arg(short, long, default_value = "info")]
    log: String,
}

#[allow(clippy::await_holding_lock)]
#[tokio::main]
async fn main() {
    let args = Args::parse();

    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(args.log)),
        )
        .init();

    // unmarshal validator private key file
    let home_dir = home_dir().unwrap();
    let file_path = home_dir.join(&args.validator_key_file);
    info!("using validator keys located at {}", file_path.display());

    let key_file =
        std::fs::read_to_string(file_path).expect("failed to read validator private key file");
    let key_file: ValidatorPrivateKeyFile =
        serde_json::from_str(&key_file).expect("failed to unmarshal validator key file");

    let sequencer_client =
        SequencerClient::new(args.sequencer_endpoint).expect("failed to create sequencer client");
    let da_client = CelestiaClient::new(args.celestia_endpoint)
        .expect("failed to create data availability client");

    let sleep_duration = time::Duration::from_millis(args.block_time);
    let mut interval = tokio::time::interval(sleep_duration);

    let mut relayer = Relayer::new(sequencer_client, da_client, key_file);
    let state = relayer.get_state();

    tokio::task::spawn(async move {
        info!("starting RPC server on port {}", args.rpc_port);
        server::start("127.0.0.1", args.rpc_port, state)
            .await
            .expect("failed to start RPC server");
        info!("started RPC server on port {}", args.rpc_port);
    });

    loop {
        interval.tick().await;
        relayer.run().await;
    }
}
