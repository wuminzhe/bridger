mod darwinia_tracker;
mod error;

#[macro_use]
extern crate log;

pub type Error = error::Error;
pub type Result<T> = std::result::Result<T, Error>;

use bus::{
	BridgerBus, BridgerMessage
};
use lifeline::prelude::*;
use postage::{sink::Sink, stream::Stream};
use darwinia::Darwinia;
use crate::darwinia_tracker::DarwiniaBlockTracker;

pub struct DarwiniaRelayerService {
	line: Lifeline,
	line2: Lifeline,
}

impl Service for DarwiniaRelayerService {
	type Bus = BridgerBus;
	type Lifeline = anyhow::Result<Self>;

	fn spawn(bus: &Self::Bus) -> Self::Lifeline {

		let mut rx = bus.rx::<BridgerMessage>()?;
		let mut tx = bus.tx::<BridgerMessage>()?;

		let line = Self::try_task("darwinia-relayer", async move {
			// while let Some(recv) = rx.recv().await {
			// 	println!(">>------------------- {:?}", recv);
			// }
			Ok(())
		});

		let line2 = Self::try_task("darwinia-relayer-worker", async move {
			match Darwinia::new("wss://rpc.darwinia.network").await {
				Ok(darwinia) => {
					let mut tracker = DarwiniaBlockTracker::new(darwinia, 10000);
					loop {
						match tracker.next_block().await {
							Ok(header) => {
								// let header_string = format!("{:?}", header);
								let msg = BridgerMessage::DarwiniaHeader(
									header
								);

								tx.send(msg).await?;
							}
							Err(_err) => {
								println!("----------");
								// return self.start(app).await;
							}
						}
					}
				}
				Err(_err) => {
					println!("2----------");
					// return Err(service::Error::Stopped(
							// "未能连接上 Darwinia 节点, 请检查网络",
							// ));
				}
			}

			Ok(())
		});

		Ok(Self { line, line2 })
	}
}
