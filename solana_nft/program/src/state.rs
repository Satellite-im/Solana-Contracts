//! State transition types
use borsh::{BorshDeserialize, BorshSchema, BorshSerialize};
use solana_program::pubkey::Pubkey;
#[derive(BorshSerialize, BorshDeserialize, Debug, Clone, Copy, PartialEq, BorshSchema)]
pub enum MintVersion {
    Uninitialized,
    Initialized,
}

pub const SYMBOL_LEN: usize = 8;
pub const NAME_LEN: usize = 32;

#[derive(BorshSerialize, BorshDeserialize, Debug, Clone, BorshSchema)]
pub struct Mint {
    pub version: MintVersion,
    pub symbol: [u8; SYMBOL_LEN],
    pub name: [u8; NAME_LEN],
    pub authority: Pubkey,
}

impl Mint {
    pub const LEN: u64 = 73;
    pub fn new(symbol: [u8; 8], name: [u8; NAME_LEN], authority: Pubkey) -> Self {
        Self {
            version: MintVersion::Initialized,
            symbol,
            name,
            authority,
        }
    }
    pub fn is_initialized(&self) -> bool {
        self.version == MintVersion::Initialized
    }
}

pub const URI_LEN: usize = 256;

#[derive(BorshSerialize, BorshDeserialize, Debug, Clone, Copy, PartialEq, BorshSchema)]
pub enum TokenStatus {
    Uninitialized,
    Initialized,
}

#[derive(BorshSerialize, BorshDeserialize, Debug, Clone, BorshSchema)]
pub struct Token {
    pub version: TokenStatus,
    pub mint: Pubkey,
    pub owner: Pubkey,
    pub approval: Option<Pubkey>,
}

impl Token {
    pub const LEN: u64 = 98;
}

//NOTE:  BorshSchema can be fixed by wrapping OR with BorshSchema changes with Rust 1.51
//the trait `BorshSchema` is not implemented for `[u8; 256]`
#[derive(BorshSerialize, BorshDeserialize, Debug, Clone, Copy, PartialEq)]
pub enum TokenDataStatus {
    Uninitialized,
    Initialized,
}

#[derive(BorshSerialize, BorshDeserialize, Debug, Clone)]
pub struct TokenData {
    pub version: TokenDataStatus,
    pub token: Pubkey,
    pub hash: Pubkey,
    pub uri: [u8; URI_LEN],
}

impl TokenData {
    pub const LEN: u64 = 321;

    pub fn get_uri(&self) -> url::Url {
        url::Url::parse(&String::from_utf8(self.uri.to_vec()).unwrap()).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::{Mint, Token, TokenData, TokenDataStatus, TokenStatus};
    use borsh::{BorshDeserialize, BorshSerialize};
    use solana_program::pubkey::Pubkey;

    #[test]
    fn pack_unpack_mint() {
        let data = Mint::new(
            [1; super::SYMBOL_LEN],
            [2; super::NAME_LEN],
            Pubkey::default(),
        );
        let packed = data.try_to_vec().unwrap();
        assert_eq!(Mint::LEN, packed.len() as u64);
        let unpack = Mint::deserialize(&mut &packed[..]).unwrap();

        assert_eq!(data.symbol, unpack.symbol);
        assert_eq!(data.name, unpack.name);
    }

    #[test]
    fn pack_unpack_token() {
        let data = Token {
            version: TokenStatus::Initialized,
            mint: Pubkey::new_unique(),
            owner: Pubkey::new_unique(),
            approval: Some(Pubkey::new_unique()),
        };
        let packed = data.try_to_vec().unwrap();
        assert_eq!(Token::LEN, packed.len() as u64);
        let unpack = Token::deserialize(&mut &packed[..]).unwrap();

        assert_eq!(data.version, unpack.version);
        assert_eq!(data.owner, unpack.owner);
    }

    #[test]
    fn pack_unpack_token_data() {
        let data = TokenData {
            version: TokenDataStatus::Initialized,
            hash: Pubkey::new_unique(),
            token: Pubkey::new_unique(),
            uri: [11; super::URI_LEN],
        };
        let packed = data.try_to_vec().unwrap();
        assert_eq!(TokenData::LEN, packed.len() as u64);
        let unpack = TokenData::deserialize(&mut &packed[..]).unwrap();

        assert_eq!(data.version, unpack.version);
        assert_eq!(data.uri, unpack.uri);
    }
}
