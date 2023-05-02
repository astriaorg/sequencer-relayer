use eyre::Result;
use gossipnet::network::{Network, NetworkBuilder, Sha256Topic};

use crate::sequencer_block::SequencerBlock;

const BLOCKS_TOPIC: &str = "blocks";

pub(crate) struct GossipNetwork(pub(crate) Network);

impl GossipNetwork {
    pub(crate) fn new() -> Result<Self> {
        let network = NetworkBuilder::new().build()?;
        Ok(Self(network))
    }

    pub(crate) async fn publish(&mut self, block: &SequencerBlock) -> Result<()> {
        self.0
            .publish(block.to_bytes()?, Sha256Topic::new(BLOCKS_TOPIC))
            .await?;
        Ok(())
    }
}
