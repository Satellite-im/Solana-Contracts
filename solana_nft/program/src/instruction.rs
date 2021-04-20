//! Instruction types

use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
    sysvar,
};

use crate::state::{NAME_LEN, SYMBOL_LEN, URI_LEN};

#[derive(BorshSerialize, BorshDeserialize, Debug, Clone, PartialEq)]
pub struct MintData {
    pub symbol: [u8; SYMBOL_LEN],
    pub name: [u8; NAME_LEN],
}

impl MintData {
    pub fn new<S: AsRef<str>>(symbol: S, name: S) -> Result<Self, &'static str> {
        let symbol = symbol.as_ref().as_bytes();
        let name = name.as_ref().as_bytes();
        if symbol.len() > SYMBOL_LEN || name.len() > NAME_LEN {
            return Err("symbol or name too long");
        }
        let mut this = Self {
            name: [0; NAME_LEN],
            symbol: [0; SYMBOL_LEN],
        };
        // any shorter notation
        let (left, _) = this.name.split_at_mut(name.len());
        left.copy_from_slice(name);

        let (left, _) = this.symbol.split_at_mut(symbol.len());
        left.copy_from_slice(symbol);

        Ok(this)
    }
}

#[derive(BorshSerialize, BorshDeserialize, Debug, Clone, PartialEq)]
pub struct TokenDataArgs {
    pub hash: Pubkey,
    pub uri: [u8; URI_LEN],
}

impl TokenDataArgs {
    pub fn new(hash: Pubkey, uri: url::Url) -> Result<Self, &'static str> {
        let uri = uri.as_str().as_bytes();
        if uri.len() > URI_LEN {
            return Err("uri too long");
        }
        let mut this = Self {
            hash,
            uri: [0; URI_LEN],
        };
        let (left, _) = this.uri.split_at_mut(uri.len());
        left.copy_from_slice(uri);
        Ok(this)
    }
}

/// Instruction definition
#[allow(clippy::large_enum_variant)]
// can consider making it from primitive, read as input header and manually dispatch to borsh if needed (cause transfer as most often operation is empty input)
#[derive(BorshSerialize, BorshDeserialize, Debug, Clone, PartialEq)]
pub enum NftInstruction {
    /// Initializes a new mint with metadata.
    ///
    /// The `InitializeMint` instruction MUST be
    /// included within the same Transaction as the system program's
    /// `CreateAccount` instruction that creates the account being initialized.
    /// Otherwise another party can acquire ownership of the uninitialized
    /// account.
    ///
    /// Accounts expected by this instruction:
    ///
    ///   0. `[writable]` The mint to initialize.
    ///   1. `[signer]` Authority
    ///   2. `[]` Rent sysvar    InitializeMint,
    InitializeMint(MintData),

    /// Initializes token and token meta data accounts
    ///
    /// The `InitializeToken` instruction requires initialized Mint account and must
    /// and must be in same transaction as `CreateAccount.
    /// Accounts expected by this instruction:
    ///
    /// 0. [writable] token
    /// 1. [writable] token data
    /// 3. [] mint
    /// 4. [] rent
    /// 5. [] owner
    /// 6. [signer]
    InitializeToken(TokenDataArgs),

    /// Transfer from one owner to another.
    /// Current of owner or approval must be signer of transaction.
    /// Cleans existing approval of token.
    ///
    /// Accounts expected by this instruction:
    ///
    /// 1. [writable] token
    /// 2. []         from current owner
    /// 3. []         to new owner
    /// 3. [signer]   approval or owner
    Transfer,

    /// Updates token approval delegate.
    ///
    /// Accounts expected by this instruction:
    ///
    /// 0. [writable] token
    /// 1. []         new_approval
    /// 2. [signer]   approval or owner
    Approve,

    /// Burns token by setting lamports to zero and erasing approval and owner.
    ////
    /// Accounts expected by this instruction:
    ///
    /// 0. [writable] token
    /// 1. [writable] token data
    /// 2. [signer, writable]   approval or owner. move token balance to(thats why it is writable)
    Burn,
}

impl NftInstruction {
    /// Creates `InitializeMint` instruction.
    ///
    /// parameters:
    /// - nft program id
    /// - account created for nft mint metadata
    /// - mint data
    pub fn initialize_mint(
        mint_account: &Pubkey,
        data: MintData,
        authority: &Pubkey,
    ) -> Instruction {
        let data = NftInstruction::InitializeMint(data);
        let accounts = vec![
            AccountMeta::new(*mint_account, false),
            AccountMeta::new_readonly(sysvar::rent::id(), false),
            AccountMeta::new_readonly(*authority, true),
        ];
        Instruction::new_with_borsh(crate::id(), &data, accounts)
    }

    pub fn initialize_token(
        token: &Pubkey,
        token_data: &Pubkey,
        mint: &Pubkey,
        owner: &Pubkey,
        input: TokenDataArgs,
        mint_authority: &Pubkey,
    ) -> Instruction {
        let data = NftInstruction::InitializeToken(input);
        let accounts = vec![
            AccountMeta::new(*token, false),
            AccountMeta::new(*token_data, false),
            AccountMeta::new_readonly(*mint, false),
            AccountMeta::new_readonly(sysvar::rent::id(), false),
            AccountMeta::new_readonly(*owner, false),
            AccountMeta::new_readonly(*mint_authority, true),
        ];
        Instruction::new_with_borsh(crate::id(), &data, accounts)
    }

    pub fn transfer(token: Pubkey, new_owner: Pubkey, approval_or_owner: Pubkey) -> Instruction {
        let data = NftInstruction::Transfer;
        let accounts = vec![
            AccountMeta::new(token, false),
            AccountMeta::new_readonly(new_owner, false),
            AccountMeta::new_readonly(approval_or_owner, true),
        ];
        Instruction::new_with_borsh(crate::id(), &data, accounts)
    }

    pub fn approve(token: Pubkey, new_approval: Pubkey, approval_or_owner: Pubkey) -> Instruction {
        let data = Self::Approve;
        let accounts = vec![
            AccountMeta::new(token, false),
            AccountMeta::new_readonly(new_approval, false),
            AccountMeta::new_readonly(approval_or_owner, true),
        ];
        Instruction::new_with_borsh(crate::id(), &data, accounts)
    }

    pub fn burn(token: Pubkey, token_data: Pubkey, approval_or_owner: Pubkey) -> Instruction {
        let data = Self::Burn;
        let accounts = vec![
            AccountMeta::new(token, false),
            AccountMeta::new(token_data, false),
            AccountMeta::new(approval_or_owner, true),
        ];
        Instruction::new_with_borsh(crate::id(), &data, accounts)
    }
}

#[cfg(test)]
mod tests {
    use super::{MintData, NftInstruction};
    use borsh::{BorshDeserialize, BorshSerialize};

    #[test]
    fn pack_unpack() {
        let mint = NftInstruction::InitializeMint(MintData::new("KC", "Kitty").unwrap());
        let packed = mint.try_to_vec().unwrap();
        let unpack = NftInstruction::deserialize(&mut &packed[..]).unwrap();

        assert_eq!(mint, unpack);
        assert!(matches!(unpack, NftInstruction::InitializeMint(data) if !data.name.is_empty()));
    }
}
