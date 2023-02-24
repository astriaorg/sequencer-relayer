use base64::{engine::general_purpose, Engine as _};
use hex;
use serde::{
    de::{self, Visitor},
    Deserialize, Deserializer, Serialize,
};
use std::fmt;

/// cosmos-sdk RPC types.
/// see https://v1.cosmos.network/rpc/v0.41.4

#[derive(Serialize, Debug)]
pub struct EmptyRequest {}

pub struct Hash(Vec<u8>);

impl fmt::Debug for Hash {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&hex::encode(&self.0))
    }
}

impl<'de> Deserialize<'de> for Hash {
    fn deserialize<D>(deserializer: D) -> Result<Hash, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_string(HashVisitor)
    }
}

struct HashVisitor;

impl<'de> Visitor<'de> for HashVisitor {
    type Value = Hash;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a base64-encoded string")
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        let bytes_res = &general_purpose::STANDARD.decode(value);
        match bytes_res {
            Ok(bytes) => Ok(Hash(bytes.to_vec())),
            Err(e) => Err(E::custom(format!(
                "failed to decode string {} from base64: {:?}",
                value, e
            ))),
        }
    }

    fn visit_string<E>(self, value: String) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        let bytes_res = &general_purpose::STANDARD.decode(&value);
        match bytes_res {
            Ok(bytes) => Ok(Hash(bytes.to_vec())),
            Err(e) => Err(E::custom(format!(
                "failed to decode string {} from base64: {:?}",
                &value, e
            ))),
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct LatestBlockResponse {
    pub block_id: BlockId,
    pub block: Block,
}

#[derive(Deserialize, Debug)]
pub struct BlockId {
    pub hash: Hash,
    // TODO: part_set_header
}

#[derive(Deserialize, Debug)]
pub struct Block {
    pub header: Header,
    pub data: Data,
    // TODO: evidence
}

#[derive(Deserialize, Debug)]
pub struct Header {
    // TODO: version
    pub chain_id: String,
    pub height: String,
    pub time: String,
    // TODO: last_block_id
    pub last_commit_hash: Hash,
    pub data_hash: Hash,
    pub validators_hash: Hash,
    pub next_validators_hash: Hash,
    pub consensus_hash: Hash,
    pub app_hash: Hash,
    pub last_results_hash: Hash,
    pub evidence_hash: Hash,
    pub proposer_address: Hash,
}

#[derive(Deserialize, Debug)]
pub struct Data {
    pub txs: Vec<String>,
}
