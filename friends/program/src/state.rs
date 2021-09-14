//! State transition types

use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;

// /// Friend info
// #[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug, Clone, Default)]
// pub struct FriendInfo {
//     /// Count of incoming friend requests
//     pub requests_incoming: u64,
//     /// Count of outgoing friend requests
//     pub requests_outgoing: u64,
//     /// Count of friends
//     pub friends: u64,
//     /// User key
//     pub user: Pubkey,
// }

// /// Friend request
// #[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug, Clone, Default)]
// pub struct Request {
//     /// Encrypted key 1
//     pub encrypted_key1: [u8; 32],
//     /// Encrypted key 2
//     pub encrypted_key2: [u8; 32],
//     /// From key
//     pub from: Pubkey,
//     /// To key
//     pub to: Pubkey,
// }

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

// impl FriendInfo {
//     /// Data len
//     pub const LEN: usize = 56;

//     /// Check if struct is initialized
//     pub fn is_initialized(&self) -> bool {
//         *self != FriendInfo::default()
//     }
// }

// impl Request {
//     /// Data len
//     pub const LEN: usize = 128;

//     /// Check if struct is initialized
//     pub fn is_initialized(&self) -> bool {
//         *self != Request::default()
//     }
// }

impl Friend {
    /// Data len
    pub const LEN: usize = 321;

    /// Check if struct is initialized
    pub fn is_initialized(&self) -> bool {
        *self != Friend::default()
    }
}
