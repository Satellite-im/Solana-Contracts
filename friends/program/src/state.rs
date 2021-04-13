//! State transition types

use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;

/// Friend info
#[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug, Clone)]
pub struct FriendInfo {
    /// User key
    pub user: Pubkey,
    /// Count of incoming friend requests
    pub requests_incoming: u64,
    /// Count of outgoing friend requests
    pub requests_outgoing: u64,
    /// Count of friends
    pub friends: u64,
}

/// Friend request
#[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug, Clone)]
pub struct Request {
    /// From key
    pub from: Pubkey,
    /// To key
    pub to: Pubkey,
}

/// Friend
#[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug, Clone)]
pub struct Friend {
    /// User key
    pub user: Pubkey,
    /// Friend key
    pub friend: Pubkey,
    /// Conversation thread hash 1
    pub thread_id1: [u8; 32],
    /// Conversation thread hash 2
    pub thread_id2: [u8; 32],
}
