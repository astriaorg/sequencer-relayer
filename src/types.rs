use anyhow::Error;
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;
use tendermint::{
    account::Id as AccountId,
    block::{
        header::Version as TmVersion, parts::Header as TmPartSetHeader, Header as TmHeader,
        Height as TmHeight, Id as TmBlockId,
    },
    chain::Id as TmChainId,
    hash::{AppHash, Hash as TmHash},
    Time,
};

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

#[derive(Deserialize, Debug)]
pub struct BlockId {
    pub hash: Base64String,
    pub part_set_header: Parts,
}

#[derive(Deserialize, Debug)]
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

#[derive(Deserialize, Debug)]
pub struct Header {
    pub version: Version,
    pub chain_id: String,
    pub height: String,
    pub time: String,
    pub last_block_id: Option<BlockId>,
    pub last_commit_hash: Option<Base64String>,
    pub data_hash: Option<Base64String>,
    pub validators_hash: Base64String,
    pub next_validators_hash: Base64String,
    pub consensus_hash: Base64String,
    pub app_hash: Base64String,
    pub last_results_hash: Option<Base64String>,
    pub evidence_hash: Option<Base64String>,
    pub proposer_address: Base64String,
}

#[derive(Deserialize, Debug)]
pub struct Version {
    pub block: String,
    pub app: String,
}

#[derive(Deserialize, Debug)]
pub struct Data {
    pub txs: Vec<Base64String>,
}

/// header_to_tendermint_header converts a Tendermint RPC header to a tendermint-rs header.
#[allow(dead_code)]
fn header_to_tendermint_header(header: &Header) -> Result<TmHeader, Error> {
    let last_block_id = header
        .last_block_id
        .as_ref()
        .map(|id| {
            Ok(TmBlockId {
                hash: TmHash::try_from(id.hash.0.clone())?,
                part_set_header: TmPartSetHeader::new(
                    id.part_set_header.total,
                    TmHash::try_from(id.part_set_header.hash.0.clone())?,
                )?,
            })
        })
        .map_or(Ok(None), |r: Result<TmBlockId, Error>| r.map(Some))?;

    let last_commit_hash = header
        .last_commit_hash
        .as_ref()
        .map(|h| TmHash::try_from(h.0.clone()))
        .map_or(Ok(None), |r| r.map(Some))?;

    let data_hash = header
        .data_hash
        .as_ref()
        .map(|h| TmHash::try_from(h.0.clone()))
        .map_or(Ok(None), |r| r.map(Some))?;

    let last_results_hash = header
        .last_results_hash
        .as_ref()
        .map(|h| TmHash::try_from(h.0.clone()))
        .map_or(Ok(None), |r| r.map(Some))?;

    let evidence_hash = header
        .evidence_hash
        .as_ref()
        .map(|h| TmHash::try_from(h.0.clone()))
        .map_or(Ok(None), |r| r.map(Some))?;

    Ok(TmHeader {
        version: TmVersion {
            block: header.version.block.parse::<u64>()?,
            app: header.version.app.parse::<u64>()?,
        },
        chain_id: TmChainId::try_from(header.chain_id.clone())?,
        height: TmHeight::try_from(header.height.parse::<u64>()?)?,
        time: Time::parse_from_rfc3339(&header.time)?,
        last_block_id,
        last_commit_hash,
        data_hash,
        validators_hash: TmHash::try_from(header.validators_hash.0.clone())?,
        next_validators_hash: TmHash::try_from(header.next_validators_hash.0.clone())?,
        consensus_hash: TmHash::try_from(header.consensus_hash.0.clone())?,
        app_hash: AppHash::try_from(header.app_hash.0.clone())?,
        last_results_hash,
        evidence_hash,
        proposer_address: AccountId::try_from(header.proposer_address.0.clone())?,
    })
}

#[cfg(test)]
mod test {
    use super::header_to_tendermint_header;
    use crate::sequencer::SequencerClient;

    #[tokio::test]
    async fn test_header_to_tendermint_header() {
        let cosmos_endpoint = "http://localhost:1317".to_string();
        let client = SequencerClient::new(cosmos_endpoint).unwrap();
        let resp = client.get_latest_block().await.unwrap();
        let tm_header = header_to_tendermint_header(&resp.block.header).unwrap();
        let tm_header_hash = tm_header.hash();
        assert_eq!(tm_header_hash.as_bytes(), &resp.block_id.hash.0);
    }
}
