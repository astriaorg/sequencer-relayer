use bech32::{self, ToBase32, Variant};
use serde::Deserialize;
use std::str::FromStr;
use std::sync::Arc;
use tokio::sync::Mutex;
use tracing::{info, warn};

use crate::base64_string::Base64String;
use crate::da::CelestiaClient;
use crate::keys::{private_key_bytes_to_keypair, validator_hex_to_address};
use crate::sequencer::SequencerClient;
use crate::sequencer_block::SequencerBlock;

#[derive(Deserialize)]
pub struct ValidatorPrivateKeyFile {
    pub address: String,
    pub pub_key: KeyWithType,
    pub priv_key: KeyWithType,
}

#[derive(Deserialize)]
pub struct KeyWithType {
    #[serde(rename = "type")]
    pub key_type: String,
    pub value: String,
}

pub struct Relayer {
    sequencer_client: SequencerClient,
    da_client: CelestiaClient,
    keypair: ed25519_dalek::Keypair,
    validator_address: String,
    validator_address_bytes: Vec<u8>,

    state: Arc<Mutex<State>>,
}

#[derive(Clone, Debug)]
pub struct State {
    pub curr_sequencer_height: u64,
    pub curr_da_height: u64,
}

impl Relayer {
    pub fn new(
        sequencer_client: SequencerClient,
        da_client: CelestiaClient,
        key_file: ValidatorPrivateKeyFile,
    ) -> Self {
        // generate our private-public keypair
        let keypair = private_key_bytes_to_keypair(
            &Base64String::from_string(key_file.priv_key.value)
                .expect("failed to decode validator private key; must be base64 string")
                .0,
        )
        .expect("failed to convert validator private key to keypair");

        // generate our bech32 validator address
        let validator_address = validator_hex_to_address(&key_file.address)
            .expect("failed to convert validator address to bech32");

        // generate our validator address bytes
        let validator_address_bytes = hex::decode(&key_file.address)
            .expect("failed to decode validator address; must be hex string");

        Self {
            sequencer_client,
            da_client,
            keypair,
            validator_address,
            validator_address_bytes,
            state: Arc::new(Mutex::new(State {
                curr_sequencer_height: 0,
                curr_da_height: 0,
            })),
        }
    }

    pub fn get_state(&self) -> Arc<Mutex<State>> {
        self.state.clone()
    }

    pub async fn run(&mut self) {
        let mut state = self.state.lock().await;

        match self.sequencer_client.get_latest_block().await {
            Ok(resp) => {
                let maybe_height: Result<u64, <u64 as FromStr>::Err> =
                    resp.block.header.height.parse();
                if let Err(e) = maybe_height {
                    warn!(
                        error = ?e,
                        "got invalid block height {} from sequencer",
                        resp.block.header.height,
                    );
                    return;
                }

                let height = maybe_height.unwrap();
                if height <= state.curr_sequencer_height {
                    return;
                }

                info!("got block with height {} from sequencer", height);
                state.curr_sequencer_height = height;

                if resp.block.header.proposer_address.0 != self.validator_address_bytes {
                    let proposer_address = bech32::encode(
                        "metrovalcons",
                        resp.block.header.proposer_address.0.to_base32(),
                        Variant::Bech32,
                    )
                    .expect("should encode block proposer address");
                    info!(
                        %proposer_address,
                        validator_address = %self.validator_address,
                        "ignoring block: proposer address is not ours",
                    );
                    return;
                }

                let sequencer_block = match SequencerBlock::from_cosmos_block(resp.block) {
                    Ok(block) => block,
                    Err(e) => {
                        warn!(error = ?e, "failed to convert block to DA block");
                        return;
                    }
                };

                let tx_count =
                    sequencer_block.rollup_txs.len() + sequencer_block.sequencer_txs.len();
                match self
                    .da_client
                    .submit_block(sequencer_block, &self.keypair)
                    .await
                {
                    Ok(resp) => {
                        state.curr_da_height = resp.height;
                        info!(
                            "submitted sequencer block {} to DA layer (included in block {}): tx count={}",
                            height, resp.height, &tx_count,
                        )
                    }
                    Err(e) => warn!(error = ?e, "failed to submit block to DA layer"),
                }
            }
            Err(e) => warn!(error = ?e, "failed to get latest block from sequencer"),
        }
    }
}
