use crate::{EthlikeClient, Result};
use web3::types::{Log, H160, H256};

#[async_trait]
pub trait EthlikeChain {
    const NAME: &'static str;

    async fn next_range(from: u64, client: &EthlikeClient) -> Result<(u64, u64)>;
}

#[async_trait]
pub trait LogsHandler {
    async fn handle(
        &self,
        client: &EthlikeClient,
        topics_list: &Vec<(H160, Vec<H256>)>,
        logs: Vec<Log>,
    ) -> Result<()>;
}
