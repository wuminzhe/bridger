
use bus::{
	BridgerBus, 
	BridgerMessage, DarwiniaAffirmMessage
};

use lifeline::prelude::*;
use postage::{sink::Sink, stream::Stream};
use simple_logger::SimpleLogger;

use tokio::time::{sleep, Duration};

use darwinia_relayer_service::DarwiniaRelayerService;
use darwinia_events_service::DarwiniaEventsService;
use darwinia_affirm_service::DarwiniaAffirmService;
use console_service::ConsoleService;

/// Spawn a simple bus, and a services
/// The services execution is tied to the 'lifeline' it returns
/// If `services` is dropped, all it's tasks are cancelled.
#[tokio::main]
pub async fn main() -> anyhow::Result<()> {
    SimpleLogger::new().with_level(log::LevelFilter::Info).init().expect("log init failed");

    let bus = BridgerBus::default();

    // let _service1 = ExampleService::spawn(&bus)?;
    // let _service2 = MonitoringService::spawn(&bus)?;
    let _service3 = DarwiniaRelayerService::spawn(&bus)?;
    let _service4 = DarwiniaEventsService::spawn(&bus)?;
    let _service5 = DarwiniaAffirmService::spawn(&bus)?;
    let _service6 = ConsoleService::spawn(&bus)?;

    let mut rx = bus.rx::<BridgerMessage>()?;
    let mut tx = bus.tx::<BridgerMessage>()?;
    let mut tx2 = bus.tx::<DarwiniaAffirmMessage>()?;

    drop(bus);

    tx.send(BridgerMessage::Darwinia("fuck".to_string())).await?;
    tx2.send(DarwiniaAffirmMessage::LastAffirmed(100)).await?;
    tx2.send(DarwiniaAffirmMessage::ToAffirm(150)).await?;
    tx2.send(DarwiniaAffirmMessage::DoAffirm).await?;

    // let oh_hello = rx.recv().await;
    // assert_eq!(Some(ExampleSend::OhHello), oh_hello);
    // println!("Service says {:?}", oh_hello.unwrap());

    // let aww_ok = rx.recv().await;
    // assert_eq!(Some(ExampleSend::AwwOk), aww_ok);
    // println!("Service says {:?}", aww_ok.unwrap());

	sleep(Duration::from_secs(100)).await;
    println!("All done.");

    Ok(())
}
