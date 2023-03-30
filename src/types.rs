use serde::{Deserialize, Serialize};

use crate::base64_string::Base64String;

/// cosmos-sdk (Tendermint) RPC types.
/// see https://v1.cosmos.network/rpc/v0.41.4

#[derive(Serialize, Debug)]
pub struct EmptyRequest {}

#[derive(Deserialize, Debug)]
pub struct BlockResponse {
    pub block_id: BlockId,
    pub block: Block,
}

#[derive(Clone, Deserialize, Debug, PartialEq, Eq, Serialize)]
pub struct BlockId {
    pub hash: Base64String,
    pub part_set_header: Parts,
}

#[derive(Clone, Deserialize, Debug, PartialEq, Eq, Serialize)]
pub struct Parts {
    pub total: u32,
    pub hash: Base64String,
}

#[derive(Deserialize, Debug)]
pub struct Block {
    pub header: Header,
    pub data: Data,
    // TODO: evidence
    pub last_commit: Commit,
}

#[derive(Deserialize, Debug)]
pub struct Commit {
    pub height: String,
    pub round: u64,
    pub block_id: BlockId,
    pub signatures: Vec<CommitSig>,
}

#[derive(Deserialize, Debug)]
pub struct CommitSig {
    pub block_id_flag: String,
    pub validator_address: Base64String,
    pub timestamp: String,
    pub signature: Base64String,
}

#[derive(Clone, Deserialize, Debug, Eq, PartialEq, Serialize)]
pub struct Version {
    pub block: String,
    pub app: String,
}

#[derive(Deserialize, Debug)]
pub struct Data {
    pub txs: Vec<Base64String>,
}

#[derive(Clone, Deserialize, Debug, Eq, PartialEq, Serialize)]
pub struct Header(pub tendermint::block::Header);

impl Header {
    pub fn hash(&self) -> tendermint::hash::Hash {
        self.0.hash()
    }
}

#[cfg(test)]
mod test {
    use crate::sequencer::SequencerClient;

    #[tokio::test]
    async fn test_header_to_tendermint_header() {
        let cosmos_endpoint = "http://localhost:1317".to_string();
        let client = SequencerClient::new(cosmos_endpoint).unwrap();
        let resp = client.get_latest_block().await.unwrap();
        let tm_header_hash = resp.block.header.hash();
        assert_eq!(tm_header_hash.as_bytes(), &resp.block_id.hash.0);
    }
}
