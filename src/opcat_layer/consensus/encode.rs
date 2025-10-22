// Re-export specific utilities we need
pub use bitcoin::consensus::encode::{
    Decodable, Encodable,
    serialize, deserialize,
    deserialize_partial,
    serialize_hex, 
    ReadExt, WriteExt,
    Error, 
};

/// Maximum size, in bytes, of a vector we are allowed to decode, related to the block size limit.
pub const MAX_VEC_SIZE: usize = 2 * 32_000_000; // 2x the current max block size
