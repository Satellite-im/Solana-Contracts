//! State transition types
use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;

/// Sticker factory account
#[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug, Clone, Default)]
pub struct StickerFactory {
    /// Artist count
    pub artist_count: u64,
    /// Sticker count
    pub sticker_count: u64,
    /// Owner
    pub owner: Pubkey,
}

/// Artist account
#[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug, Clone)]
pub struct Artist {
    /// User
    pub user: Pubkey,
    /// User token account
    pub user_token_acc: Pubkey,
    /// Name
    pub name: [u8; 32],
    /// Signature
    pub signature: [u8; 256],
    /// Description
    pub description: [u8; 256],
}

/// Sticker account
#[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug, Clone)]
pub struct Sticker {
    /// Creator
    pub creator: Pubkey,
    /// Supply
    pub supply: u64,
    /// Max supply
    pub max_supply: u64,
    /// Price
    pub price: u64,
    /// Mint
    pub mint: Pubkey,
    /// URI
    pub uri: [u8; 256],
}

// Implement Default by hands because of big arrays
impl Default for Artist {
    fn default() -> Artist {
        unsafe { std::mem::zeroed() }
    }
}

// Implement Default by hands because of big arrays
impl Default for Sticker {
    fn default() -> Sticker {
        unsafe { std::mem::zeroed() }
    }
}

impl StickerFactory {
    /// LEN
    pub const LEN: usize = 48;

    /// Check if struct is initialized
    pub fn is_initialized(&self) -> bool {
        self.owner != Pubkey::default()
    }
}

impl Artist {
    /// LEN
    pub const LEN: usize = 608;

    /// Check if struct is initialized
    pub fn is_initialized(&self) -> bool {
        *self != Artist::default()
    }
}

impl Sticker {
    /// LEN
    pub const LEN: usize = 344;

    /// Check if struct is initialized
    pub fn is_initialized(&self) -> bool {
        *self != Sticker::default()
    }
}
