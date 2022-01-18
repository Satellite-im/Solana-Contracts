//! Program state processor

use crate::{
    error::FriendsProgramError,
    instruction::FriendsInstruction,
    state::{Friend},
};
use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::next_account_info,
    account_info::AccountInfo,
    entrypoint::ProgramResult,
    msg,
    program::invoke_signed,
    program_error::ProgramError,
    pubkey::Pubkey,
    system_instruction,
    sysvar::{rent::Rent, Sysvar},
};
// use std::mem;

/// Program state handler.
pub struct Processor {}
impl Processor {
    /// FriendInfo seed
    pub const FRIEND_INFO_SEED: &'static str = "friendinfo";
    /// Outgoing request seed
    pub const OUTGOING_REQUEST: &'static str = "outgoing";
    /// Incoming request seed
    pub const INCOMING_REQUEST: &'static str = "incoming";
    /// Friend seed
    pub const FRIEND_SEED: &'static str = "friend";

    fn create_account<'a>(
        funder: AccountInfo<'a>,
        account_to_create: AccountInfo<'a>,
        base: AccountInfo<'a>,
        seed: &str,
        required_lamports: u64,
        space: u64,
        owner: &Pubkey,
        signer_seeds: &[&[u8]],
    ) -> ProgramResult {
        invoke_signed(
            &system_instruction::create_account_with_seed(
                &funder.key,
                &account_to_create.key,
                &base.key,
                seed,
                required_lamports,
                space,
                owner,
            ),
            &[funder.clone(), account_to_create.clone(), base.clone()],
            &[&signer_seeds],
        )
    }

    /// Create friend request
    pub fn process_create_request_instruction(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        t_f1: [u8; 32],
        t_f2: [u8; 32],
        t_f3: [u8; 32],
        t_f4: [u8; 32],
    ) -> ProgramResult {
        let account_info_iter = &mut accounts.iter();
        let friend_account = next_account_info(account_info_iter)?;
        let mirrored_friend_account = next_account_info(account_info_iter)?;
        let from_account = next_account_info(account_info_iter)?;
        let to_account = next_account_info(account_info_iter)?;
        let rent_account_info = next_account_info(account_info_iter)?;
        let rent = &Rent::from_account_info(rent_account_info)?;

        let (base, _) = Pubkey::find_program_address(
            &[
                &from_account.key.to_bytes()[..32],
                &to_account.key.to_bytes()[..32],
            ],
            program_id,
        );
        let generated_friend_key =
            Pubkey::create_with_seed(&base, Self::FRIEND_SEED, program_id)?;
        if generated_friend_key != *friend_account.key {
            return Err(ProgramError::InvalidSeeds);
        }

        let (base, _) = Pubkey::find_program_address(
            &[
                &to_account.key.to_bytes()[..32],
                &from_account.key.to_bytes()[..32],
            ],
            program_id,
        );
        let generated_mirrored_friend_key =
            Pubkey::create_with_seed(&base, Self::FRIEND_SEED, program_id)?;
        if generated_mirrored_friend_key != *mirrored_friend_account.key {
            return Err(ProgramError::InvalidSeeds);
        }

        if !from_account.is_signer {
            return Err(ProgramError::MissingRequiredSignature);
        }

        let mut friend =
            Friend::try_from_slice(&friend_account.data.borrow())?;
        if friend.status != 0 && friend.status != 3 && friend.status != 4 {
            return Err(ProgramError::AccountAlreadyInitialized);
        }
        let mirrored_friend =
            Friend::try_from_slice(&mirrored_friend_account.data.borrow())?;
        if mirrored_friend.status != 0 && mirrored_friend.status != 3 && mirrored_friend.status != 4 {
            return Err(ProgramError::AccountAlreadyInitialized);
        }

        if !rent.is_exempt(
            friend_account.lamports(),
            friend_account.data_len(),
        ) {
            return Err(ProgramError::AccountNotRentExempt);
        }
        if !rent.is_exempt(
            mirrored_friend_account.lamports(),
            mirrored_friend_account.data_len(),
        ) {
            return Err(ProgramError::AccountNotRentExempt);
        }

        if !friend.is_initialized() {
            friend.from = *from_account.key;
            friend.to = *to_account.key;
            friend.status = 1;
            friend.from_encrypted_key1 = t_f1;
            friend.from_encrypted_key2 = t_f2;
            friend.from_encrypted_key3 = t_f3;
            friend.from_encrypted_key4 = t_f4;
        } else {
            if friend.from != *from_account.key ||
               friend.to != *to_account.key {
                return Err(FriendsProgramError::WrongRequestData.into());
            }
            friend.status = 1;
            friend.from_encrypted_key1 = t_f1;
            friend.from_encrypted_key2 = t_f2;
            friend.from_encrypted_key3 = t_f3;
            friend.from_encrypted_key4 = t_f4;
        }

        if mirrored_friend.is_initialized() {
            if mirrored_friend.from != *to_account.key ||
               mirrored_friend.to != *from_account.key {
                return Err(FriendsProgramError::WrongRequestData.into());
            }
            Friend::default().serialize(&mut *mirrored_friend_account.data.borrow_mut())?;
        }

        friend.serialize(&mut *friend_account.data.borrow_mut()).map_err(|e| e.into())
    }

    /// Accept friend request
    pub fn process_accept_request_instruction(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        t_t1: [u8; 32],
        t_t2: [u8; 32],
        t_t3: [u8; 32],
        t_t4: [u8; 32],
    ) -> ProgramResult {
        let account_info_iter = &mut accounts.iter();
        let friend_account = next_account_info(account_info_iter)?;
        let from_account = next_account_info(account_info_iter)?;
        let to_account = next_account_info(account_info_iter)?;
        let rent_account_info = next_account_info(account_info_iter)?;
        let rent = &Rent::from_account_info(rent_account_info)?;

        let (base, _) = Pubkey::find_program_address(
            &[
                &from_account.key.to_bytes()[..32],
                &to_account.key.to_bytes()[..32],
            ],
            program_id,
        );
        let generated_friend_struct_key =
            Pubkey::create_with_seed(&base, Self::FRIEND_SEED, program_id)?;
        if generated_friend_struct_key != *friend_account.key {
            return Err(ProgramError::InvalidSeeds);
        }

        if !to_account.is_signer {
            return Err(ProgramError::MissingRequiredSignature);
        }

        let mut friend =
            Friend::try_from_slice(&friend_account.data.borrow())?;
        if !friend.is_initialized() {
            return Err(ProgramError::UninitializedAccount);
        }

        if !rent.is_exempt(
            friend_account.lamports(),
            friend_account.data_len(),
        ) {
            return Err(ProgramError::AccountNotRentExempt);
        }

        if friend.from != *from_account.key ||
           friend.to != *to_account.key {
            return Err(FriendsProgramError::WrongRequestData.into());
        }

        if friend.status != 1 {
            return Err(FriendsProgramError::NotPendingRequest.into());
        }

        friend.status = 2;
        friend.to_encrypted_key1 = t_t1;
        friend.to_encrypted_key2 = t_t2;
        friend.to_encrypted_key3 = t_t3;
        friend.to_encrypted_key4 = t_t4;

        friend.serialize(&mut *friend_account.data.borrow_mut()).map_err(|e| e.into())
    }

    /// Deny friend request
    pub fn process_deny_request_instruction(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
    ) -> ProgramResult {
        let account_info_iter = &mut accounts.iter();
        let friend_account = next_account_info(account_info_iter)?;
        let from_account = next_account_info(account_info_iter)?;
        let to_account = next_account_info(account_info_iter)?;
        let rent_account_info = next_account_info(account_info_iter)?;
        let rent = &Rent::from_account_info(rent_account_info)?;

        let (base, _) = Pubkey::find_program_address(
            &[
                &from_account.key.to_bytes()[..32],
                &to_account.key.to_bytes()[..32],
            ],
            program_id,
        );
        let generated_friend_struct_key =
            Pubkey::create_with_seed(&base, Self::FRIEND_SEED, program_id)?;
        if generated_friend_struct_key != *friend_account.key {
            return Err(ProgramError::InvalidSeeds);
        }

        if !to_account.is_signer {
            return Err(ProgramError::MissingRequiredSignature);
        }

        let mut friend =
            Friend::try_from_slice(&friend_account.data.borrow())?;
        if !friend.is_initialized() {
            return Err(ProgramError::UninitializedAccount);
        }

        if !rent.is_exempt(
            friend_account.lamports(),
            friend_account.data_len(),
        ) {
            return Err(ProgramError::AccountNotRentExempt);
        }

        if friend.from != *from_account.key ||
            friend.to != *to_account.key {
            return Err(FriendsProgramError::WrongRequestData.into());
        }

        if friend.status != 1 {
            return Err(FriendsProgramError::NotPendingRequest.into());
        }

        friend.status = 3;
        friend.from_encrypted_key1 = Default::default();
        friend.from_encrypted_key2 = Default::default();
        friend.from_encrypted_key3 = Default::default();
        friend.from_encrypted_key4 = Default::default();
        friend.to_encrypted_key1 = Default::default();
        friend.to_encrypted_key2 = Default::default();
        friend.to_encrypted_key3 = Default::default();
        friend.to_encrypted_key4 = Default::default();

        friend.serialize(&mut *friend_account.data.borrow_mut()).map_err(|e| e.into())
    }

    /// Remove friend request
    pub fn process_remove_request_instruction(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
    ) -> ProgramResult {
        let account_info_iter = &mut accounts.iter();
        let friend_account = next_account_info(account_info_iter)?;
        let from_account = next_account_info(account_info_iter)?;
        let to_account = next_account_info(account_info_iter)?;
        let rent_account_info = next_account_info(account_info_iter)?;
        let rent = &Rent::from_account_info(rent_account_info)?;

        let (base, _) = Pubkey::find_program_address(
            &[
                &from_account.key.to_bytes()[..32],
                &to_account.key.to_bytes()[..32],
            ],
            program_id,
        );
        let generated_friend_struct_key =
            Pubkey::create_with_seed(&base, Self::FRIEND_SEED, program_id)?;
        if generated_friend_struct_key != *friend_account.key {
            return Err(ProgramError::InvalidSeeds);
        }

        if !from_account.is_signer {
            return Err(ProgramError::MissingRequiredSignature);
        }

        let friend =
            Friend::try_from_slice(&friend_account.data.borrow())?;
        if !friend.is_initialized() {
            return Err(ProgramError::UninitializedAccount);
        }

        if !rent.is_exempt(
            friend_account.lamports(),
            friend_account.data_len(),
        ) {
            return Err(ProgramError::AccountNotRentExempt);
        }

        if friend.from != *from_account.key ||
            friend.to != *to_account.key {
            return Err(FriendsProgramError::WrongRequestData.into());
        }

        if friend.status != 1 {
            return Err(FriendsProgramError::NotPendingRequest.into());
        }

        Friend::default().serialize(&mut *friend_account.data.borrow_mut()).map_err(|e| e.into())
    }

    /// Remove friend
    pub fn process_remove_friend_instruction(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
    ) -> ProgramResult {
        let account_info_iter = &mut accounts.iter();
        let friend_account = next_account_info(account_info_iter)?;
        let from_account = next_account_info(account_info_iter)?;
        let to_account = next_account_info(account_info_iter)?;
        let rent_account_info = next_account_info(account_info_iter)?;
        let rent = &Rent::from_account_info(rent_account_info)?;

        let (base, _) = Pubkey::find_program_address(
            &[
                &from_account.key.to_bytes()[..32],
                &to_account.key.to_bytes()[..32],
            ],
            program_id,
        );
        let generated_friend_struct_key =
            Pubkey::create_with_seed(&base, Self::FRIEND_SEED, program_id)?;
        if generated_friend_struct_key != *friend_account.key {
            return Err(ProgramError::InvalidSeeds);
        }

        if !(from_account.is_signer || to_account.is_signer) {
            return Err(ProgramError::MissingRequiredSignature);
        }

        let mut friend =
            Friend::try_from_slice(&friend_account.data.borrow())?;
        if !friend.is_initialized() {
            return Err(ProgramError::UninitializedAccount);
        }

        if !rent.is_exempt(
            friend_account.lamports(),
            friend_account.data_len(),
        ) {
            return Err(ProgramError::AccountNotRentExempt);
        }

        if friend.from != *from_account.key ||
            friend.to != *to_account.key {
            return Err(FriendsProgramError::WrongRequestData.into());
        }

        if friend.status != 2 {
            return Err(FriendsProgramError::NotFriends.into());
        }

        friend.status = 4;
        friend.from_encrypted_key1 = Default::default();
        friend.from_encrypted_key2 = Default::default();
        friend.from_encrypted_key3 = Default::default();
        friend.from_encrypted_key4 = Default::default();
        friend.to_encrypted_key1 = Default::default();
        friend.to_encrypted_key2 = Default::default();
        friend.to_encrypted_key3 = Default::default();
        friend.to_encrypted_key4 = Default::default();

        friend.serialize(&mut *friend_account.data.borrow_mut()).map_err(|e| e.into())
    }

    /// Create derived address
    pub fn process_create_address_instruction(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        friend_key: Pubkey,
    ) -> ProgramResult {
        let account_info_iter = &mut accounts.iter();
        let payer_account_info = next_account_info(account_info_iter)?;
        let user_account_info = next_account_info(account_info_iter)?;
        let base_account_info = next_account_info(account_info_iter)?;
        let account_to_create_info = next_account_info(account_info_iter)?;
        let rent_account_info = next_account_info(account_info_iter)?;
        let rent = &Rent::from_account_info(rent_account_info)?;
        let _system_program = next_account_info(account_info_iter)?;

        let (program_base_address, bump_seed) = Pubkey::find_program_address(
            &[
                &user_account_info.key.to_bytes()[..32],
                &friend_key.to_bytes()[..32],
            ],
            program_id,
        );
        if program_base_address != *base_account_info.key {
            return Err(ProgramError::InvalidSeeds);
        }
        let address_to_create =
            Pubkey::create_with_seed(&program_base_address, Self::FRIEND_SEED, program_id)?;
        if address_to_create != *account_to_create_info.key {
            return Err(ProgramError::InvalidSeeds);
        }
        let signature = &[
            &user_account_info.key.to_bytes()[..32],
            &friend_key.to_bytes()[..32],
            &[bump_seed],
        ];
        Self::create_account(
            payer_account_info.clone(),
            account_to_create_info.clone(),
            base_account_info.clone(),
            Self::FRIEND_SEED,
            rent.minimum_balance(Friend::LEN),
            Friend::LEN as u64,
            program_id,
            signature,
        )?;
        Ok(())
    }

    /// Processes an instruction
    pub fn process_instruction(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        input: &[u8],
    ) -> ProgramResult {
        let instruction = FriendsInstruction::try_from_slice(input)?;
        match instruction {
            FriendsInstruction::MakeRequest(t_f1, t_f2, t_f3, t_f4) => {
                msg!("Instruction: MakeRequest");
                Self::process_create_request_instruction(
                    program_id, accounts, t_f1, t_f2, t_f3, t_f4
                )
            }
            FriendsInstruction::AcceptRequest(t_t1, t_t2, t_t3, t_t4) => {
                msg!("Instruction: AcceptRequest");
                Self::process_accept_request_instruction(
                    program_id, accounts, t_t1, t_t2, t_t3, t_t4
                )
            }
            FriendsInstruction::DenyRequest => {
                msg!("Instruction: DenyRequest");
                Self::process_deny_request_instruction(
                    program_id, accounts,
                )
            }
            FriendsInstruction::RemoveRequest => {
                msg!("Instruction: RemoveRequest");
                Self::process_remove_request_instruction(
                    program_id, accounts,
                )
            }
            FriendsInstruction::RemoveFriend => {
                msg!("Instruction: RemoveFriend");
                Self::process_remove_friend_instruction(
                    program_id, accounts,
                )
            }
            FriendsInstruction::CreateAccount(friend_key) => {
                msg!("Instruction: CreateAccount");
                Self::process_create_address_instruction(program_id, accounts, friend_key)
            }
        }
    }
}
