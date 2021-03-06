use crate::{EthlikeChain, EthlikeClient, LogsHandler, Result};
use web3::types::{Log, H160, H256};

pub struct DefaultLogsHandler;

#[async_trait]
impl LogsHandler for DefaultLogsHandler {
    async fn handle(
        &self,
        _client: &EthlikeClient,
        _topics_list: &Vec<(H160, Vec<H256>)>,
        logs: Vec<Log>,
    ) -> Result<()> {
        for log in logs {
            info!("{:?}", log);
        }
        Ok(())
    }
}
