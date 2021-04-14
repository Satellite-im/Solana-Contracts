//! Program state processor

use crate::{
    error::FriendsProgramError,
    instruction::FriendsInstruction,
    state::{Friend, FriendInfo, Request},
};
use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::next_account_info,
    account_info::AccountInfo,
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
    sysvar::{rent::Rent, Sysvar},
};
use std::mem;

/// Program state handler.
pub struct Processor {}
impl Processor {
    /// FriendInfo seed
    pub const FRIEND_INFO_SEED: &'static [u8] = b"friendinfo";
    /// Outgoing request seed
    pub const OUTGOING_REQUEST: &'static [u8] = b"outgoing";
    /// Incoming request seed
    pub const INCOMING_REQUEST: &'static [u8] = b"incoming";
    /// Friend seed
    pub const FRIEND_SEED: &'static [u8] = b"friend";

    /// Remove request
    pub fn remove_request(
        request_from_to: &mut Request,
        request_from_to_acc: &AccountInfo,
        request_to_from: &mut Request,
        request_to_from_acc: &AccountInfo,
        last_request_from_to: &mut Request,
        last_request_from_to_acc: &AccountInfo,
        last_request_to_from: &mut Request,
        last_request_to_from_acc: &AccountInfo,
        friend_info_from: &FriendInfo,
        friend_info_to: &FriendInfo,
        program_id: &Pubkey,
    ) -> Result<(), ProgramError> {
        if request_from_to_acc.key == last_request_from_to_acc.key {
            let index = friend_info_from
                .requests_outgoing
                .checked_sub(1)
                .ok_or::<ProgramError>(FriendsProgramError::CalculationError.into())?;
            let generated_request_key = Pubkey::create_program_address(
                &[
                    &request_from_to.from.to_bytes()[..32],
                    &index.to_le_bytes()[..8],
                    Self::OUTGOING_REQUEST,
                ],
                program_id,
            )?;
            if generated_request_key != *request_from_to_acc.key {
                return Err(ProgramError::InvalidSeeds);
            }
            *request_from_to = Request::default();
            request_from_to.serialize(&mut *request_from_to_acc.data.borrow_mut())?;
        } else {
            let index = friend_info_from
                .requests_outgoing
                .checked_sub(1)
                .ok_or::<ProgramError>(FriendsProgramError::CalculationError.into())?;
            let generated_request_key = Pubkey::create_program_address(
                &[
                    &request_from_to.from.to_bytes()[..32],
                    &index.to_le_bytes()[..8],
                    Self::OUTGOING_REQUEST,
                ],
                program_id,
            )?;
            if generated_request_key != *last_request_from_to_acc.key {
                return Err(ProgramError::InvalidSeeds);
            }
            mem::swap(request_from_to, last_request_from_to);
            request_from_to.serialize(&mut *request_from_to_acc.data.borrow_mut())?;
            last_request_from_to.serialize(&mut *last_request_from_to_acc.data.borrow_mut())?;
        }
        if request_to_from_acc.key == last_request_to_from_acc.key {
            let index = friend_info_to
                .requests_incoming
                .checked_sub(1)
                .ok_or::<ProgramError>(FriendsProgramError::CalculationError.into())?;
            let generated_request_key = Pubkey::create_program_address(
                &[
                    &request_to_from.to.to_bytes()[..32],
                    &index.to_le_bytes()[..8],
                    Self::INCOMING_REQUEST,
                ],
                program_id,
            )?;
            if generated_request_key != *request_to_from_acc.key {
                return Err(ProgramError::InvalidSeeds);
            }
            *request_to_from = Request::default();
            request_to_from.serialize(&mut *request_to_from_acc.data.borrow_mut())?;
        } else {
            let index = friend_info_to
                .requests_incoming
                .checked_sub(1)
                .ok_or::<ProgramError>(FriendsProgramError::CalculationError.into())?;
            let generated_request_key = Pubkey::create_program_address(
                &[
                    &request_to_from.to.to_bytes()[..32],
                    &index.to_le_bytes()[..8],
                    Self::INCOMING_REQUEST,
                ],
                program_id,
            )?;
            if generated_request_key != *last_request_to_from_acc.key {
                return Err(ProgramError::InvalidSeeds);
            }
            mem::swap(request_to_from, last_request_to_from);
            request_to_from.serialize(&mut *request_to_from_acc.data.borrow_mut())?;
            last_request_to_from.serialize(&mut *last_request_to_from_acc.data.borrow_mut())?;
        }
        Ok(())
    }

    /// Initialize the friend info
    pub fn process_init_friend_info_instruction(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
    ) -> ProgramResult {
        let account_info_iter = &mut accounts.iter();
        let friend_info_account = next_account_info(account_info_iter)?;
        let user_account_info = next_account_info(account_info_iter)?;
        let rent_account_info = next_account_info(account_info_iter)?;
        let rent = &Rent::from_account_info(rent_account_info)?;

        let generated_friend_info = Pubkey::create_program_address(
            &[
                &user_account_info.key.to_bytes()[..32],
                Self::FRIEND_INFO_SEED,
            ],
            program_id,
        )?;
        if generated_friend_info != *friend_info_account.key {
            return Err(ProgramError::InvalidSeeds);
        }

        let mut friend_info = FriendInfo::try_from_slice(&friend_info_account.data.borrow())?;
        if friend_info.is_initialized() {
            return Err(ProgramError::AccountAlreadyInitialized);
        }

        if !rent.is_exempt(
            friend_info_account.lamports(),
            friend_info_account.data_len(),
        ) {
            return Err(ProgramError::AccountNotRentExempt);
        }

        if !user_account_info.is_signer {
            return Err(ProgramError::MissingRequiredSignature);
        }

        friend_info.user = *user_account_info.key;
        friend_info.requests_incoming = 0;
        friend_info.requests_outgoing = 0;
        friend_info.friends = 0;

        friend_info
            .serialize(&mut *friend_info_account.data.borrow_mut())
            .map_err(|e| e.into())
    }

    /// Create friend request
    pub fn process_create_request_instruction(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
    ) -> ProgramResult {
        let account_info_iter = &mut accounts.iter();
        let request_from_to_account_info = next_account_info(account_info_iter)?;
        let request_to_from_account_info = next_account_info(account_info_iter)?;
        let friend_info_from_account_info = next_account_info(account_info_iter)?;
        let friend_info_to_account_info = next_account_info(account_info_iter)?;
        let user_account_info = next_account_info(account_info_iter)?;
        let rent_account_info = next_account_info(account_info_iter)?;
        let rent = &Rent::from_account_info(rent_account_info)?;

        let mut friend_info_from =
            FriendInfo::try_from_slice(&friend_info_from_account_info.data.borrow())?;
        if !friend_info_from.is_initialized() {
            return Err(ProgramError::UninitializedAccount);
        }

        let mut friend_info_to =
            FriendInfo::try_from_slice(&friend_info_to_account_info.data.borrow())?;
        if !friend_info_to.is_initialized() {
            return Err(ProgramError::UninitializedAccount);
        }

        let generated_request_from_to_key = Pubkey::create_program_address(
            &[
                &friend_info_from.user.to_bytes()[..32],
                &friend_info_from.requests_outgoing.to_le_bytes()[..8],
                Self::OUTGOING_REQUEST,
            ],
            program_id,
        )?;
        if generated_request_from_to_key != *request_from_to_account_info.key {
            return Err(ProgramError::InvalidSeeds);
        }

        let generated_request_to_from_key = Pubkey::create_program_address(
            &[
                &friend_info_to.user.to_bytes()[..32],
                &friend_info_to.requests_incoming.to_le_bytes()[..8],
                Self::INCOMING_REQUEST,
            ],
            program_id,
        )?;
        if generated_request_to_from_key != *request_to_from_account_info.key {
            return Err(ProgramError::InvalidSeeds);
        }

        if !rent.is_exempt(
            request_from_to_account_info.lamports(),
            request_from_to_account_info.data_len(),
        ) {
            return Err(ProgramError::AccountNotRentExempt);
        }

        if !rent.is_exempt(
            request_to_from_account_info.lamports(),
            request_to_from_account_info.data_len(),
        ) {
            return Err(ProgramError::AccountNotRentExempt);
        }

        let mut request_from_to =
            Request::try_from_slice(&request_from_to_account_info.data.borrow())?;
        if request_from_to.is_initialized() {
            return Err(ProgramError::AccountAlreadyInitialized);
        }

        let mut request_to_from =
            Request::try_from_slice(&request_to_from_account_info.data.borrow())?;
        if request_to_from.is_initialized() {
            return Err(ProgramError::AccountAlreadyInitialized);
        }

        if friend_info_from.user != *user_account_info.key || !user_account_info.is_signer {
            return Err(ProgramError::MissingRequiredSignature);
        }

        request_from_to.from = friend_info_from.user;
        request_from_to.to = friend_info_to.user;

        request_to_from.from = friend_info_from.user;
        request_to_from.to = friend_info_to.user;

        friend_info_from.requests_outgoing =
            friend_info_from
                .requests_outgoing
                .checked_add(1)
                .ok_or::<ProgramError>(FriendsProgramError::CalculationError.into())?;

        friend_info_to.requests_incoming =
            friend_info_to
                .requests_incoming
                .checked_add(1)
                .ok_or::<ProgramError>(FriendsProgramError::CalculationError.into())?;

        request_from_to.serialize(&mut *request_from_to_account_info.data.borrow_mut())?;
        request_to_from.serialize(&mut *request_to_from_account_info.data.borrow_mut())?;

        friend_info_from.serialize(&mut *friend_info_from_account_info.data.borrow_mut())?;
        friend_info_to
            .serialize(&mut *friend_info_to_account_info.data.borrow_mut())
            .map_err(|e| e.into())
    }

    /// Accept friend request
    pub fn process_accept_request_instruction(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        thread_id1: [u8; 32],
        thread_id2: [u8; 32],
    ) -> ProgramResult {
        let account_info_iter = &mut accounts.iter();
        let request_from_to_account_info = next_account_info(account_info_iter)?;
        let request_to_from_account_info = next_account_info(account_info_iter)?;
        let last_request_from_to_account_info = next_account_info(account_info_iter)?;
        let last_request_to_from_account_info = next_account_info(account_info_iter)?;
        let friend_info_from_account_info = next_account_info(account_info_iter)?;
        let friend_info_to_account_info = next_account_info(account_info_iter)?;
        let friend_to_account_info = next_account_info(account_info_iter)?;
        let friend_from_account_info = next_account_info(account_info_iter)?;
        let user_to_account_info = next_account_info(account_info_iter)?;
        let rent_account_info = next_account_info(account_info_iter)?;
        let rent = &Rent::from_account_info(rent_account_info)?;

        let mut request_from_to =
            Request::try_from_slice(&request_from_to_account_info.data.borrow())?;
        if !request_from_to.is_initialized() {
            return Err(ProgramError::UninitializedAccount);
        }

        let mut request_to_from =
            Request::try_from_slice(&request_to_from_account_info.data.borrow())?;
        if !request_to_from.is_initialized() {
            return Err(ProgramError::UninitializedAccount);
        }

        let mut last_request_from_to =
            Request::try_from_slice(&last_request_from_to_account_info.data.borrow())?;
        if !last_request_from_to.is_initialized() {
            return Err(ProgramError::UninitializedAccount);
        }

        let mut last_request_to_from =
            Request::try_from_slice(&last_request_to_from_account_info.data.borrow())?;
        if !last_request_to_from.is_initialized() {
            return Err(ProgramError::UninitializedAccount);
        }

        let mut friend_info_from =
            FriendInfo::try_from_slice(&friend_info_from_account_info.data.borrow())?;
        if !friend_info_from.is_initialized() {
            return Err(ProgramError::UninitializedAccount);
        }

        let mut friend_info_to =
            FriendInfo::try_from_slice(&friend_info_to_account_info.data.borrow())?;
        if !friend_info_to.is_initialized() {
            return Err(ProgramError::UninitializedAccount);
        }

        if request_from_to.from != friend_info_from.user
            || request_from_to.to != friend_info_to.user
        {
            return Err(FriendsProgramError::WrongRequestData.into());
        }

        if request_to_from.from != friend_info_from.user
            || request_to_from.to != friend_info_to.user
        {
            return Err(FriendsProgramError::WrongRequestData.into());
        }

        if last_request_from_to.from != friend_info_from.user {
            return Err(FriendsProgramError::WrongRequestData.into());
        }

        if last_request_to_from.to != friend_info_to.user {
            return Err(FriendsProgramError::WrongRequestData.into());
        }

        let mut friend_to = Friend::try_from_slice(&friend_to_account_info.data.borrow())?;
        if friend_to.is_initialized() {
            return Err(ProgramError::AccountAlreadyInitialized); // TODO: add error which inform about being friends already
        }

        if !rent.is_exempt(
            friend_to_account_info.lamports(),
            friend_to_account_info.data_len(),
        ) {
            return Err(ProgramError::AccountNotRentExempt);
        }

        let mut friend_from = Friend::try_from_slice(&friend_from_account_info.data.borrow())?;
        if friend_from.is_initialized() {
            return Err(ProgramError::AccountAlreadyInitialized); // TODO: add error which inform about being friends already
        }

        if !rent.is_exempt(
            friend_from_account_info.lamports(),
            friend_from_account_info.data_len(),
        ) {
            return Err(ProgramError::AccountNotRentExempt);
        }

        let generated_friend_to_key = Pubkey::create_program_address(
            &[
                &friend_info_to.user.to_bytes()[..32],
                &friend_info_from.user.to_bytes()[..32],
                Self::FRIEND_SEED,
            ],
            program_id,
        )?;
        if generated_friend_to_key != *friend_to_account_info.key {
            return Err(ProgramError::InvalidSeeds);
        }

        let generated_friend_from_key = Pubkey::create_program_address(
            &[
                &friend_info_from.user.to_bytes()[..32],
                &friend_info_to.user.to_bytes()[..32],
                Self::FRIEND_SEED,
            ],
            program_id,
        )?;
        if generated_friend_from_key != *friend_from_account_info.key {
            return Err(ProgramError::InvalidSeeds);
        }

        if friend_info_to.user != *user_to_account_info.key || !user_to_account_info.is_signer {
            return Err(ProgramError::MissingRequiredSignature);
        }

        Self::remove_request(
            &mut request_from_to,
            &request_from_to_account_info,
            &mut request_to_from,
            &request_to_from_account_info,
            &mut last_request_from_to,
            &last_request_from_to_account_info,
            &mut last_request_to_from,
            &last_request_to_from_account_info,
            &friend_info_from,
            &friend_info_to,
            program_id,
        )?;

        friend_to.thread_id1 = thread_id1;
        friend_to.thread_id2 = thread_id2;
        friend_to.user = friend_info_to.user;
        friend_to.friend = friend_info_from.user;

        friend_from.thread_id1 = thread_id1;
        friend_from.thread_id2 = thread_id2;
        friend_from.user = friend_info_from.user;
        friend_from.friend = friend_info_to.user;

        friend_info_from.requests_outgoing =
            friend_info_from
                .requests_outgoing
                .checked_sub(1)
                .ok_or::<ProgramError>(FriendsProgramError::CalculationError.into())?;
        friend_info_from.friends = friend_info_from
            .friends
            .checked_add(1)
            .ok_or::<ProgramError>(FriendsProgramError::CalculationError.into())?;

        friend_info_to.requests_incoming =
            friend_info_to
                .requests_incoming
                .checked_sub(1)
                .ok_or::<ProgramError>(FriendsProgramError::CalculationError.into())?;
        friend_info_to.friends = friend_info_to
            .friends
            .checked_add(1)
            .ok_or::<ProgramError>(FriendsProgramError::CalculationError.into())?;

        friend_to.serialize(&mut *friend_to_account_info.data.borrow_mut())?;
        friend_from.serialize(&mut *friend_from_account_info.data.borrow_mut())?;
        friend_info_from.serialize(&mut *friend_info_from_account_info.data.borrow_mut())?;
        friend_info_to
            .serialize(&mut *friend_info_to_account_info.data.borrow_mut())
            .map_err(|e| e.into())
    }

    /// Deny friend request
    pub fn process_deny_request_instruction(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
    ) -> ProgramResult {
        let account_info_iter = &mut accounts.iter();
        let request_from_to_account_info = next_account_info(account_info_iter)?;
        let request_to_from_account_info = next_account_info(account_info_iter)?;
        let last_request_from_to_account_info = next_account_info(account_info_iter)?;
        let last_request_to_from_account_info = next_account_info(account_info_iter)?;
        let friend_info_from_account_info = next_account_info(account_info_iter)?;
        let friend_info_to_account_info = next_account_info(account_info_iter)?;
        let user_to_account_info = next_account_info(account_info_iter)?;

        let mut request_from_to =
            Request::try_from_slice(&request_from_to_account_info.data.borrow())?;
        if !request_from_to.is_initialized() {
            return Err(ProgramError::UninitializedAccount);
        }

        let mut request_to_from =
            Request::try_from_slice(&request_to_from_account_info.data.borrow())?;
        if !request_to_from.is_initialized() {
            return Err(ProgramError::UninitializedAccount);
        }

        let mut last_request_from_to =
            Request::try_from_slice(&last_request_from_to_account_info.data.borrow())?;
        if !last_request_from_to.is_initialized() {
            return Err(ProgramError::UninitializedAccount);
        }

        let mut last_request_to_from =
            Request::try_from_slice(&last_request_to_from_account_info.data.borrow())?;
        if !last_request_to_from.is_initialized() {
            return Err(ProgramError::UninitializedAccount);
        }

        let mut friend_info_from =
            FriendInfo::try_from_slice(&friend_info_from_account_info.data.borrow())?;
        if !friend_info_from.is_initialized() {
            return Err(ProgramError::UninitializedAccount);
        }

        let mut friend_info_to =
            FriendInfo::try_from_slice(&friend_info_to_account_info.data.borrow())?;
        if !friend_info_to.is_initialized() {
            return Err(ProgramError::UninitializedAccount);
        }

        if request_from_to.from != friend_info_from.user
            || request_from_to.to != friend_info_to.user
        {
            return Err(FriendsProgramError::WrongRequestData.into());
        }

        if request_to_from.from != friend_info_from.user
            || request_to_from.to != friend_info_to.user
        {
            return Err(FriendsProgramError::WrongRequestData.into());
        }

        if last_request_from_to.from != friend_info_from.user {
            return Err(FriendsProgramError::WrongRequestData.into());
        }

        if last_request_to_from.to != friend_info_to.user {
            return Err(FriendsProgramError::WrongRequestData.into());
        }

        if friend_info_to.user != *user_to_account_info.key || !user_to_account_info.is_signer {
            return Err(ProgramError::MissingRequiredSignature);
        }

        Self::remove_request(
            &mut request_from_to,
            &request_from_to_account_info,
            &mut request_to_from,
            &request_to_from_account_info,
            &mut last_request_from_to,
            &last_request_from_to_account_info,
            &mut last_request_to_from,
            &last_request_to_from_account_info,
            &friend_info_from,
            &friend_info_to,
            program_id,
        )?;

        friend_info_from.requests_outgoing =
            friend_info_from
                .requests_outgoing
                .checked_sub(1)
                .ok_or::<ProgramError>(FriendsProgramError::CalculationError.into())?;

        friend_info_to.requests_incoming =
            friend_info_to
                .requests_incoming
                .checked_sub(1)
                .ok_or::<ProgramError>(FriendsProgramError::CalculationError.into())?;

        friend_info_from.serialize(&mut *friend_info_from_account_info.data.borrow_mut())?;
        friend_info_to
            .serialize(&mut *friend_info_to_account_info.data.borrow_mut())
            .map_err(|e| e.into())
    }

    /// Remove friend request
    pub fn process_remove_request_instruction(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
    ) -> ProgramResult {
        let account_info_iter = &mut accounts.iter();
        let request_from_to_account_info = next_account_info(account_info_iter)?;
        let request_to_from_account_info = next_account_info(account_info_iter)?;
        let last_request_from_to_account_info = next_account_info(account_info_iter)?;
        let last_request_to_from_account_info = next_account_info(account_info_iter)?;
        let friend_info_from_account_info = next_account_info(account_info_iter)?;
        let friend_info_to_account_info = next_account_info(account_info_iter)?;
        let user_from_account_info = next_account_info(account_info_iter)?;

        let mut request_from_to =
            Request::try_from_slice(&request_from_to_account_info.data.borrow())?;
        if !request_from_to.is_initialized() {
            return Err(ProgramError::UninitializedAccount);
        }

        let mut request_to_from =
            Request::try_from_slice(&request_to_from_account_info.data.borrow())?;
        if !request_to_from.is_initialized() {
            return Err(ProgramError::UninitializedAccount);
        }

        let mut last_request_from_to =
            Request::try_from_slice(&last_request_from_to_account_info.data.borrow())?;
        if !last_request_from_to.is_initialized() {
            return Err(ProgramError::UninitializedAccount);
        }

        let mut last_request_to_from =
            Request::try_from_slice(&last_request_to_from_account_info.data.borrow())?;
        if !last_request_to_from.is_initialized() {
            return Err(ProgramError::UninitializedAccount);
        }

        let mut friend_info_from =
            FriendInfo::try_from_slice(&friend_info_from_account_info.data.borrow())?;
        if !friend_info_from.is_initialized() {
            return Err(ProgramError::UninitializedAccount);
        }

        let mut friend_info_to =
            FriendInfo::try_from_slice(&friend_info_to_account_info.data.borrow())?;
        if !friend_info_to.is_initialized() {
            return Err(ProgramError::UninitializedAccount);
        }

        if request_from_to.from != friend_info_from.user
            || request_from_to.to != friend_info_to.user
        {
            return Err(FriendsProgramError::WrongRequestData.into());
        }

        if request_to_from.from != friend_info_from.user
            || request_to_from.to != friend_info_to.user
        {
            return Err(FriendsProgramError::WrongRequestData.into());
        }

        if last_request_from_to.from != friend_info_from.user {
            return Err(FriendsProgramError::WrongRequestData.into());
        }

        if last_request_to_from.to != friend_info_to.user {
            return Err(FriendsProgramError::WrongRequestData.into());
        }

        if friend_info_from.user != *user_from_account_info.key || !user_from_account_info.is_signer {
            return Err(ProgramError::MissingRequiredSignature);
        }

        Self::remove_request(
            &mut request_from_to,
            &request_from_to_account_info,
            &mut request_to_from,
            &request_to_from_account_info,
            &mut last_request_from_to,
            &last_request_from_to_account_info,
            &mut last_request_to_from,
            &last_request_to_from_account_info,
            &friend_info_from,
            &friend_info_to,
            program_id,
        )?;

        friend_info_from.requests_outgoing =
            friend_info_from
                .requests_outgoing
                .checked_sub(1)
                .ok_or::<ProgramError>(FriendsProgramError::CalculationError.into())?;

        friend_info_to.requests_incoming =
            friend_info_to
                .requests_incoming
                .checked_sub(1)
                .ok_or::<ProgramError>(FriendsProgramError::CalculationError.into())?;

        friend_info_from.serialize(&mut *friend_info_from_account_info.data.borrow_mut())?;
        friend_info_to
            .serialize(&mut *friend_info_to_account_info.data.borrow_mut())
            .map_err(|e| e.into())
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
        let instruction = FriendsInstruction::try_from_slice(input)?;
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
