#[derive(Copy, PartialEq, Eq, PartialOrd, Ord, Clone, Hash, Debug)]
pub enum Network {
    /// Classic Bitcoin
    Mainnet,
    Testnet,
    Regtest,
}

pub const FEE_RATE: f64 = 0.0000001; // const fee rate for opcat layer network, in BTC/kB
