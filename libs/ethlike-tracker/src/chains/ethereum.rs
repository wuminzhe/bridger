use crate::{Result, EthlikeChain, EthlikeClient};
use async_trait::async_trait;
use tokio::time::{delay_for, Duration};

/// Ethereum
pub struct Ethereum;

#[async_trait]
impl EthlikeChain for Ethereum {
    const NAME: &'static str = "Ethereum";

    async fn next_range(from: u64, client: &EthlikeClient) -> Result<(u64, u64)> {
        let to = client.get_latest_block_number().await?;
        if to - from > 10 {
            let result = (from, to);
            Ok(result)
        } else {
            delay_for(Duration::from_secs(30)).await;
            Ethereum::next_range(from, client).await
        }
    }
}