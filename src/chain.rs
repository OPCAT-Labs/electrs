use std::str::FromStr;

#[cfg(not(feature = "opcat_layer"))] // use regular Bitcoin data structures
pub use bitcoin::{
    blockdata::{opcodes, script, witness::Witness},
    consensus::deserialize,
    hashes,
    util::address,
    Block, BlockHash, BlockHeader, OutPoint, Script, Transaction, TxIn, TxOut, Txid,
};

#[cfg(feature = "opcat_layer")]
pub use crate::opcat_layer::{
    address,
    blockdata::{opcodes, script},
    hashes, Address, Block, BlockHash, BlockHeader, OutPoint, Script, Transaction, TxIn, TxOut,
    Txid,
};

use bitcoin::blockdata::constants::genesis_block;
pub use bitcoin::network::constants::Network as BNetwork;

#[cfg(not(feature = "opcat_layer"))]
pub type Value = u64;

#[cfg(feature = "opcat_layer")]
pub use crate::opcat_layer::Amount as Value;

#[derive(Debug, Copy, Clone, PartialEq, Hash, Serialize, Ord, PartialOrd, Eq)]
pub enum Network {
    #[cfg(not(feature = "opcat_layer"))]
    Bitcoin,
    #[cfg(not(feature = "opcat_layer"))]
    Testnet,
    #[cfg(not(feature = "opcat_layer"))]
    Testnet4,
    #[cfg(not(feature = "opcat_layer"))]
    Regtest,
    #[cfg(not(feature = "opcat_layer"))]
    Signet,

    #[cfg(feature = "opcat_layer")]
    OpcatLayerMainnet,
    #[cfg(feature = "opcat_layer")]
    OpcatLayerTestnet,
    #[cfg(feature = "opcat_layer")]
    OpcatLayerRegtest,
}

#[cfg(feature = "opcat_layer")]
pub use crate::opcat_layer::address::{
    OPCAT_MAINNET_PARAMS, OPCAT_REGTEST_PARAMS, OPCAT_TESTNET_PARAMS,
};

/// Magic for testnet4, 0x1c163f28 (from BIP94) with flipped endianness.
#[cfg(not(feature = "opcat_layer"))]
const TESTNET4_MAGIC: u32 = 0x283f161c;

impl Network {
    #[cfg(not(feature = "opcat_layer"))]
    pub fn magic(self) -> u32 {
        match self {
            Self::Testnet4 => TESTNET4_MAGIC,
            _ => BNetwork::from(self).magic(),
        }
    }

    #[cfg(feature = "opcat_layer")]
    pub fn magic(self) -> u32 {
        match self {
            Network::OpcatLayerMainnet => 0xF9BE_B4D9, // Same as Bitcoin for now
            Network::OpcatLayerTestnet => 0x0709_110B, // Same as Bitcoin testnet for now
            Network::OpcatLayerRegtest => 0xFABF_B5DA, // Same as Bitcoin regtest for now
        }
    }

    pub fn is_regtest(self) -> bool {
        match self {
            #[cfg(not(feature = "opcat_layer"))]
            Network::Regtest => true,
            #[cfg(feature = "opcat_layer")]
            Network::OpcatLayerRegtest => true,
            _ => false,
        }
    }

    #[cfg(feature = "opcat_layer")]
    pub fn address_params(self) -> &'static crate::opcat_layer::address::OpcatAddressParams {
        match self {
            Network::OpcatLayerMainnet => &OPCAT_MAINNET_PARAMS,
            Network::OpcatLayerTestnet => &OPCAT_TESTNET_PARAMS,
            Network::OpcatLayerRegtest => &OPCAT_REGTEST_PARAMS,
        }
    }

    pub fn names() -> Vec<String> {
        #[cfg(not(feature = "opcat_layer"))]
        return vec![
            "mainnet".to_string(),
            "testnet".to_string(),
            "regtest".to_string(),
            "signet".to_string(),
        ];

        #[cfg(feature = "opcat_layer")]
        return vec![
            "opcat".to_string(),
            "opcattestnet".to_string(),
            "opcatregtest".to_string(),
        ];
    }
}

pub fn genesis_hash(network: Network) -> BlockHash {
    #[cfg(not(feature = "opcat_layer"))]
    return bitcoin_genesis_hash(network);
    #[cfg(feature = "opcat_layer")]
    return opcat_genesis_hash(network);
}

pub fn bitcoin_genesis_hash(network: Network) -> bitcoin::BlockHash {
    lazy_static! {
        static ref BITCOIN_GENESIS: bitcoin::BlockHash =
            genesis_block(BNetwork::Bitcoin).block_hash();
        static ref TESTNET_GENESIS: bitcoin::BlockHash =
            genesis_block(BNetwork::Testnet).block_hash();
        static ref TESTNET4_GENESIS: bitcoin::BlockHash = bitcoin::BlockHash::from_str(
            "00000000da84f2bafbbc53dee25a72ae507ff4914b867c565be350b0da8bf043"
        )
        .unwrap();
        static ref REGTEST_GENESIS: bitcoin::BlockHash =
            genesis_block(BNetwork::Regtest).block_hash();
        static ref SIGNET_GENESIS: bitcoin::BlockHash =
            genesis_block(BNetwork::Signet).block_hash();
    }
    #[cfg(not(feature = "opcat_layer"))]
    match network {
        Network::Bitcoin => *BITCOIN_GENESIS,
        Network::Testnet => *TESTNET_GENESIS,
        Network::Testnet4 => *TESTNET4_GENESIS,
        Network::Regtest => *REGTEST_GENESIS,
        Network::Signet => *SIGNET_GENESIS,
    }
    #[cfg(feature = "opcat_layer")]
    match network {
        Network::OpcatLayerMainnet => *BITCOIN_GENESIS, // Use Bitcoin genesis for now
        Network::OpcatLayerTestnet => *TESTNET_GENESIS, // Use Bitcoin testnet genesis for now
        Network::OpcatLayerRegtest => *REGTEST_GENESIS, // Use Bitcoin regtest genesis for now
    }
}

#[cfg(feature = "opcat_layer")]
pub fn opcat_genesis_hash(network: Network) -> crate::opcat_layer::BlockHash {
    // For now, use the same genesis blocks as Bitcoin
    // These can be replaced with actual OPCAT Layer genesis blocks later
    let bitcoin_hash = bitcoin_genesis_hash(network);

    // Convert Bitcoin BlockHash to OPCAT Layer BlockHash
    // This assumes they have the same internal representation
    crate::opcat_layer::BlockHash::from_hash(bitcoin_hash.as_hash())
}

impl From<&str> for Network {
    fn from(network_name: &str) -> Self {
        match network_name {
            #[cfg(not(feature = "opcat_layer"))]
            "mainnet" => Network::Bitcoin,
            #[cfg(not(feature = "opcat_layer"))]
            "testnet" => Network::Testnet,
            #[cfg(not(feature = "opcat_layer"))]
            "testnet4" => Network::Testnet4,
            #[cfg(not(feature = "opcat_layer"))]
            "regtest" => Network::Regtest,
            #[cfg(not(feature = "opcat_layer"))]
            "signet" => Network::Signet,

            #[cfg(feature = "opcat_layer")]
            "opcat" => Network::OpcatLayerMainnet,
            #[cfg(feature = "opcat_layer")]
            "opcattestnet" => Network::OpcatLayerTestnet,
            #[cfg(feature = "opcat_layer")]
            "opcatregtest" => Network::OpcatLayerRegtest,

            _ => panic!("unsupported network: {:?}", network_name),
        }
    }
}

#[cfg(not(feature = "opcat_layer"))]
impl From<Network> for BNetwork {
    fn from(network: Network) -> Self {
        match network {
            Network::Bitcoin => BNetwork::Bitcoin,
            Network::Testnet => BNetwork::Testnet,
            Network::Testnet4 => BNetwork::Testnet,
            Network::Regtest => BNetwork::Regtest,
            Network::Signet => BNetwork::Signet,
        }
    }
}

#[cfg(not(feature = "opcat_layer"))]
impl From<BNetwork> for Network {
    fn from(network: BNetwork) -> Self {
        match network {
            BNetwork::Bitcoin => Network::Bitcoin,
            BNetwork::Testnet => Network::Testnet,
            BNetwork::Regtest => Network::Regtest,
            BNetwork::Signet => Network::Signet,
        }
    }
}

#[cfg(feature = "opcat_layer")]
impl From<Network> for BNetwork {
    fn from(network: Network) -> Self {
        match network {
            Network::OpcatLayerMainnet => BNetwork::Bitcoin,
            Network::OpcatLayerTestnet => BNetwork::Testnet,
            Network::OpcatLayerRegtest => BNetwork::Regtest,
        }
    }
}

#[cfg(feature = "opcat_layer")]
impl From<BNetwork> for Network {
    fn from(network: BNetwork) -> Self {
        match network {
            BNetwork::Bitcoin => Network::OpcatLayerMainnet,
            BNetwork::Testnet => Network::OpcatLayerTestnet,
            BNetwork::Regtest => Network::OpcatLayerRegtest,
            BNetwork::Signet => Network::OpcatLayerTestnet, // Map signet to testnet for now
        }
    }
}
