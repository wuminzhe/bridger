#[macro_use]
extern crate async_trait;
#[macro_use]
extern crate log;

mod error;
mod traits;
mod ethlike_client;
mod chains;
mod default_logs_handler;
mod ethlike_tracker;

pub use error::Error;
pub type Result<T> = std::result::Result<T, Error>;

pub use chains::{
    Ethereum, Bsc, Heco
};
pub use default_logs_handler::DefaultLogsHandler;
pub use traits::{EthlikeChain, LogsHandler};
pub use ethlike_client::EthlikeClient;