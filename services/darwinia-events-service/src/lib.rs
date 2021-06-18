
#[macro_use]
extern crate log;

use bus::{
	BridgerBus, BridgerMessage
};
use lifeline::prelude::*;
use postage::{sink::Sink, stream::Stream};
use darwinia::Darwinia;

pub struct DarwiniaEventsService {
	line: Lifeline,
}

impl Service for DarwiniaEventsService {
	type Bus = BridgerBus;
	type Lifeline = anyhow::Result<Self>;

	fn spawn(bus: &Self::Bus) -> Self::Lifeline {

		let mut rx = bus.rx::<BridgerMessage>()?;
		let mut _tx = bus.tx::<BridgerMessage>()?;

		let line = Self::try_task("darwinia-events", async move {
			while let Some(recv) = rx.recv().await {
				match recv {
					BridgerMessage::DarwiniaHeader(ref header) => {
						// tx.send(ExampleSend::OhHello).await?;
						info!("Darwinia block {}", header.number);


					},
					_ => {
					}
				}
			}
			Ok(())
		});

		Ok(Self { line })
	}
}

#[cfg(test)]
mod tests {
	#[test]
	fn it_works() {
		assert_eq!(2 + 2, 4);
	}
}
