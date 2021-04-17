//! Program state processor

use crate::{error::ProgramTemplateError, instruction::{StickerInstruction, RegisterArtist, CreateNewSticker, AddressType}};
use borsh::BorshDeserialize;
use solana_program::{
    account_info::next_account_info, account_info::AccountInfo, entrypoint::ProgramResult, msg,
    pubkey::Pubkey,
};

/// Program state handler.
pub struct Processor {}
impl Processor {
    /// Register new artist
    pub fn process_register_artist_instruction(
        _program_id: &Pubkey,
        accounts: &[AccountInfo],
        _args: RegisterArtist,
    ) -> ProgramResult {
        let account_info_iter = &mut accounts.iter();
        let _example_account_info = next_account_info(account_info_iter)?;

        Ok(())
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
