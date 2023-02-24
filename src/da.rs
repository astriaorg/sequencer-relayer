use anyhow::{anyhow, Error};
use async_trait::async_trait;
use rs_cnc::{CelestiaNodeClient, PayForDataResponse};
use serde::{Deserialize, Serialize};

use crate::types::Block;

/// SequencerBlock represents a sequencer layer block to be submitted to
/// the DA layer.
/// Currently, it consists of the Block.Data field of the cosmos-sdk block
/// returned by a sequencer, which contains the block's transactions.
/// TODO: include other fields (such as block hash)?
/// TODO: compression or a better serialization method?
/// TODO: rename this b/c it's kind of confusing, types::Block is a cosmos-sdk block
/// which is also a sequencer block in a way.
#[derive(Serialize, Deserialize, Debug)]
pub struct SequencerBlock {
    txs: Vec<String>,
}

impl SequencerBlock {
    pub fn new() -> Self {
        SequencerBlock { txs: vec![] }
    }
}

impl From<Block> for SequencerBlock {
    fn from(b: Block) -> Self {
        // TODO: we need to unwrap sequencer txs into rollup-specific txs here,
        // and namespace them correspondingly
        Self { txs: b.data.txs }
    }
}

#[derive(Deserialize, Debug)]
pub struct SubmitBlockResponse(PayForDataResponse);

/// DataAvailabilityClient is able to submit and query blocks from the DA layer.
#[async_trait]
pub trait DataAvailabilityClient {
    async fn submit_block(&self, block: SequencerBlock) -> Result<SubmitBlockResponse, Error>;
}

pub struct CelestiaClient(CelestiaNodeClient);

impl CelestiaClient {
    pub fn new(endpoint: String) -> Result<Self, Error> {
        let cnc = CelestiaNodeClient::new(endpoint)?;
        Ok(CelestiaClient(cnc))
    }
}

#[async_trait]
impl DataAvailabilityClient for CelestiaClient {
    async fn submit_block(&self, block: SequencerBlock) -> Result<SubmitBlockResponse, Error> {
        // TODO: don't use json, use our own serializer
        let block_bytes = serde_json::to_string(&block).map_err(|e| anyhow!(e))?;
        let namespace = "sequencer-relayer";
        let fee = 1; // TODO
        let gas_limit = 1000000; // TODO
        let pay_for_data_response = self
            .0
            .submit_pay_for_data(namespace, &block_bytes.into(), fee, gas_limit)
            .await?;
        Ok(SubmitBlockResponse(pay_for_data_response))
    }
}

#[cfg(test)]
mod tests {
    use super::{CelestiaClient, DataAvailabilityClient, SequencerBlock};

    #[tokio::test]
    async fn test_submit_block() {
        let base_url = "http://localhost:26659".to_string();
        let client = CelestiaClient::new(base_url).unwrap();
        let block = SequencerBlock::new();
        let resp = client.submit_block(block).await.unwrap();
        println!("{:?}", resp);
    }
}
