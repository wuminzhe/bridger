mod config;
mod my_logs_handler;

#[macro_use]
extern crate log;

#[macro_use]
extern crate async_trait;

use bus::{BridgerBus, BridgerMessage, DarwiniaAffirmMessage};
use ethlike_tracker::{Ethereum, EthlikeClient, EthlikeTracker};
use lifeline::prelude::*;
use postage::{sink::Sink, stream::Stream};

use web3::{
    transports::Http,
    types::{Log, H160, H256},
    Web3,
};

use crate::my_logs_handler::MyLogsHandler;
use array_bytes::hex2bytes_unchecked as bytes;

pub struct EthereumTrackerService {
    line: Lifeline,
}

impl Service for EthereumTrackerService {
    type Bus = BridgerBus;
    type Lifeline = anyhow::Result<Self>;

    fn spawn(bus: &Self::Bus) -> Self::Lifeline {
        let mut tx = bus.tx::<DarwiniaAffirmMessage>()?;

        let line = Self::try_task("ethereum-track-task", async move {
            let cfg: config::Ethereum = confy::load("bridger", Some("ethereum-tracking-service"))?;

            let web3 = Web3::new(
                Http::new(cfg.rpc.as_str()).unwrap(),
            );

            let mut topics_list = vec![];
            for contract in cfg.contracts {
                let contract_address = H160::from_slice(&bytes(contract.address.as_str()));
                if let Some(topics) = contract.topics {
                    let topics = topics.iter().map(|t| H256::from_slice(&bytes(t))).collect::<Vec<H256>>();
                    topics_list.push((contract_address, topics));
                }
            }


            let client = EthlikeClient::new(web3);
            let mut tracker = EthlikeTracker::<Ethereum, MyLogsHandler>::new(
                client,
                topics_list,
                MyLogsHandler::new(tx),
                100,
            );

            tracker.start().await;
            Ok(())
        });

        Ok(Self { line })
    }
}
