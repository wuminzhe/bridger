use crate::{Result, EthlikeChain, EthlikeClient};
use async_trait::async_trait;
use tokio::time::{delay_for, Duration};

/// Binance Smart Chain
pub struct Bsc;

#[async_trait]
impl EthlikeChain for Bsc {
    const NAME: &'static str = "Bsc";

    async fn next_range(from: u64, client: &EthlikeClient) -> Result<(u64, u64)> {
        let latest = client.get_latest_block_number().await?;
        let to = if from + 5000 >= latest {
            latest
        } else {
            from + 5000
        };
        if to - from > 5 {
            let result = (from, to);
            Ok(result)
        } else {
            delay_for(Duration::from_secs(30)).await;
            Bsc::next_range(from, client).await
        }
    }
}