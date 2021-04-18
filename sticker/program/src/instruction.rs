//! Instruction types

use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    instruction::{AccountMeta, Instruction},
    program_error::ProgramError,
    pubkey::Pubkey,
    system_program, sysvar,
};

/// Arguments to create new artist
#[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug, Clone)]
pub struct RegisterArtist {
    /// Name
    pub name: [u8; 32],
    /// Signature
    pub signature: [u8; 256],
    /// Description
    pub description: [u8; 256],
}

/// Arguments to create new sticker
#[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug, Clone)]
pub struct CreateNewSticker {
    /// Max supply
    pub max_supply: u64,
    /// Price
    pub price: u64,
    /// URI
    pub uri: [u8; 256],
}

/// Address type
#[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug, Clone)]
pub enum AddressType {
    /// Artist
    Artist,
    /// Sticker
    Sticker,
}

/// Instruction definition
#[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug, Clone)]
pub enum StickerInstruction {
    /// RegisterArtist
    ///
    ///   0. `[r]` User
    ///   1. `[r]` Account to receive payments
    ///   2. `[w]` Uninitialized artist account
    ///   3. `[rs]` Sticker factory owner
    ///   4. `[w]` Sticker factory
    ///   5. `[r]` Rent
    RegisterArtist(RegisterArtist),

    /// CreateNewSticker
    ///
    ///   0. `[w]` Sticker account
    ///   1. `[w]` Sticker factory
    ///   2. `[r]` Created NFT mint. Authority should be the program address
    ///   3. `[r]` Artist
    ///   4. `[rs]` Artist's user
    ///   5. `[r]` Rent
    CreateNewSticker(CreateNewSticker),

    /// BuySticker
    ///
    ///   0. `[r]` Sticker to buy
    ///   1. `[w]` Artist's token account to receive payments
    ///   2. `[w]` Buyer's token account
    ///   3. `[rs]` Buyer's transfer authority
    ///   4. `[w]` NFT token
    ///   5. `[w]` NFT token data
    ///   6. `[r]` NFT token mint
    ///   7. `[r]` NFT token owner, user's account
    BuySticker,

    /// ChangeStickerPrice
    ///
    ///   0. `[w]` Sticker
    ///   1. `[rs]` Creator
    ChangeStickerPrice(u64),

    /// CreateAccount
    ///
    ///   0. `[sw]` Payer
    ///   1. `[r]` Sticker factory
    ///   2. `[r]` Base
    ///   3. `[w]` Account to create
    CreateAccount(AddressType),
}

/// Create `CreateAccount` instruction
pub fn create_account(
    program_id: &Pubkey,
    payer: &Pubkey,
    sticker_factory: &Pubkey,
    base_address: &Pubkey,
    account_to_create: &Pubkey,
    address_type: AddressType,
) -> Result<Instruction, ProgramError> {
    let init_data = StickerInstruction::CreateAccount(address_type);
    let data = init_data
        .try_to_vec()
        .or(Err(ProgramError::InvalidArgument))?;
    let accounts = vec![
        AccountMeta::new(*payer, true),
        AccountMeta::new_readonly(*sticker_factory, false),
        AccountMeta::new_readonly(*base_address, false),
        AccountMeta::new(*account_to_create, false),
        AccountMeta::new_readonly(sysvar::rent::id(), false),
        AccountMeta::new_readonly(system_program::id(), false),
    ];
    Ok(Instruction {
        program_id: *program_id,
        accounts,
        data,
    })
}

/// Create `RegisterArtist` instruction
pub fn register_artist(
    program_id: &Pubkey,
    user: &Pubkey,
    user_token: &Pubkey,
    artist_to_create: &Pubkey,
    sticker_factory_owner: &Pubkey,
    sticker_factory: &Pubkey,
    args: RegisterArtist,
) -> Result<Instruction, ProgramError> {
    let init_data = StickerInstruction::RegisterArtist(args);
    let data = init_data.try_to_vec()?;
    let accounts = vec![
        AccountMeta::new_readonly(*user, false),
        AccountMeta::new_readonly(*user_token, false),
        AccountMeta::new(*artist_to_create, false),
        AccountMeta::new_readonly(*sticker_factory_owner, true),
        AccountMeta::new(*sticker_factory, false),
        AccountMeta::new_readonly(sysvar::rent::id(), false),
    ];
    Ok(Instruction {
        program_id: *program_id,
        accounts,
        data,
    })
}

/// Create `CreateNewSticker` instruction
pub fn create_new_sticker(
    program_id: &Pubkey,
    sticker: &Pubkey,
    sticker_factory: &Pubkey,
    mint: &Pubkey,
    artist: &Pubkey,
    user: &Pubkey,
    args: CreateNewSticker,
) -> Result<Instruction, ProgramError> {
    let init_data = StickerInstruction::CreateNewSticker(args);
    let data = init_data.try_to_vec()?;
    let accounts = vec![
        AccountMeta::new(*sticker, false),
        AccountMeta::new(*sticker_factory, false),
        AccountMeta::new_readonly(*mint, false),
        AccountMeta::new_readonly(*artist, false),
        AccountMeta::new_readonly(*user, true),
        AccountMeta::new_readonly(sysvar::rent::id(), false),
    ];
    Ok(Instruction {
        program_id: *program_id,
        accounts,
        data,
    })
}

/// Create `BuySticker` instruction
pub fn buy_sticker(
    program_id: &Pubkey,
    sticker_to_buy: &Pubkey,
    artist_token_acc: &Pubkey,
    buyer_token_acc: &Pubkey,
    buyer_transfer_authority: &Pubkey,
    nft_token: &Pubkey,
    nft_token_data: &Pubkey,
    nft_token_mint: &Pubkey,
    nft_token_owner: &Pubkey,
) -> Result<Instruction, ProgramError> {
    let init_data = StickerInstruction::BuySticker;
    let data = init_data.try_to_vec()?;
    let accounts = vec![
        AccountMeta::new_readonly(*sticker_to_buy, false),
        AccountMeta::new(*artist_token_acc, false),
        AccountMeta::new(*buyer_token_acc, false),
        AccountMeta::new_readonly(*buyer_transfer_authority, true),
        AccountMeta::new(*nft_token, false),
        AccountMeta::new(*nft_token_data, false),
        AccountMeta::new_readonly(*nft_token_mint, false),
        AccountMeta::new_readonly(*nft_token_owner, false),
    ];
    Ok(Instruction {
        program_id: *program_id,
        accounts,
        data,
    })
}

// TODO: change sticker price
