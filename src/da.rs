use anyhow::{anyhow, Error};
use async_trait::async_trait;
use protobuf::Message;
use rs_cnc::{CelestiaNodeClient, NamespacedSharesResponse, PayForDataResponse};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tracing::warn;

use crate::proto::msg::SequencerMsg;
use crate::types::{Base64String, Block};

static DEFAULT_PFD_FEE: i64 = 2_000;
static DEFAULT_PFD_GAS_LIMIT: u64 = 90_000;
static DEFAULT_NAMESPACE: &str = "0011223344556677"; // TODO; hash something to get this

/// SequencerBlock represents a sequencer layer block to be submitted to
/// the DA layer.
/// Currently, it consists of the Block.Data field of the cosmos-sdk block
/// returned by a sequencer, which contains the block's transactions.
/// TODO: compression or a better serialization method?
/// TODO: rename this b/c it's kind of confusing, types::Block is a cosmos-sdk block
/// which is also a sequencer block in a way.
#[derive(Serialize, Deserialize, Debug)]
pub struct SequencerBlock {
    block_hash: Base64String,
    sequencer_txs: Vec<Base64String>,
    rollup_txs: HashMap<Base64String, Vec<Base64String>>,
}

// impl SequencerBlock {
//     /// new returns a new empty SequencerBlock.
//     pub fn new() -> Self {
//         SequencerBlock { sequencer_txs: vec![], rollup_txs: HashMap::new() }
//     }
// }

impl From<Block> for SequencerBlock {
    fn from(b: Block) -> Self {
        // we unwrap generic txs into rollup-specific txs here,
        // and namespace them correspondingly

        let mut sequencer_txs = vec![]; // todo
        let mut rollup_txs = HashMap::new();

        for tx in b.data.txs.iter() {
            match SequencerMsg::parse_from_bytes(&tx.0) {
                Ok(msg) => {
                    let namespace = msg.chain_id;
                    if namespace.is_empty() {
                        sequencer_txs.push(Base64String(msg.data));
                        continue;
                    }

                    let txs = rollup_txs.entry(Base64String(namespace)).or_insert(vec![]);
                    txs.push(tx.clone());
                }
                Err(e) => warn!("failed to parse tx: {}", e),
            }
        }

        Self {
            block_hash: b.header.data_hash, // TODO: is this the right hash?
            sequencer_txs,
            rollup_txs,
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct SubmitDataResponse(pub PayForDataResponse);

#[derive(Deserialize, Debug)]
pub struct CheckBlockAvailabilityResponse(pub NamespacedSharesResponse);

/// DataAvailabilityClient is able to submit and query blocks from the DA layer.
#[async_trait]
pub trait DataAvailabilityClient {
    /// submit_block submits a block to the DA layer.
    /// it writes each transaction to a specific namespace given its chain ID.
    async fn submit_block(&self, block: SequencerBlock) -> Result<SubmitBlockResponse, Error>;
    async fn submit_namespaced_data(
        &self,
        namespace: &str,
        data: &[u8],
    ) -> Result<SubmitDataResponse, Error>;
    async fn check_block_availability(
        &self,
        height: u64,
    ) -> Result<CheckBlockAvailabilityResponse, Error>;
    async fn get_data(&self, height: u64, namespace: &str) -> Result<Vec<Base64String>, Error>;
    async fn get_blocks(&self, height: u64) -> Result<Vec<SequencerBlock>, Error>;
}

pub struct CelestiaClient(CelestiaNodeClient);

impl CelestiaClient {
    pub fn new(endpoint: String) -> Result<Self, Error> {
        let cnc = CelestiaNodeClient::new(endpoint)?;
        Ok(CelestiaClient(cnc))
    }
}

/// SequencerNamespaceData represents the data written to the "base"
/// sequencer namespace. It contains all the other namespaces that were
/// also written to in the same block.
#[derive(Serialize, Deserialize, Debug)]
struct SequencerNamespaceData {
    block_hash: Base64String,
    sequencer_txs: Vec<Base64String>,
    rollup_namespaces: Vec<Base64String>,
}

impl SequencerNamespaceData {
    fn to_bytes(&self) -> Result<Vec<u8>, Error> {
        // TODO: don't use json, use our own serializer (or protobuf for now?)
        let string = serde_json::to_string(self).map_err(|e| anyhow!(e))?;
        Ok(string.into_bytes())
    }

    fn from_bytes(bytes: &[u8]) -> Result<Self, Error> {
        let string = String::from_utf8(bytes.to_vec()).map_err(|e| anyhow!(e))?;
        let data = serde_json::from_str(&string).map_err(|e| anyhow!(e))?;
        Ok(data)
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct RollupNamespaceData {
    block_hash: Base64String,
    rollup_txs: Vec<Base64String>,
}

impl RollupNamespaceData {
    fn to_bytes(&self) -> Result<Vec<u8>, Error> {
        // TODO: don't use json, use our own serializer (or protobuf for now?)
        let string = serde_json::to_string(self).map_err(|e| anyhow!(e))?;
        Ok(string.into_bytes())
    }

    fn from_bytes(bytes: &[u8]) -> Result<Self, Error> {
        let string = String::from_utf8(bytes.to_vec()).map_err(|e| anyhow!(e))?;
        let data = serde_json::from_str(&string).map_err(|e| anyhow!(e))?;
        Ok(data)
    }
}

pub struct SubmitBlockResponse {
    pub namespace_to_block_num: HashMap<Base64String, Option<u64>>,
}

#[async_trait]
impl DataAvailabilityClient for CelestiaClient {
    async fn submit_block(&self, block: SequencerBlock) -> Result<SubmitBlockResponse, Error> {
        let mut namespace_to_block_num = HashMap::new();

        let rollup_namespaces = block.rollup_txs.keys();

        let sequencer_namespace_data = SequencerNamespaceData {
            block_hash: block.block_hash.clone(),
            sequencer_txs: block.sequencer_txs,
            rollup_namespaces: rollup_namespaces.cloned().collect(),
        };

        let bytes = sequencer_namespace_data.to_bytes()?;
        let resp = self
            .submit_namespaced_data(DEFAULT_NAMESPACE, &bytes)
            .await?;
        namespace_to_block_num.insert(
            Base64String(DEFAULT_NAMESPACE.to_string().into_bytes()),
            resp.0.height,
        );

        for (namespace, txs) in block.rollup_txs {
            let rollup_namespace_data = RollupNamespaceData {
                block_hash: block.block_hash.clone(),
                rollup_txs: txs,
            };
            let rollup_data_bytes = rollup_namespace_data.to_bytes()?;
            let resp = self
                .submit_namespaced_data(&namespace.to_string(), &rollup_data_bytes)
                .await?;
            namespace_to_block_num.insert(namespace, resp.0.height);
        }

        Ok(SubmitBlockResponse {
            namespace_to_block_num,
        })
    }

    async fn submit_namespaced_data(
        &self,
        namespace: &str,
        data: &[u8],
    ) -> Result<SubmitDataResponse, Error> {
        let pay_for_data_response = self
            .0
            .submit_pay_for_data(
                namespace,
                &data.to_vec().into(),
                DEFAULT_PFD_FEE,
                DEFAULT_PFD_GAS_LIMIT,
            )
            .await?;
        Ok(SubmitDataResponse(pay_for_data_response))
    }

    async fn check_block_availability(
        &self,
        height: u64,
    ) -> Result<CheckBlockAvailabilityResponse, Error> {
        let resp = self.0.namespaced_shares(DEFAULT_NAMESPACE, height).await?;
        Ok(CheckBlockAvailabilityResponse(resp))
    }

    async fn get_data(&self, height: u64, namespace: &str) -> Result<Vec<Base64String>, Error> {
        let namespaced_data_response = self.0.namespaced_data(namespace, height).await?;

        let data = namespaced_data_response
            .data
            .unwrap_or_default()
            .iter()
            .map(|d| Base64String(d.0.clone()))
            .collect();
        Ok(data)
    }

    async fn get_blocks(&self, height: u64) -> Result<Vec<SequencerBlock>, Error> {
        let namespaced_data_response = self.0.namespaced_data(DEFAULT_NAMESPACE, height).await?;

        // retrieve all sequencer blocks stored at this height
        let sequencer_namespace_datas: Vec<SequencerNamespaceData> = namespaced_data_response
            .data
            .unwrap_or_default()
            .iter()
            .filter_map(|d| {
                if let Ok(data) = SequencerNamespaceData::from_bytes(&d.0) {
                    Some(data)
                } else {
                    None
                }
            })
            .collect();

        let mut blocks = vec![];

        // for all the sequencer blocks retrieved, create the corresponding SequencerBlock
        for sequencer_namespace_data in &sequencer_namespace_datas {
            let rollup_namespaces = sequencer_namespace_data.rollup_namespaces.clone();
            let mut rollup_txs_map = HashMap::new();

            // for each rollup namespace, retrieve the corresponding rollup block
            for rollup_namespace in rollup_namespaces {
                let namespaced_data_response = self
                    .0
                    .namespaced_data(&rollup_namespace.to_string(), height)
                    .await?;

                let rollup_txs: Vec<RollupNamespaceData> = namespaced_data_response
                    .data
                    .unwrap_or_default()
                    .iter()
                    .filter_map(|d| {
                        if let Ok(data) = RollupNamespaceData::from_bytes(&d.0) {
                            Some(data)
                        } else {
                            None
                        }
                    })
                    .collect();

                for rollup_tx in rollup_txs {
                    if rollup_tx.block_hash == sequencer_namespace_data.block_hash {
                        rollup_txs_map.insert(rollup_namespace.clone(), rollup_tx.rollup_txs);
                    }
                }
            }

            blocks.push(SequencerBlock {
                block_hash: sequencer_namespace_data.block_hash.clone(),
                sequencer_txs: sequencer_namespace_data.sequencer_txs.clone(),
                rollup_txs: rollup_txs_map,
            });
        }

        Ok(blocks)
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::{CelestiaClient, DataAvailabilityClient, SequencerBlock, DEFAULT_NAMESPACE};
    use crate::types::Base64String;

    #[tokio::test]
    async fn test_celestia_client() {
        // unfortunately, this needs to be all one test for now, since
        // submitting multiple blocks to celestia concurrently returns
        // "incorrect account sequence" errors.

        // test submit_block
        let base_url = "http://localhost:26659".to_string();
        let client = CelestiaClient::new(base_url).unwrap();
        let tx = Base64String(b"noot_was_here".to_vec());

        let block_hash = Base64String(vec![99; 32]);
        let block = SequencerBlock {
            block_hash: block_hash.clone(),
            sequencer_txs: vec![tx.clone()],
            rollup_txs: HashMap::new(),
        };

        let submit_block_resp = client.submit_block(block).await.unwrap();
        let height = submit_block_resp
            .namespace_to_block_num
            .get(&Base64String(DEFAULT_NAMESPACE.as_bytes().to_vec()))
            .unwrap()
            .unwrap();

        // test check_block_availability
        let resp = client.check_block_availability(height).await.unwrap();
        assert_eq!(resp.0.height, height);

        // test get_blocks
        let resp = client.get_blocks(height).await.unwrap();
        assert_eq!(resp.len(), 1);
        assert_eq!(resp[0].block_hash, block_hash);
        assert_eq!(resp[0].sequencer_txs.len(), 1);
        assert_eq!(resp[0].sequencer_txs[0], tx);
        assert_eq!(resp[0].rollup_txs.len(), 0);
    }
}
