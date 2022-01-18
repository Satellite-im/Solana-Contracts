//! State transition types

use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;

/// Friend
#[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug, Clone, Default)]
pub struct Friend {
    /// User key
    pub from: Pubkey,
    /// Request status
    pub status: u8,
    /// Friend key
    pub to: Pubkey,
    /// Textile user encrypted key
    pub from_encrypted_key1: [u8; 32],
    /// Textile user encrypted key
    pub from_encrypted_key2: [u8; 32],
    /// Textile user encrypted key
    pub from_encrypted_key3: [u8; 32],
    /// Textile user encrypted key
    pub from_encrypted_key4: [u8; 32],
    /// Textile friend encrypted key
    pub to_encrypted_key1: [u8; 32],
    /// Textile friend encrypted key
    pub to_encrypted_key2: [u8; 32],
    /// Textile friend encrypted key
    pub to_encrypted_key3: [u8; 32],
    /// Textile friend encrypted key
    pub to_encrypted_key4: [u8; 32],
}

impl Friend {
    /// Data len
    pub const LEN: usize = 321;

    /// Check if struct is initialized
    pub fn is_initialized(&self) -> bool {
        *self != Friend::default()
    }
}
