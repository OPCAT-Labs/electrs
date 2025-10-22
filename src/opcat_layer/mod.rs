// OPCAT Layer integration module
pub mod consensus;
pub mod address;
pub mod network;
pub mod blockdata;

pub use network::constants::{Network, FEE_RATE};
pub use address::Address;
pub use blockdata::block::{Block, BlockHeader, BlockHash};
pub use blockdata::transaction::{Transaction, TxIn, TxOut, OutPoint, Txid};
pub use blockdata::units::{Amount};

pub use self::blockdata::script::{Script, Instruction, Error as ScriptError};
pub use self::consensus::encode::{deserialize, serialize};

// Re-exporting bitcoin hashes for compatibility
pub use bitcoin::{hashes};
