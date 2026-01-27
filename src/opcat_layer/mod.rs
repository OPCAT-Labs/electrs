// OPCAT Layer integration module
pub mod address;
pub mod blockdata;
pub mod consensus;
pub mod network;

pub use address::Address;
pub use blockdata::block::{Block, BlockHash, BlockHeader};
pub use blockdata::transaction::{OutPoint, Transaction, TxIn, TxOut, Txid};
pub use blockdata::units::Amount;
pub use network::constants::Network;

pub use self::blockdata::script::{Error as ScriptError, Instruction, Script};
pub use self::consensus::encode::{deserialize, serialize};

// Re-exporting bitcoin hashes for compatibility
pub use bitcoin::hashes;
