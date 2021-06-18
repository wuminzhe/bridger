use serde::{Deserialize, Serialize};

/// Ethereum Settings
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Ethereum {
    /// Ethereum rpc url
    pub rpc: String,
    /// Ethereum contracts
    pub contracts: Vec<EthereumContract>,
}

/// Ethereum Contract Tuple
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EthereumContract {
    /// Contract Name
    pub name: String,
    /// Contract Address
    pub address: String,
    /// Contract Topic
    pub topics: Option<Vec<String>>,
}

impl Default for Ethereum {
    fn default() -> Self {
        Ethereum {
            rpc: "https://ropsten.infura.io/v3/60703fcc6b4e48079cfc5e385ee7af80".to_string(),
            contracts: vec![
                EthereumContract {
                    name: "ring".to_string(),
                    address: "0xb52FBE2B925ab79a821b261C82c5Ba0814AAA5e0".to_string(),
                    topics: Some(vec![
                        "0xc9dcda609937876978d7e0aa29857cb187aea06ad9e843fd23fd32108da73f10".to_string()
                    ])
                },
                EthereumContract {
                    name: "kton".to_string(),
                    address: "0x1994100c58753793D52c6f457f189aa3ce9cEe94".to_string(),
                    topics: Some(vec![
                        "0xc9dcda609937876978d7e0aa29857cb187aea06ad9e843fd23fd32108da73f10".to_string()
                    ])
                },
                EthereumContract {
                    name: "bank".to_string(),
                    address: "0x6EF538314829EfA8386Fc43386cB13B4e0A67D1e".to_string(),
                    topics: Some(vec![
                        "0xe77bf2fa8a25e63c1e5e29e1b2fcb6586d673931e020c4e3ffede453b830fb12".to_string()
                    ])
                },
                EthereumContract {
                    name: "issuing".to_string(),
                    address: "0x49262B932E439271d05634c32978294C7Ea15d0C".to_string(),
                    topics: None
                },
                EthereumContract {
                    name: "relay".to_string(),
                    address: "0xd374292D512281b56198F0401b149370D680b89F".to_string(),
                    topics: Some(vec![
                        "0x91d6d149c7e5354d1c671fe15a5a3332c47a38e15e8ac0339b24af3c1090690f".to_string()
                    ])
                }
            ] ,
        }
    }
}
