#[derive(Copy, PartialEq, Eq, PartialOrd, Ord, Clone, Hash, Debug)]
pub enum Network {
    /// Classic Bitcoin
    Mainnet,
    Testnet,
    Regtest,
}
