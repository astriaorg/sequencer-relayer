use anyhow::Error;
use base64::{engine::general_purpose, Engine as _};
use serde::{
    de::{self, Visitor},
    Deserialize, Deserializer, Serialize,
};
use std::convert::TryFrom;
use std::fmt;
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

/// cosmos-sdk RPC types.
/// see https://v1.cosmos.network/rpc/v0.41.4

#[derive(Serialize, Debug)]
pub struct EmptyRequest {}

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Base64String(pub Vec<u8>);

impl Base64String {
    pub fn from_string(s: String) -> Result<Base64String, base64::DecodeError> {
        general_purpose::STANDARD.decode(s).map(Base64String)
    }
}

impl std::fmt::Display for Base64String {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", general_purpose::STANDARD.encode(&self.0))
    }
}

impl fmt::Debug for Base64String {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&general_purpose::STANDARD.encode(&self.0))
    }
}

impl Serialize for Base64String {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&general_purpose::STANDARD.encode(&self.0))
    }
}

impl<'de> Deserialize<'de> for Base64String {
    fn deserialize<D>(deserializer: D) -> Result<Base64String, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_string(Base64StringVisitor)
    }
}

struct Base64StringVisitor;

impl Base64StringVisitor {
    fn decode_string<E>(self, value: &str) -> Result<Base64String, E>
    where
        E: de::Error,
    {
        general_purpose::STANDARD
            .decode(value)
            .map(Base64String)
            .map_err(|e| {
                E::custom(format!(
                    "failed to decode string {} from base64: {:?}",
                    value, e
                ))
            })
    }
}

impl<'de> Visitor<'de> for Base64StringVisitor {
    type Value = Base64String;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a base64-encoded string")
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        self.decode_string(value)
    }

    fn visit_string<E>(self, value: String) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        self.decode_string(&value)
    }
}

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

#[allow(dead_code)]
fn header_to_tendermint_header(header: &Header) -> Result<TmHeader, Error> {
    let last_block_id = match &header.last_block_id {
        Some(last_block_id) => Some(TmBlockId {
            hash: TmHash::try_from(last_block_id.hash.0.clone())?,
            part_set_header: TmPartSetHeader::new(
                last_block_id.part_set_header.total,
                TmHash::try_from(last_block_id.part_set_header.hash.0.clone())?,
            )?,
        }),
        None => None,
    };

    let last_commit_hash = match &header.last_commit_hash {
        Some(last_commit_hash) => Some(TmHash::try_from(last_commit_hash.0.clone())?),
        None => None,
    };

    let data_hash = match &header.data_hash {
        Some(data_hash) => Some(TmHash::try_from(data_hash.0.clone())?),
        None => None,
    };

    let last_results_hash = match &header.last_results_hash {
        Some(last_results_hash) => Some(TmHash::try_from(last_results_hash.0.clone())?),
        None => None,
    };

    let evidence_hash = match &header.evidence_hash {
        Some(evidence_hash) => Some(TmHash::try_from(evidence_hash.0.clone())?),
        None => None,
    };

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
        println!("LatestBlockResponse: {:?}", resp);

        let tm_header = header_to_tendermint_header(&resp.block.header).unwrap();
        let tm_header_hash = tm_header.hash();
        assert_eq!(tm_header_hash.as_bytes(), &resp.block_id.hash.0);
    }
}
