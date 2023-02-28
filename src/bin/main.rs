use structopt::StructOpt;
use tracing_subscriber::EnvFilter;

use sequencer_relayer::{da::CelestiaClient, sequencer::SequencerClient};

pub const DEFAULT_SEQUENCER_ENDPOINT: &str = "http://localhost:1317";
pub const DEFAULT_CELESTIA_ENDPOINT: &str = "http://localhost:26659";

#[derive(StructOpt)]
struct Options {
    /// Sequencer node RPC endpoint. Default: http://localhost:1317
    #[structopt(short, long, default_value = DEFAULT_SEQUENCER_ENDPOINT)]
    sequencer_endpoint: String,

    /// Celestia node RPC endpoint. Default: http://localhost:26659
    #[structopt(short, long, default_value = DEFAULT_CELESTIA_ENDPOINT)]
    celestia_endpoint: String,

    /// Log level. One of debug, info, warn, or error
    #[structopt(short, long, default_value = "info")]
    log: String,
}

fn main() {
    let options: Options = Options::from_args();

    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(options.log)),
        )
        .init();

    let sequencer_client = SequencerClient::new(options.sequencer_endpoint)
        .expect("failed to create sequencer client");
    let da_client = CelestiaClient::new(options.celestia_endpoint)
        .expect("failed to create data availability client");
}
