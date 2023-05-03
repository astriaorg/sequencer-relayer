use sequencer_relayer::{sequencer::SequencerClient, transaction};
use crate::helper::init_test;

#[tokio::test]
async fn txs_to_data_hash() {
    let test_env = init_test().await;
    let sequencer_endpoint = test_env.sequencer_endpoint();
    let client = SequencerClient::new(sequencer_endpoint).unwrap();

    let resp = client.get_latest_block().await.unwrap();
    let data_hash = transaction::txs_to_data_hash(&resp.block.data.txs);
    assert_eq!(
        data_hash.as_bytes(),
        &resp.block.header.data_hash.unwrap().0
    );
}
