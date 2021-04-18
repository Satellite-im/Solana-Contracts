//! Program state processor

use crate::{
    error::StickerProgramError,
    instruction::{AddressType, CreateNewSticker, RegisterArtist, StickerInstruction},
    state::{Artist, Sticker, StickerFactory},
};
use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::next_account_info, account_info::AccountInfo, entrypoint::ProgramResult, msg,
    program_error::ProgramError, program_pack::Pack, pubkey::Pubkey, sysvar::rent::Rent,
    sysvar::Sysvar,
};
use spl_token::state::Account;

/// Program state handler.
pub struct Processor {}
impl Processor {
    /// Register new artist
    pub fn process_register_artist_instruction(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        args: RegisterArtist,
    ) -> ProgramResult {
        let account_info_iter = &mut accounts.iter();
        let user_account_info = next_account_info(account_info_iter)?;
        let user_token_account_info = next_account_info(account_info_iter)?;
        let artist_to_create_account_info = next_account_info(account_info_iter)?;
        let sticker_factory_owner_account_info = next_account_info(account_info_iter)?;
        let sticker_factory_account_info = next_account_info(account_info_iter)?;
        let rent_account_info = next_account_info(account_info_iter)?;
        let rent = &Rent::from_account_info(rent_account_info)?;

        let user_token = Account::unpack_from_slice(&user_token_account_info.data.borrow())?;
        if !user_token.is_native() {
            return Err(StickerProgramError::WrongTokenMint.into());
        }

        let mut sticker_factory =
            StickerFactory::try_from_slice(&sticker_factory_account_info.data.borrow())?;
        if !sticker_factory.is_initialized() {
            return Err(ProgramError::UninitializedAccount);
        }

        if *sticker_factory_owner_account_info.key != sticker_factory.owner {
            return Err(StickerProgramError::WrongStickerFactoryOwner.into());
        }

        if !sticker_factory_owner_account_info.is_signer {
            return Err(ProgramError::MissingRequiredSignature);
        }

        let (base, _) = Pubkey::find_program_address(
            &[&sticker_factory_account_info.key.to_bytes()[..32]],
            program_id,
        );
        let generated_artist_key = Pubkey::create_with_seed(
            &base,
            &format!("{:?}", sticker_factory.artist_count),
            program_id,
        )?;
        if generated_artist_key != *artist_to_create_account_info.key {
            return Err(ProgramError::InvalidSeeds);
        }

        if !rent.is_exempt(
            artist_to_create_account_info.lamports(),
            artist_to_create_account_info.data_len(),
        ) {
            return Err(ProgramError::AccountNotRentExempt);
        }

        let mut artist = Artist::try_from_slice(&artist_to_create_account_info.data.borrow())?;
        artist.user = *user_account_info.key;
        artist.user_token_acc = *user_token_account_info.key;
        artist.name = args.name;
        artist.signature = args.signature;
        artist.description = args.description;

        sticker_factory.artist_count = sticker_factory
            .artist_count
            .checked_add(1)
            .ok_or::<ProgramError>(StickerProgramError::CalculationError.into())?;

        artist.serialize(&mut *artist_to_create_account_info.data.borrow_mut())?;
        sticker_factory
            .serialize(&mut *sticker_factory_account_info.data.borrow_mut())
            .map_err(|e| e.into())
    }

    /// Create new sticker
    pub fn process_create_new_sticker_instruction(
        _program_id: &Pubkey,
        accounts: &[AccountInfo],
        _args: CreateNewSticker,
    ) -> ProgramResult {
        let account_info_iter = &mut accounts.iter();
        let _example_account_info = next_account_info(account_info_iter)?;

        Ok(())
    }

    /// Buy sticker
    pub fn process_buy_sticker_instruction(
        _program_id: &Pubkey,
        accounts: &[AccountInfo],
    ) -> ProgramResult {
        let account_info_iter = &mut accounts.iter();
        let _example_account_info = next_account_info(account_info_iter)?;

        Ok(())
    }

    /// Change sticker price
    pub fn process_change_sticker_price_instruction(
        _program_id: &Pubkey,
        accounts: &[AccountInfo],
        _new_price: u64,
    ) -> ProgramResult {
        let account_info_iter = &mut accounts.iter();
        let _example_account_info = next_account_info(account_info_iter)?;

        Ok(())
    }

    /// Create new account
    pub fn process_create_account_instruction(
        _program_id: &Pubkey,
        accounts: &[AccountInfo],
        _address_type: AddressType,
    ) -> ProgramResult {
        let account_info_iter = &mut accounts.iter();
        let _example_account_info = next_account_info(account_info_iter)?;

        Ok(())
    }

    /// Processes an instruction
    pub fn process_instruction(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        input: &[u8],
    ) -> ProgramResult {
        let instruction = StickerInstruction::try_from_slice(input)?;
        match instruction {
            StickerInstruction::RegisterArtist(args) => {
                msg!("Instruction: RegisterArtist");
                Self::process_register_artist_instruction(program_id, accounts, args)
            }
            StickerInstruction::CreateNewSticker(args) => {
                msg!("Instruction: CreateNewSticker");
                Self::process_create_new_sticker_instruction(program_id, accounts, args)
            }
            StickerInstruction::BuySticker => {
                msg!("Instruction: BuySticker");
                Self::process_buy_sticker_instruction(program_id, accounts)
            }
            StickerInstruction::ChangeStickerPrice(new_price) => {
                msg!("Instruction: ChangeStickerPrice");
                Self::process_change_sticker_price_instruction(program_id, accounts, new_price)
            }
            StickerInstruction::CreateAccount(account_type) => {
                msg!("Instruction: CreateAccount");
                Self::process_create_account_instruction(program_id, accounts, account_type)
            }
        }
    }
}
