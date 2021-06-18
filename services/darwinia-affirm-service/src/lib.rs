#[macro_use]
extern crate log;

use bus::{BridgerBus, DarwiniaAffirmMessage};
use darwinia::Darwinia;
use lifeline::prelude::*;
use postage::{sink::Sink, stream::Stream};

pub struct DarwiniaAffirmService {
    line: Lifeline,
}

impl Service for DarwiniaAffirmService {
    type Bus = BridgerBus;
    type Lifeline = anyhow::Result<Self>;

    fn spawn(bus: &Self::Bus) -> Self::Lifeline {
        let mut rx = bus.rx::<DarwiniaAffirmMessage>()?;
        // let mut tx = bus.tx::<BridgerMessage>()?;

        let line = Self::try_task("darwinia-affirm-task", async move {
            let mut affirmed = 1u64;
            let mut target = 1u64;
            while let Some(recv) = rx.recv().await {
                match recv {
                    DarwiniaAffirmMessage::LastAffirmed(block_number) => {
                        affirmed = block_number;
                    }
                    DarwiniaAffirmMessage::ToAffirm(block_number) => {
                        info!("++++ to affirm {}", block_number);
                        target = block_number;
                    }
                    DarwiniaAffirmMessage::DoAffirm => {
                        if target > affirmed {
                            println!("do affirm");
                            affirmed = target;
                            // let result = DarwiniaAffirmService::affirm(
                            // 	this.ethereum2darwinia.clone(),
                            // 	this.shadow.clone(),
                            // 	this.target,
                            // 	this.extrinsics_service.clone(),
                            // ).await;
                            //
                            // match result {
                            // 	Ok(()) => {
                            // 		relayed = target;
                            // 	},
                            // 	Err(err) => {
                            // 		trace!("{}", err);
                            // 	}
                            // }
                        }
                    }
                }
            }
            Ok(())
        });

        Ok(Self { line })
    }

    // async fn affirm(
    // 	ethereum2darwinia: Ethereum2Darwinia,
    // 	shadow: Arc<Shadow>,
    // 	target: u64,
    // 	extrinsics_service: Recipient<MsgExtrinsic>,
    // ) -> Result<()> {
    // 	// /////////////////////////
    // 	// checking before affirm
    // 	// /////////////////////////
    // 	// 1. last confirmed check
    // 	let last_confirmed = ethereum2darwinia.last_confirmed().await?;
    // 	if target <= last_confirmed {
    // 		return Err(
    // 			BizError::AffirmingBlockLessThanLastConfirmed(target, last_confirmed).into(),
    // 		);
    // 	}
    //
    // 	// 2. pendings check
    // 	let pending_headers = ethereum2darwinia.pending_headers().await?;
    // 	for pending_header in pending_headers {
    // 		let pending_block_number = pending_header.1.header.number;
    // 		if pending_block_number >= target {
    // 			return Err(BizError::AffirmingBlockInPending(target).into());
    // 		}
    // 	}
    //
    // 	// 3. affirmations check
    // 	for (_game_id, game) in ethereum2darwinia.affirmations().await?.iter() {
    // 		for (_round_id, affirmations) in game.iter() {
    // 			if Ethereum2Darwinia::contains(&affirmations, target) {
    // 				return Err(BizError::AffirmingBlockInGame(target).into());
    // 			}
    // 		}
    // 	}
    //
    // 	trace!("Prepare to affirm ethereum block: {}", target);
    // 	let parcel = shadow.parcel(target as usize + 1).await.with_context(|| {
    // 		format!(
    // 			"Fail to get parcel from shadow when affirming ethereum block {}",
    // 			target
    // 		)
    // 	})?;
    // 	if parcel.header == EthereumHeader::default() || parcel.mmr_root == [0u8; 32] {
    // 		return Err(BizError::ParcelFromShadowIsEmpty(target).into());
    // 	}
    //
    // 	// /////////////////////////
    // 	// do affirm
    // 	// /////////////////////////
    // 	let ex = Extrinsic::Affirm(parcel);
    // 	tools::send_extrinsic(&extrinsics_service, ex).await;
    //
    // 	Ok(())
    // }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
