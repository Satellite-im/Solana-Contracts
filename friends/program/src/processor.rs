//! Program state processor

use crate::{error::ProgramTemplateError, instruction::FriendsInstruction};
use borsh::BorshDeserialize;
use solana_program::{
    account_info::next_account_info, account_info::AccountInfo, entrypoint::ProgramResult, msg,
    pubkey::Pubkey,
};

/// Program state handler.
pub struct Processor {}
impl Processor {
    /// Initialize the friend info
    pub fn process_init_friend_info_instruction(
        _program_id: &Pubkey,
        accounts: &[AccountInfo],
    ) -> ProgramResult {
        let account_info_iter = &mut accounts.iter();
        let _example_account_info = next_account_info(account_info_iter)?;

        Ok(())
    }

    /// Create friend request
    pub fn process_create_request_instruction(
        _program_id: &Pubkey,
        accounts: &[AccountInfo],
    ) -> ProgramResult {
        let account_info_iter = &mut accounts.iter();
        let _example_account_info = next_account_info(account_info_iter)?;

        Ok(())
    }

    /// Accept friend request
    pub fn process_accept_request_instruction(
        _program_id: &Pubkey,
        accounts: &[AccountInfo],
        _thread_id1: [u8; 32],
        _thread_id2: [u8; 32],
    ) -> ProgramResult {
        let account_info_iter = &mut accounts.iter();
        let _example_account_info = next_account_info(account_info_iter)?;

        Ok(())
    }

    /// Deny friend request
    pub fn process_deny_request_instruction(
        _program_id: &Pubkey,
        accounts: &[AccountInfo],
    ) -> ProgramResult {
        let account_info_iter = &mut accounts.iter();
        let _example_account_info = next_account_info(account_info_iter)?;

        Ok(())
    }

    /// Remove friend request
    pub fn process_remove_request_instruction(
        _program_id: &Pubkey,
        accounts: &[AccountInfo],
    ) -> ProgramResult {
        let account_info_iter = &mut accounts.iter();
        let _example_account_info = next_account_info(account_info_iter)?;

        Ok(())
    }

    /// Remove friend
    pub fn process_remove_friend_instruction(
        _program_id: &Pubkey,
        accounts: &[AccountInfo],
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
        let instruction = FriendsInstruction::try_from_slice(input)
            .or(Err(ProgramTemplateError::ExampleError))?;
        match instruction {
            FriendsInstruction::InitFriendInfo => {
                msg!("Instruction: InitFriendInfo");
                Self::process_init_friend_info_instruction(program_id, accounts)
            }
            FriendsInstruction::MakeRequest => {
                msg!("Instruction: MakeRequest");
                Self::process_create_request_instruction(program_id, accounts)
            }
            FriendsInstruction::AcceptRequest(thread_id1, thread_id2) => {
                msg!("Instruction: AcceptRequest");
                Self::process_accept_request_instruction(
                    program_id, accounts, thread_id1, thread_id2,
                )
            }
            FriendsInstruction::DenyRequest => {
                msg!("Instruction: DenyRequest");
                Self::process_deny_request_instruction(program_id, accounts)
            }
            FriendsInstruction::RemoveRequest => {
                msg!("Instruction: RemoveRequest");
                Self::process_remove_request_instruction(program_id, accounts)
            }
            FriendsInstruction::RemoveFriend => {
                msg!("Instruction: RemoveFriend");
                Self::process_remove_friend_instruction(program_id, accounts)
            }
        }
    }
}
