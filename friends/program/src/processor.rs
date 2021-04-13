//! Program state processor

use crate::{error::ProgramTemplateError, instruction::FriendsInstruction, state::{Friend, FriendInfo, Request}};
use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::next_account_info, account_info::AccountInfo, entrypoint::ProgramResult, msg,
    pubkey::Pubkey,
    program_error::ProgramError,
    sysvar::{rent::Rent, Sysvar},
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
        let friend_info_account = next_account_info(account_info_iter)?;
        let user_account_info = next_account_info(account_info_iter)?;
        let rent_account_info = next_account_info(account_info_iter)?;
        let rent = &Rent::from_account_info(rent_account_info)?;

        let mut friend_info = FriendInfo::try_from_slice(&friend_info_account.data.borrow())?;
        if friend_info.is_initialized() {
            return Err(ProgramError::AccountAlreadyInitialized);
        }

        if !rent.is_exempt(friend_info_account.lamports(), friend_info_account.data_len()) {
            return Err(ProgramError::AccountNotRentExempt);
        }

        if !user_account_info.is_signer {
            return Err(ProgramError::MissingRequiredSignature);
        }

        friend_info.user = *user_account_info.key;
        friend_info.requests_incoming = 0;
        friend_info.requests_outgoing = 0;
        friend_info.friends = 0;

        friend_info.serialize(&mut *friend_info_account.data.borrow_mut())
            .map_err(|e| e.into())
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
