use std::io;

use bitcoin::consensus::{Decodable, Encodable};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Amount(pub u64);

impl Amount {
    pub fn as_sat(&self) -> u64 {
        self.0
    }
    
    pub fn from_sat(satoshis: u64) -> Self {
        Amount(satoshis)
    }
}

impl std::ops::Add for Amount {
    type Output = Amount;
    
    fn add(self, other: Amount) -> Amount {
        Amount(self.0 + other.0)
    }
}

impl std::iter::Sum for Amount {
    fn sum<I: Iterator<Item = Amount>>(iter: I) -> Amount {
        iter.fold(Amount(0), |acc, val| acc + val)
    }
}

impl std::iter::Sum<Amount> for u64 {
    fn sum<I: Iterator<Item = Amount>>(iter: I) -> u64 {
        iter.map(|v| v.0).sum()
    }
}

impl std::ops::AddAssign<Amount> for u64 {
    fn add_assign(&mut self, rhs: Amount) {
        *self += rhs.0;
    }
}

impl From<u64> for Amount {
    fn from(satoshis: u64) -> Self {
        Amount(satoshis)
    }
}

impl Into<u64> for Amount {
    fn into(self) -> u64 {
        self.0
    }
}

impl Into<i64> for Amount {
    fn into(self) -> i64 {
        self.0 as i64
    }
}

impl Encodable for Amount {
    fn consensus_encode<W: io::Write>(&self, writer: W) -> Result<usize, io::Error> {
        self.0.consensus_encode(writer)
    }
}

impl Decodable for Amount {
    fn consensus_decode<R: io::Read>(reader: R) -> Result<Self, bitcoin::consensus::encode::Error> {
        Ok(Amount(u64::consensus_decode(reader)?))
    }
}