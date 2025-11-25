use std::io;

use bitcoin::{
    consensus::{Decodable, Encodable},
    util::hash::bitcoin_merkle_root,
    TxMerkleNode, VarInt,
};

use crate::opcat_layer::blockdata::transaction::Transaction;
use crate::opcat_layer::consensus::encode::{Error, MAX_VEC_SIZE};

pub type BlockHeader = bitcoin::BlockHeader;
pub type BlockHash = bitcoin::BlockHash;

// OPCAT Layer block structure
#[derive(PartialEq, Eq, Clone, Debug)]
// #[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Block {
    pub header: BlockHeader,
    pub txdata: Vec<Transaction>,
}

impl Block {
    /// Returns the block hash.
    pub fn block_hash(&self) -> BlockHash {
        self.header.block_hash()
    }

    /// check if merkle root of header matches merkle root of the transaction list
    pub fn check_merkle_root(&self) -> bool {
        match self.compute_merkle_root() {
            Some(merkle_root) => self.header.merkle_root == merkle_root,
            None => false,
        }
    }

    /// Computes the transaction merkle root.
    pub fn compute_merkle_root(&self) -> Option<TxMerkleNode> {
        let hashes = self.txdata.iter().map(|obj| obj.txid().as_hash());
        bitcoin_merkle_root(hashes).map(|h| h.into())
    }

    /// Calculate the transaction merkle root.
    #[deprecated(
        since = "0.28.0",
        note = "Please use `block::compute_merkle_root` instead."
    )]
    pub fn merkle_root(&self) -> Option<TxMerkleNode> {
        self.compute_merkle_root()
    }

    /// base_size == size of header + size of encoded transaction count.
    fn base_size(&self) -> usize {
        80 + VarInt(self.txdata.len() as u64).len()
    }

    /// Returns the size of the block.
    #[deprecated(since = "0.28.0", note = "Please use `block::size` instead.")]
    pub fn get_size(&self) -> usize {
        self.size()
    }

    /// Returns the size of the block.
    ///
    /// size == size of header + size of encoded transaction count + total size of transactions.
    pub fn size(&self) -> usize {
        let txs_size: usize = self.txdata.iter().map(Transaction::size).sum();
        self.base_size() + txs_size
    }

    /// Returns the strippedsize of the block.
    #[deprecated(
        since = "0.28.0",
        note = "Please use `transaction::strippedsize` instead."
    )]
    pub fn get_strippedsize(&self) -> usize {
        self.strippedsize()
    }

    /// Returns the strippedsize of the block.
    pub fn strippedsize(&self) -> usize {
        let txs_size: usize = self.txdata.iter().map(Transaction::strippedsize).sum();
        self.base_size() + txs_size
    }

    /// Returns the weight of the block.
    #[deprecated(since = "0.28.0", note = "Please use `transaction::weight` instead.")]
    pub fn get_weight(&self) -> usize {
        self.weight()
    }

    /// Returns the weight of the block.
    pub fn weight(&self) -> usize {
        let base_weight = self.base_size();
        let txs_weight: usize = self.txdata.iter().map(Transaction::weight).sum();
        base_weight + txs_weight
    }

    /// Returns the coinbase transaction, if one is present.
    pub fn coinbase(&self) -> Option<&Transaction> {
        self.txdata.first()
    }
}

impl Decodable for Block {
    fn consensus_decode<D: io::Read>(d: D) -> Result<Self, Error> {
        let mut d = d.take(MAX_VEC_SIZE as u64);
        let header = BlockHeader::consensus_decode(&mut d)?;
        let txdata_len = VarInt::consensus_decode(&mut d)?.0 as usize;
        let mut txdata = Vec::with_capacity(txdata_len);
        for _ in 0..txdata_len {
            txdata.push(Transaction::consensus_decode(&mut d)?);
        }
        Ok(Block { header, txdata })
    }
}

impl Encodable for Block {
    fn consensus_encode<S: io::Write>(&self, mut s: S) -> Result<usize, io::Error> {
        let mut len = 0;
        len += self.header.consensus_encode(&mut s)?;
        len += VarInt(self.txdata.len() as u64).consensus_encode(&mut s)?;
        for tx in &self.txdata {
            len += tx.consensus_encode(&mut s)?;
        }
        Ok(len)
    }
}
