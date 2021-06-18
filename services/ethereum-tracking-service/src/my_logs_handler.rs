use bus::DarwiniaAffirmMessage;
use ethlike_tracker::{EthlikeChain, EthlikeClient, LogsHandler, Result};
use postage::broadcast::Sender;
use web3::types::{Log, H160, H256};

pub struct MyLogsHandler {
    tx: Sender<DarwiniaAffirmMessage>,
}

impl MyLogsHandler {
    pub fn new(tx: Sender<DarwiniaAffirmMessage>) -> Self {
        MyLogsHandler { tx }
    }
}

#[async_trait]
impl LogsHandler for MyLogsHandler {
    async fn handle(
        &self,
        _client: &EthlikeClient,
        _topics_list: &Vec<(H160, Vec<H256>)>,
        logs: Vec<Log>,
    ) -> Result<()> {
        for log in logs {
            info!("----------------------------------");
            info!("{:?}", log);
        }
        Ok(())
    }
}
