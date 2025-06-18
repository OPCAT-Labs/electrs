// Re-export specific utilities we need
pub use bitcoin::consensus::encode::{
    Decodable, Encodable,
    serialize, deserialize,
    deserialize_partial,
    serialize_hex, 
    ReadExt, WriteExt,
    Error, 
    MAX_VEC_SIZE,
};
