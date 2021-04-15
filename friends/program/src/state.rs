//! State transition types

use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;

/// Friend info
#[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug, Clone, Default)]
pub struct FriendInfo {
    /// Count of incoming friend requests
    pub requests_incoming: u64,
    /// Count of outgoing friend requests
    pub requests_outgoing: u64,
    /// Count of friends
    pub friends: u64,
    /// User key
    pub user: Pubkey,
}

/// Friend request
#[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug, Clone, Default)]
pub struct Request {
    /// From key
    pub from: Pubkey,
    /// To key
    pub to: Pubkey,
}

/// Friend
#[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug, Clone, Default)]
pub struct Friend {
    /// Conversation thread hash 1
    pub thread_id1: [u8; 32],
    /// Conversation thread hash 2
    pub thread_id2: [u8; 32],
    /// User key
    pub user: Pubkey,
    /// Friend key
    pub friend: Pubkey,
}

impl FriendInfo {
    /// Data len
    pub const LEN: usize = 56;

    /// Check if struct is initialized
    pub fn is_initialized(&self) -> bool {
        *self != FriendInfo::default()
    }
}

impl Request {
    /// Data len
    pub const LEN: usize = 64;

    /// Check if struct is initialized
    pub fn is_initialized(&self) -> bool {
        *self != Request::default()
    }
}

impl Friend {
    /// Data len
    pub const LEN: usize = 128;

    /// Check if struct is initialized
    pub fn is_initialized(&self) -> bool {
        *self != Friend::default()
    }
}
