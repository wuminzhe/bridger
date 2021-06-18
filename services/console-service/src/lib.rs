#[macro_use]
extern crate log;

use bus::{BridgerBus, DarwiniaAffirmMessage};
use lifeline::prelude::*;
use postage::{sink::Sink, stream::Stream};

use postage::broadcast::Sender;
use std::rc::Rc;
use std::sync::{Arc, Mutex};
use tide::prelude::*;
use tide::{Body, Request};

/// The shared application state.
#[derive(Clone)]
struct State {
    pub tx: Sender<DarwiniaAffirmMessage>,
}

#[derive(Debug, Deserialize)]
struct Affirmation {
    block_number: u64,
}

async fn submit_affirmation(mut req: Request<()>) -> tide::Result {
    let Affirmation { block_number } = req.body_json().await?;

    println!("-----------------{}", block_number);
    Ok(format!("Hello").into())
}

pub struct ConsoleService {
    line: Lifeline,
}

impl Service for ConsoleService {
    type Bus = BridgerBus;
    type Lifeline = anyhow::Result<Self>;

    fn spawn(bus: &Self::Bus) -> Self::Lifeline {
        let tx = bus.tx::<DarwiniaAffirmMessage>()?;
        let state = State { tx };

        let line = Self::try_task("console-service-web", async move {
            info!("Starting server...");
            let mut app = tide::with_state(state);
            app.at("/affirm")
                .post(|mut req: Request<State>| async move {
                    let a: Affirmation = req.body_json().await?;

                    let mut tx2 = &mut req.state().tx.clone();
                    tx2.send(DarwiniaAffirmMessage::ToAffirm(a.block_number))
                        .await?;

                    Ok(json!({
                        "meta": { "xxx": 2 },
                    }))
                });
            app.listen("0.0.0.0:3000").await?;

            Ok(())
        });

        Ok(Self { line })
    }
}
