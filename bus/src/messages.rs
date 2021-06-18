use lifeline::prelude::*;
use crate::BridgerBus;
use postage::broadcast;

use substrate_subxt::{
    sp_runtime::generic::Header,
	sp_runtime::traits::BlakeTwo256,
};

/// BridgerMessage
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum BridgerMessage {
    Darwinia(String),
    DarwiniaHeader(Header<u32, BlakeTwo256>),
}
impl Message<BridgerBus> for BridgerMessage {
    type Channel = broadcast::Sender<Self>;
}

/// DarwiniaAffirmMessage
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum DarwiniaAffirmMessage {
    LastAffirmed(u64),
    ToAffirm(u64),
    DoAffirm,
}
impl Message<BridgerBus> for DarwiniaAffirmMessage {
    type Channel = broadcast::Sender<Self>;
}
