// OPCAT Layer script operations and utilities

// Re-export everything from bitcoin script module so it can be used as script::Type
pub use bitcoin::blockdata::script::*;

// OPCAT Layer specific script operations can be added here
// For example:
//
// /// OPCAT Layer specific opcodes
// pub mod opcodes {
//     pub use bitcoin::blockdata::opcodes::*;
//
//     // Custom OPCAT opcodes
//     pub const OP_CAT: u8 = 0x7e;  // Concatenate two byte arrays
//     pub const OP_OPCAT_VERIFY: u8 = 0xba;  // Custom OPCAT verification
// }
//
// /// Check if script contains OPCAT operations
// pub fn script_has_opcat_ops(script: &Script) -> bool {
//     script.as_bytes().contains(&opcodes::OP_CAT)
// }
//
// /// Validate OPCAT script execution
// pub fn validate_opcat_script(script: &Script, witness: &bitcoin::Witness) -> Result<bool, ScriptError> {
//     // Custom script validation logic for OPCAT operations
// }

// Script utilities that might be useful for OPCAT Layer

/// Extract script public key hash if this is a P2PKH script
pub fn extract_p2pkh_hash(script: &Script) -> Option<[u8; 20]> {
    if script.is_p2pkh() {
        // Extract the 20-byte hash from P2PKH script
        if script.len() == 25 {
            let mut hash = [0u8; 20];
            hash.copy_from_slice(&script.as_bytes()[3..23]);
            Some(hash)
        } else {
            None
        }
    } else {
        None
    }
}

/// Extract script hash if this is a P2SH script
pub fn extract_p2sh_hash(script: &Script) -> Option<[u8; 20]> {
    if script.is_p2sh() {
        // Extract the 20-byte hash from P2SH script
        if script.len() == 23 {
            let mut hash = [0u8; 20];
            hash.copy_from_slice(&script.as_bytes()[2..22]);
            Some(hash)
        } else {
            None
        }
    } else {
        None
    }
}

/// Check if script is spendable (not OP_RETURN or similar)
pub fn is_spendable_script(script: &Script) -> bool {
    // Basic spendability check - can be customized for OPCAT Layer
    !script.is_op_return() && !script.is_provably_unspendable()
}
