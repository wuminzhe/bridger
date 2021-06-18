mod messages;

pub use messages::{BridgerMessage, DarwiniaAffirmMessage};

use lifeline::prelude::*;

// This is a macro that generates an BridgerBus struct,
//   and implements DynBus for it.
// DynBus stores the channels in Box<dyn Any> slots,
//  and deals with all the dyn trait magic for us.
lifeline_bus!(pub struct BridgerBus);
