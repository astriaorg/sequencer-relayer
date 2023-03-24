use anyhow::{anyhow, Error};
use ed25519_dalek::{ed25519::signature::Signature, Keypair, PublicKey, Signer, Verifier};
use rs_cnc::{CelestiaNodeClient, NamespacedSharesResponse, PayForDataResponse};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use tracing::{debug, warn};

use crate::base64_string::Base64String;
use crate::sequencer_block::{IndexedTransaction, Namespace, SequencerBlock, DEFAULT_NAMESPACE};
use crate::types::Header;

static DEFAULT_PFD_FEE: i64 = 2_000;
static DEFAULT_PFD_GAS_LIMIT: u64 = 90_000;

/// SubmitBlockResponse is the response to a SubmitBlock request.
/// It contains a map of namespaces to the block number that it was written to.
pub struct SubmitBlockResponse {
    pub namespace_to_block_num: HashMap<String, Option<u64>>,
}

/// SequencerNamespaceData represents the data written to the "base"
/// sequencer namespace. It contains all the other namespaces that were
/// also written to in the same block.
#[derive(Serialize, Deserialize, Debug)]
struct SequencerNamespaceData {
    block_hash: Base64String,
    header: Header,
    sequencer_txs: Vec<IndexedTransaction>,
    /// vector of (block height, namespace) tuples
    rollup_namespaces: Vec<(u64, String)>,
}

impl SequencerNamespaceData {
    fn hash(&self) -> Result<Vec<u8>, Error> {
        let mut hasher = Sha256::new();
        hasher.update(self.to_bytes()?);
        let hash = hasher.finalize();
        Ok(hash.to_vec())
    }

    fn to_signed(
        self: SequencerNamespaceData,
        keypair: &Keypair,
    ) -> Result<SignedSequencerNamespaceData, Error> {
        let hash = self.hash()?;

        let data = SignedSequencerNamespaceData {
            data: self,
            signature: Base64String(keypair.sign(&hash).as_bytes().to_vec()),
        };
        Ok(data)
    }

    fn to_bytes(&self) -> Result<Vec<u8>, Error> {
        // TODO: don't use json, use our own serializer (or protobuf for now?)
        let string = serde_json::to_string(self).map_err(|e| anyhow!(e))?;
        Ok(string.into_bytes())
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct SignedSequencerNamespaceData {
    data: SequencerNamespaceData,
    signature: Base64String,
}

impl SignedSequencerNamespaceData {
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

/// RollupNamespaceData represents the data written to a rollup namespace.
#[derive(Serialize, Deserialize, Debug)]
struct RollupNamespaceData {
    block_hash: Base64String,
    rollup_txs: Vec<IndexedTransaction>,
}

impl RollupNamespaceData {
    fn hash(&self) -> Result<Vec<u8>, Error> {
        let mut hasher = Sha256::new();
        hasher.update(self.to_bytes()?);
        let hash = hasher.finalize();
        Ok(hash.to_vec())
    }

    fn to_signed(
        self: RollupNamespaceData,
        keypair: &Keypair,
    ) -> Result<SignedRollupNamespaceData, Error> {
        let hash = self.hash()?;
        let data = SignedRollupNamespaceData {
            data: self,
            signature: Base64String(keypair.sign(&hash).as_bytes().to_vec()),
        };
        Ok(data)
    }

    fn to_bytes(&self) -> Result<Vec<u8>, Error> {
        // TODO: don't use json, use our own serializer (or protobuf for now?)
        let string = serde_json::to_string(self).map_err(|e| anyhow!(e))?;
        Ok(string.into_bytes())
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct SignedRollupNamespaceData {
    data: RollupNamespaceData,
    signature: Base64String,
}

impl SignedRollupNamespaceData {
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

/// CelestiaClient is a DataAvailabilityClient that submits blocks to a Celestia Node.
pub struct CelestiaClient {
    client: CelestiaNodeClient,
    keypair: Keypair,
}

impl CelestiaClient {
    pub fn new(endpoint: String, keypair: Keypair) -> Result<Self, Error> {
        let cnc = CelestiaNodeClient::new(endpoint)?;
        Ok(CelestiaClient {
            client: cnc,
            keypair,
        })
    }

    async fn submit_namespaced_data(
        &self,
        namespace: &str,
        data: &[u8],
    ) -> Result<PayForDataResponse, Error> {
        let pay_for_data_response = self
            .client
            .submit_pay_for_data(
                namespace,
                &data.to_vec().into(),
                DEFAULT_PFD_FEE,
                DEFAULT_PFD_GAS_LIMIT,
            )
            .await?;
        Ok(pay_for_data_response)
    }

    pub async fn submit_block(&self, block: SequencerBlock) -> Result<SubmitBlockResponse, Error> {
        let mut namespace_to_block_num: HashMap<String, Option<u64>> = HashMap::new();
        let mut block_height_and_namespace: Vec<(u64, String)> = Vec::new();

        // then, format and submit data for each rollup namespace
        for (namespace, txs) in block.rollup_txs {
            debug!(
                "submitting rollup namespace data for namespace {}",
                namespace
            );
            let rollup_namespace_data = RollupNamespaceData {
                block_hash: block.block_hash.clone(),
                rollup_txs: txs,
            };
            let rollup_data_bytes = rollup_namespace_data.to_signed(&self.keypair)?.to_bytes()?;
            let resp = self
                .submit_namespaced_data(&namespace.to_string(), &rollup_data_bytes)
                .await?;
            namespace_to_block_num.insert(namespace.to_string(), resp.height);
            block_height_and_namespace.push((resp.height.unwrap(), namespace.to_string()))
            // TODO: no unwrap
        }

        // first, format and submit data to the base sequencer namespace
        let sequencer_namespace_data = SequencerNamespaceData {
            block_hash: block.block_hash.clone(),
            header: block.header,
            sequencer_txs: block.sequencer_txs,
            rollup_namespaces: block_height_and_namespace,
        };

        let bytes = sequencer_namespace_data
            .to_signed(&self.keypair)?
            .to_bytes()?;
        let resp = self
            .submit_namespaced_data(&DEFAULT_NAMESPACE.to_string(), &bytes)
            .await?;
        namespace_to_block_num.insert(DEFAULT_NAMESPACE.to_string(), resp.height);

        Ok(SubmitBlockResponse {
            namespace_to_block_num,
        })
    }

    pub async fn check_block_availability(
        &self,
        height: u64,
    ) -> Result<NamespacedSharesResponse, Error> {
        let resp = self
            .client
            .namespaced_shares(&DEFAULT_NAMESPACE.to_string(), height)
            .await?;
        Ok(resp)
    }

    pub async fn get_blocks(
        &self,
        height: u64,
        public_key: PublicKey,
    ) -> Result<Vec<SequencerBlock>, Error> {
        let namespaced_data_response = self
            .client
            .namespaced_data(&DEFAULT_NAMESPACE.to_string(), height)
            .await?;

        // retrieve all sequencer blocks stored at this height
        let sequencer_namespace_datas: Vec<SignedSequencerNamespaceData> = namespaced_data_response
            .data
            .unwrap_or_default()
            .iter()
            .filter_map(|d| {
                if let Ok(data) = SignedSequencerNamespaceData::from_bytes(&d.0) {
                    Some(data)
                } else {
                    None
                }
            })
            .collect();

        // find data that was signed by the given public key
        let sequencer_namespace_datas = sequencer_namespace_datas
            .into_iter()
            .filter(|d| {
                let hash = match d.data.hash() {
                    Ok(hash) => hash,
                    Err(_) => return false,
                };

                match Signature::from_bytes(&d.signature.0) {
                    Ok(sig) => public_key.verify(&hash, &sig).is_ok(),
                    Err(_) => false,
                }
            })
            .collect::<Vec<_>>();

        let mut blocks = vec![];

        // TODO: there should NOT be multiple datas with the same block hash and signer

        // for all the sequencer blocks retrieved, create the corresponding SequencerBlock
        for sequencer_namespace_data in &sequencer_namespace_datas {
            let rollup_namespaces = sequencer_namespace_data.data.rollup_namespaces.clone();
            let mut rollup_txs_map = HashMap::new();

            // for each rollup namespace, retrieve the corresponding rollup block
            for (height, rollup_namespace) in rollup_namespaces {
                let namespaced_data_response = self
                    .client
                    .namespaced_data(&rollup_namespace.to_string(), height)
                    .await?;

                let rollup_datas: Vec<SignedRollupNamespaceData> = namespaced_data_response
                    .data
                    .unwrap_or_default()
                    .iter()
                    .filter_map(|d| {
                        if let Ok(data) = SignedRollupNamespaceData::from_bytes(&d.0) {
                            Some(data)
                        } else {
                            warn!("failed to deserialize rollup namespace data");
                            None
                        }
                    })
                    .filter(|d| {
                        let hash = match d.data.hash() {
                            Ok(hash) => hash,
                            Err(_) => return false,
                        };

                        match Signature::from_bytes(&d.signature.0) {
                            Ok(sig) => public_key.verify(&hash, &sig).is_ok(),
                            Err(_) => false,
                        }
                    })
                    .collect();

                // TODO: there should NOT be multiple datas with the same block hash and signer

                for rollup_data in rollup_datas {
                    // TODO: there a chance multiple blocks could be written with the same block hash, however
                    // only one will be valid; we need to sign this before submitting and verify sig upon reading
                    if rollup_data.data.block_hash == sequencer_namespace_data.data.block_hash {
                        let namespace = Namespace::from_string(&rollup_namespace)?;
                        // this replaces what was already in the map, this is bad, but ok for now
                        // since we need to implementation sig verification anyways.
                        rollup_txs_map.insert(namespace, rollup_data.data.rollup_txs);
                    }
                }
            }

            blocks.push(SequencerBlock {
                block_hash: sequencer_namespace_data.data.block_hash.clone(),
                header: sequencer_namespace_data.data.header.clone(),
                sequencer_txs: sequencer_namespace_data.data.sequencer_txs.clone(),
                rollup_txs: rollup_txs_map,
            });
        }

        Ok(blocks)
    }
}

#[cfg(test)]
mod tests {
    use ed25519_dalek::{Keypair, PublicKey};
    use rand::rngs::OsRng;
    use std::collections::HashMap;

    use super::{CelestiaClient, SequencerBlock, DEFAULT_NAMESPACE};
    use crate::base64_string::Base64String;
    use crate::sequencer_block::{get_namespace, IndexedTransaction};

    #[tokio::test]
    async fn test_celestia_client() {
        // unfortunately, this needs to be all one test for now, since
        // submitting multiple blocks to celestia concurrently returns
        // "incorrect account sequence" errors.

        // test submit_block
        let keypair = Keypair::generate(&mut OsRng);
        let public_key = PublicKey::from_bytes(&keypair.public.to_bytes()).unwrap();

        let base_url = "http://localhost:26659".to_string();
        let client = CelestiaClient::new(base_url, keypair).unwrap();
        let tx = Base64String(b"noot_was_here".to_vec());
        let secondary_namespace = get_namespace(b"test_namespace");
        let secondary_tx = Base64String(b"noot_was_here_too".to_vec());

        let block_hash = Base64String(vec![99; 32]);
        let mut block = SequencerBlock {
            block_hash: block_hash.clone(),
            header: Default::default(),
            sequencer_txs: vec![IndexedTransaction {
                index: 0,
                transaction: tx.clone(),
            }],
            rollup_txs: HashMap::new(),
        };
        block.rollup_txs.insert(
            secondary_namespace.clone(),
            vec![IndexedTransaction {
                index: 1,
                transaction: secondary_tx.clone(),
            }],
        );

        let submit_block_resp = client.submit_block(block).await.unwrap();
        #[allow(clippy::unnecessary_to_owned)]
        let height = submit_block_resp
            .namespace_to_block_num
            .get(&DEFAULT_NAMESPACE.to_string())
            .unwrap()
            .unwrap();

        // test check_block_availability
        let resp = client.check_block_availability(height).await.unwrap();
        assert_eq!(resp.height, height);

        // test get_blocks
        let resp = client.get_blocks(height, public_key).await.unwrap();
        assert_eq!(resp.len(), 1);
        assert_eq!(resp[0].block_hash, block_hash);
        assert_eq!(resp[0].header, Default::default());
        assert_eq!(resp[0].sequencer_txs.len(), 1);
        assert_eq!(resp[0].sequencer_txs[0].index, 0);
        assert_eq!(resp[0].sequencer_txs[0].transaction, tx);
        assert_eq!(resp[0].rollup_txs.len(), 1);
        assert_eq!(resp[0].rollup_txs[&secondary_namespace][0].index, 1);
        assert_eq!(
            resp[0].rollup_txs[&secondary_namespace][0].transaction,
            secondary_tx
        );
    }
}
