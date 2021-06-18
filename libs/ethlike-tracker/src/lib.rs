#[macro_use]
extern crate async_trait;
#[macro_use]
extern crate log;

mod chains;
mod default_logs_handler;
mod error;
mod ethlike_client;
mod ethlike_tracker;
mod traits;

pub use error::Error;
pub type Result<T> = std::result::Result<T, Error>;

pub use chains::{Bsc, Ethereum, Heco};
pub use default_logs_handler::DefaultLogsHandler;
pub use ethlike_client::EthlikeClient;
pub use ethlike_tracker::EthlikeTracker;
pub use traits::{EthlikeChain, LogsHandler};
