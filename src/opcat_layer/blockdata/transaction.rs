use std::io;

use crate::opcat_layer::{
    blockdata::units::Amount,
    consensus::encode::{serialize, Decodable, Encodable, Error, MAX_VEC_SIZE},
};

use bitcoin::{
    hashes::{sha256, Hash},
    VarInt,
};
pub use bitcoin::{OutPoint, Txid};

// OPCAT Layer transaction structure
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Transaction {
    pub version: i32,
    pub lock_time: u32,
    pub input: Vec<TxIn>,
    pub output: Vec<TxOut>,
    // OPCAT Layer specific fields can be added here
    // pub opcat_data: Option<OpcatData>,
}

// Implement required methods for Transaction
impl Transaction {
    pub fn txid(&self) -> Txid {
        let mut enc = Txid::engine();
        self.version
            .consensus_encode(&mut enc)
            .expect("engines don't error");
        VarInt(self.input.len() as u64)
            .consensus_encode(&mut enc)
            .expect("engines don't error");
        for input in &self.input {
            // prevout
            input
                .previous_output
                .consensus_encode(&mut enc)
                .expect("engines don't error");
            // sha256(unlcocking Script)
            sha256::Hash::hash(input.script_sig.as_ref())
                .consensus_encode(&mut enc)
                .expect("engines don't error");
            // sequence
            input
                .sequence
                .consensus_encode(&mut enc)
                .expect("engines don't error");
        }
        VarInt(self.output.len() as u64)
            .consensus_encode(&mut enc)
            .expect("engines don't error");
        for output in &self.output {
            // value
            output
                .value
                .consensus_encode(&mut enc)
                .expect("engines don't error");
            // script hash
            sha256::Hash::hash(output.script_pubkey.as_ref())
                .consensus_encode(&mut enc)
                .expect("engines don't error");
            // data hash
            sha256::Hash::hash(output.data.as_ref())
                .consensus_encode(&mut enc)
                .expect("engines don't error");
        }
        self.lock_time
            .consensus_encode(&mut enc)
            .expect("engines don't error");
        Txid::from_engine(enc)
    }

    pub fn is_coin_base(&self) -> bool {
        self.input.len() == 1 && self.input[0].previous_output.is_null()
    }

    pub fn weight(&self) -> usize {
        self.size()
    }

    fn get_base_size(&self) -> usize {
        // Calculate base transaction size (without witness data)
        // This is a simplified calculation
        4 + // version
        bitcoin::VarInt(self.input.len() as u64).len() +
        self.input.iter().map(|i| 36 + bitcoin::VarInt(i.script_sig.len() as u64).len() + i.script_sig.len() + 4).sum::<usize>() +
        bitcoin::VarInt(self.output.len() as u64).len() +
        self.output.iter().map(|o| 8 + bitcoin::VarInt(o.script_pubkey.len() as u64).len() + o.script_pubkey.len()).sum::<usize>() +
        4 // lock_time
    }

    // fn get_total_size(&self) -> usize {
    //     // Calculate total transaction size (including witness data)
    //     // This is a simplified calculation
    //     self.get_base_size() +
    //     self.input.iter().map(|i| i.witness.serialized_len()).sum::<usize>()
    // }

    pub fn strippedsize(&self) -> usize {
        self.get_base_size()
    }

    pub fn size(&self) -> usize {
        serialize(self).len()
    }
}

impl Encodable for Transaction {
    fn consensus_encode<S: io::Write>(&self, mut s: S) -> Result<usize, io::Error> {
        let mut len = 0;
        len += self.version.consensus_encode(&mut s)?;
        // To avoid serialization ambiguity, no inputs means we use BIP141 serialization (see
        // `Transaction` docs for full explanation).
        len += bitcoin::VarInt(self.input.len() as u64).consensus_encode(&mut s)?;
        for input in &self.input {
            len += input.consensus_encode(&mut s)?;
        }
        len += bitcoin::VarInt(self.output.len() as u64).consensus_encode(&mut s)?;
        for output in &self.output {
            len += output.consensus_encode(&mut s)?;
        }
        len += self.lock_time.consensus_encode(s)?;
        Ok(len)
    }
}

impl Decodable for Transaction {
    fn consensus_decode<D: io::Read>(d: D) -> Result<Self, Error> {
        let mut d = d.take(MAX_VEC_SIZE as u64);
        let version = i32::consensus_decode(&mut d)?;

        let input_len = bitcoin::VarInt::consensus_decode(&mut d)?.0 as usize;
        let mut input = Vec::with_capacity(input_len);
        for _ in 0..input_len {
            input.push(TxIn::consensus_decode(&mut d)?);
        }

        let output_len = bitcoin::VarInt::consensus_decode(&mut d)?.0 as usize;
        let mut output = Vec::with_capacity(output_len);
        for _ in 0..output_len {
            output.push(TxOut::consensus_decode(&mut d)?);
        }

        Ok(Transaction {
            version,
            input,
            output,
            lock_time: Decodable::consensus_decode(d)?,
        })
    }
}

// OPCAT Layer transaction output
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TxOut {
    pub value: Amount,
    pub script_pubkey: bitcoin::Script,
    // OPCAT Layer specific output field to store additional data
    pub data: Vec<u8>,
}

// OPCAT Layer transaction input
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TxIn {
    pub previous_output: bitcoin::OutPoint,
    pub script_sig: bitcoin::Script,
    pub sequence: u32,
    // OPCAT Layer specific input fields can be added here
}

impl TxIn {
    pub fn is_coinbase(&self) -> bool {
        self.previous_output.is_null()
    }
}

impl Encodable for TxIn {
    fn consensus_encode<W: io::Write>(&self, mut writer: W) -> Result<usize, io::Error> {
        let mut len = 0;
        len += self.previous_output.consensus_encode(&mut writer)?;
        len += self.script_sig.consensus_encode(&mut writer)?;
        len += self.sequence.consensus_encode(&mut writer)?;
        // Note: witness is encoded separately in Bitcoin format
        Ok(len)
    }
}

impl Decodable for TxIn {
    fn consensus_decode<R: io::Read>(mut reader: R) -> Result<Self, Error> {
        Ok(TxIn {
            previous_output: Decodable::consensus_decode(&mut reader)?,
            script_sig: Decodable::consensus_decode(&mut reader)?,
            sequence: Decodable::consensus_decode(&mut reader)?,
        })
    }
}

impl Encodable for TxOut {
    fn consensus_encode<W: io::Write>(&self, mut writer: W) -> Result<usize, io::Error> {
        let mut len = 0;
        len += self.value.consensus_encode(&mut writer)?;
        len += self.script_pubkey.consensus_encode(&mut writer)?;
        len += self.data.consensus_encode(&mut writer)?;
        Ok(len)
    }
}

impl Decodable for TxOut {
    fn consensus_decode<R: io::Read>(mut reader: R) -> Result<Self, Error> {
        Ok(TxOut {
            value: Decodable::consensus_decode(&mut reader)?,
            script_pubkey: Decodable::consensus_decode(&mut reader)?,
            data: Decodable::consensus_decode(&mut reader)?,
        })
    }
}

// TODO: add tests
