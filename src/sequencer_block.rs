use anyhow::Error;
use base64::{engine::general_purpose, Engine as _};
use hex;
use protobuf::Message;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use tracing::{debug, info, warn};

use crate::proto::msg::SequencerMsg;
use crate::proto::tx::{TxBody, TxRaw};
use crate::types::{Base64String, Block};

static SEQUENCER_TYPE_URL: &str = "/SequencerMsg";

// get_namespace returns an 8-byte namespace given a byte slice.
pub(crate) fn get_namespace(bytes: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(bytes);
    let result = hasher.finalize();
    hex::encode(&result[0..8])
}

/// SequencerBlock represents a sequencer layer block to be submitted to
/// the DA layer.
/// Currently, it consists of the Block.Data field of the cosmos-sdk block
/// returned by a sequencer, which contains the block's transactions.
/// TODO: compression or a better serialization method?
/// TODO: rename this b/c it's kind of confusing, types::Block is a cosmos-sdk block
/// which is also a sequencer block in a way.
#[derive(Serialize, Deserialize, Debug)]
pub struct SequencerBlock {
    pub block_hash: Base64String,
    pub sequencer_txs: Vec<Base64String>, // TODO: do we need this?
    /// namespace -> rollup txs
    pub rollup_txs: HashMap<String, Vec<Base64String>>,
}

impl SequencerBlock {
    /// from_cosmos_block converts a cosmos-sdk block into a SequencerBlock.
    /// it parses the block for SequencerMsgs and namespaces them accordingly.
    pub fn from_cosmos_block(b: Block) -> Result<Self, Error> {
        // we unwrap generic txs into rollup-specific txs here,
        // and namespace them correspondingly
        let mut sequencer_txs = vec![];
        let mut rollup_txs = HashMap::new();

        for tx in b.data.txs.iter() {
            debug!(
                "parsing tx: {:?}",
                general_purpose::STANDARD.encode(tx.0.clone())
            );

            let tx_body = parse_cosmos_tx(tx)?;
            let msgs = cosmos_tx_body_to_sequencer_msgs(tx_body)?;

            for msg in msgs {
                info!("parsed SequencerMsg: {:?}", msg);
                let namespace = msg.chain_id;
                if namespace.is_empty() {
                    // TODO: should we allow this case? seems sus
                    sequencer_txs.push(Base64String(msg.data));
                    continue;
                }

                let txs = rollup_txs
                    .entry(get_namespace(&namespace))
                    .or_insert(vec![]);
                txs.push(tx.clone());
            }
        }

        Ok(Self {
            block_hash: b.header.data_hash, // TODO: is this the right hash?
            sequencer_txs,
            rollup_txs,
        })
    }
}

fn parse_cosmos_tx(tx: &Base64String) -> Result<TxBody, Error> {
    let tx_raw = TxRaw::parse_from_bytes(&tx.0)?;
    let tx_body = TxBody::parse_from_bytes(&tx_raw.body_bytes)?;
    Ok(tx_body)
}

fn cosmos_tx_body_to_sequencer_msgs(tx_body: TxBody) -> Result<Vec<SequencerMsg>, Error> {
    let mut msgs = vec![];
    for msg in tx_body.messages {
        debug!("parsing cosmos-sdk message: {:?}", msg);
        if msg.type_url == SEQUENCER_TYPE_URL {
            let msg = SequencerMsg::parse_from_bytes(&msg.value)?;
            msgs.push(msg);
        } else {
            // TODO: do we want to write sequencer "primary txs" to the DA layer?
            warn!("ignoring message with non-sequencer type URL: {:?}", msg);
        }
    }
    Ok(msgs)
}

#[cfg(test)]
mod test {
    use super::{cosmos_tx_body_to_sequencer_msgs, parse_cosmos_tx, SEQUENCER_TYPE_URL};
    use crate::types::Base64String;

    #[test]
    fn test_parse_primary_tx() {
        let primary_tx = "CosBCogBChwvY29zbW9zLmJhbmsudjFiZXRhMS5Nc2dTZW5kEmgKLG1ldHJvMXFwNHo0amMwdndxd3hzMnl0NmNrNDRhZWo5bWV5ZnQ0eHg4bXN5EixtZXRybzEwN2Nod2U2MGd2Z3JneXlmbjAybWRsNmxuNjd0dndtOGhyZjR2MxoKCgV1dGljaxIBMRJsClAKRgofL2Nvc21vcy5jcnlwdG8uc2VjcDI1NmsxLlB1YktleRIjCiEDkoWc0MT/06rTUjNPZcvNLqcQJtOvzIWtenGsJXEfEJkSBAoCCAEYBRIYChAKBXV0aWNrEgcxMDAwMDAwEICU69wDGkBeBi44QbvLMvzndkNj+6dckqOR19eNTKV9qZyvtVOrj1+UN/VqeN9Rf0+M6Rmg24uNE5A4jsRcTXh7RkUm9ItT".to_string();
        let tx = parse_cosmos_tx(&Base64String::from_string(primary_tx).unwrap()).unwrap();
        assert_eq!(tx.messages.len(), 1);
        assert_eq!(tx.messages[0].type_url, "/cosmos.bank.v1beta1.MsgSend");
        let sequencer_msgs = cosmos_tx_body_to_sequencer_msgs(tx).unwrap();
        assert_eq!(sequencer_msgs.len(), 0);
    }

    #[test]
    fn test_parse_secondary_tx() {
        let secondary_tx = "Ck0KSwoNL1NlcXVlbmNlck1zZxI6CgNhYWESBWhlbGxvGixtZXRybzFwbHprNzZuamVzdmR0ZnhubTI2dHl5NmV2NGxjYTh3dmZ1M2Q1cxJxClAKRgofL2Nvc21vcy5jcnlwdG8uc2VjcDI1NmsxLlB1YktleRIjCiECjL7oF1zd07+3mCVNz4YHGRleoPDWP08/rGDh14xTkvgSBAoCCAEYBBIYChAKBXV0aWNrEgcxMDAwMDAwEICU69wDIgNhYWEaQMzTIFlWe+yur00V3pXJEZ8uo6AzZ81Q1JJjD+u5EgGDKBslbiabXjPwiRcRMyuHRekBVOGLjNoAPsbhr0F+lTI=".to_string();
        let tx = parse_cosmos_tx(&Base64String::from_string(secondary_tx).unwrap()).unwrap();
        assert_eq!(tx.messages.len(), 1);
        assert_eq!(tx.messages[0].type_url, SEQUENCER_TYPE_URL);
        let sequencer_msgs = cosmos_tx_body_to_sequencer_msgs(tx).unwrap();
        assert_eq!(sequencer_msgs.len(), 1);
        assert_eq!(sequencer_msgs[0].chain_id, "aaa".as_bytes());
        assert_eq!(sequencer_msgs[0].data, "hello".as_bytes());
    }
}
