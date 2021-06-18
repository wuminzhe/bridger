
#[macro_use]
extern crate log;

use bus::{
    BridgerBus, BridgerMessage
};
use lifeline::prelude::*;
use postage::{sink::Sink, stream::Stream};
use ethlike_tracker::{EthlikeTracker, EthlikeClient, Ethereum, DefaultLogsHandler};

use web3::{
    Web3,
    transports::Http,
    types::{H160, H256, Log}
};

use array_bytes::hex2bytes_unchecked as bytes;

pub struct EthereumTrackerService {
    line: Lifeline,
}

impl Service for EthereumTrackerService {
    type Bus = BridgerBus;
    type Lifeline = anyhow::Result<Self>;

    fn spawn(bus: &Self::Bus) -> Self::Lifeline {

        // let mut rx = bus.rx::<BridgerMessage>()?;
        // let mut _tx = bus.tx::<BridgerMessage>()?;

        let line = Self::try_task("ethereum-track-task", async move {
            let web3 = Web3::new(Http::new("https://ropsten.infura.io/v3/60703fcc6b4e48079cfc5e385ee7af80").unwrap());

            let contract_address = "0xD35Bb6F1bc1C84b53E0995c1830454AB7C4147f1";
            let contract_address = H160::from_slice(&bytes(contract_address));

            let topics = &vec!["0x96635f5f1b0b05ed7e2265d4e13634378280f038e5a958227d4f383f825c2771"];
            let topics = topics
                .iter()
                .map(|t| H256::from_slice(&bytes(t)))
                .collect();

            let client = EthlikeClient::new(web3);
            let mut tracker = EthlikeTracker::<Ethereum, DefaultLogsHandler>::new(
                client,
                vec![(contract_address, topics)],
                DefaultLogsHandler {},
                100,
            );

            tracker.start().await;
            Ok(())
        });

        Ok(Self { line })
    }
}