//! Program state processor

use crate::{
    error::NftError,
    instruction::{MintData, NftInstruction, TokenDataArgs},
    state::{Mint, MintVersion, Token, TokenDataStatus, TokenStatus},
};
use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::AccountInfo, entrypoint::ProgramResult, program_error::ProgramError,
    pubkey::Pubkey, rent::Rent, sysvar::Sysvar,
};

/// Program state handler.
pub struct Processor {}
impl Processor {
    /// Seed
    pub const TOKEN_DATA_SEED: &'static str = "token_data";

    /// Initialize the mint
    pub fn process_initialize_mint(
        program_id: &Pubkey,
        mint: &AccountInfo,
        data: MintData,
        rent: &AccountInfo,
        authority: &AccountInfo,
    ) -> ProgramResult {
        // validate
        if data.symbol == [0; 8] || data.name.is_empty() {
            return Err(ProgramError::InvalidInstructionData);
        }

        validate_program(&program_id, &mint)?;

        if !authority.is_signer {
            return Err(ProgramError::MissingRequiredSignature);
        }

        if !Rent::from_account_info(rent)?.is_exempt(mint.lamports(), Mint::LEN as usize) {
            return Err(ProgramError::AccountNotRentExempt);
        }

        let mut mint_data = mint.try_borrow_mut_data()?;
        if Mint::LEN != mint_data.len() as u64 {
            return Err(ProgramError::InvalidAccountData);
        }

        let mut mint_state = Mint::try_from_slice(&mint_data)?;
        if mint_state.version != MintVersion::Uninitialized {
            return Err(ProgramError::AccountAlreadyInitialized);
        }

        // update
        mint_state.symbol = data.symbol;
        mint_state.name = data.name;
        mint_state.authority = *authority.key;

        mint_state.serialize(&mut mint_data.as_mut())?;
        Ok(())
    }

    #[allow(clippy::too_many_arguments)]
    pub fn process_initialize_token(
        program_id: &Pubkey,
        token_account: &AccountInfo,
        token_data_account: &AccountInfo,
        mint: &AccountInfo,
        data: TokenDataArgs,
        owner: &AccountInfo,
        rent: &AccountInfo,
        mint_authority: &AccountInfo,
    ) -> ProgramResult {
        validate_program(&program_id, &token_account)?;
        validate_program(&program_id, &token_data_account)?;
        validate_program(&program_id, &mint)?;

        let mint_acc_data = Mint::try_from_slice(&mint.data.borrow())?;

        if !mint_authority.is_signer || mint_acc_data.authority != *mint_authority.key {
            return Err(ProgramError::MissingRequiredSignature);
        }

        let rent = Rent::from_account_info(rent)?;
        if !rent.is_exempt(token_account.lamports(), Token::LEN as usize)
            || !rent.is_exempt(
                token_data_account.lamports(),
                super::state::TokenData::LEN as usize,
            )
        {
            return Err(ProgramError::AccountNotRentExempt);
        }

        let generated_token_data_key =
            Pubkey::create_with_seed(token_account.key, Self::TOKEN_DATA_SEED, program_id)?;

        if generated_token_data_key != *token_data_account.key {
            return Err(ProgramError::InvalidSeeds);
        }

        {
            // there are several patters which can be optimized:
            // 1. read(simple deref/drop wrapper)
            // 2. read/write (return deref wrapper which is drop on serialize call)
            // so there will be no explicit (de)serialize (like several other places in solana with methods like `.._with_borsh`)
            // fn from_account<T: BorshDeserialize + BorshSerialize>(v: &[u8]) -> Result<T, ProgramError> {
            //     let mut v_mut = v;
            //     Ok(T::deserialize(&mut v_mut)?)
            // }
            let mut token_data = token_account.try_borrow_mut_data()?;
            let mut data = *token_data as &[u8];
            let mut token_state = Token::deserialize(&mut data)?;

            if token_state.version != TokenStatus::Uninitialized {
                return Err(ProgramError::AccountAlreadyInitialized);
            }

            token_state.mint = *mint.key;
            token_state.owner = *owner.key;
            token_state.version = TokenStatus::Initialized;
            token_state.serialize(&mut token_data.as_mut())?;
        }

        {
            let mut token_data_data = token_data_account.try_borrow_mut_data()?;
            let mut data_data = *token_data_data as &[u8];
            let mut token_data_state = super::state::TokenData::deserialize(&mut data_data)?;

            if token_data_state.version != TokenDataStatus::Uninitialized {
                return Err(ProgramError::AccountAlreadyInitialized);
            }

            let url = String::from_utf8(data.uri.to_vec())
                .map_err(|_| ProgramError::InvalidInstructionData)?;
            let _ = url::Url::parse(&url).map_err(|_| ProgramError::InvalidInstructionData)?;

            token_data_state.hash = data.hash;
            token_data_state.uri = data.uri;
            token_data_state.token = *token_account.key;
            token_data_state.version = TokenDataStatus::Initialized;

            token_data_state.serialize(&mut token_data_data.as_mut())?;
        }

        Ok(())
    }

    pub fn process_transfer_token(
        program_id: &Pubkey,
        token_account: &AccountInfo,
        new_owner: &AccountInfo,
        signer: &AccountInfo,
    ) -> ProgramResult {
        validate_program(&program_id, &token_account)?;

        if !signer.is_signer {
            return Err(ProgramError::MissingRequiredSignature);
        }

        {
            let mut token_data = token_account.try_borrow_mut_data()?;
            let mut data = *token_data as &[u8];
            let mut token_state = Token::deserialize(&mut data)?;
            if token_state.version != TokenStatus::Initialized {
                return Err(ProgramError::InvalidAccountData);
            }

            if token_state.owner == *signer.key
                || (token_state.approval.is_some() && token_state.approval.unwrap() == *signer.key)
            {
                token_state.owner = *new_owner.key;
                token_state.approval = None;
                token_state.serialize(&mut token_data.as_mut())?;
            } else {
                return Err(NftError::SignerNotOwnerOrApproval.into());
            }
        }

        Ok(())
    }

    pub fn process_approve(
        program_id: &Pubkey,
        token_account: &AccountInfo,
        new_approval: &AccountInfo,
        signer: &AccountInfo,
    ) -> ProgramResult {
        validate_program(&program_id, &token_account)?;

        if !signer.is_signer {
            return Err(ProgramError::MissingRequiredSignature);
        }

        {
            let mut token_data = token_account.try_borrow_mut_data()?;
            let mut data = *token_data as &[u8];
            let mut token_state = Token::deserialize(&mut data)?;

            if token_state.version != TokenStatus::Initialized {
                return Err(ProgramError::InvalidAccountData);
            }

            if token_state.owner == *signer.key
                || (token_state.approval.is_some() && token_state.approval.unwrap() == *signer.key)
            {
                token_state.approval = Some(*new_approval.key);
                token_state.serialize(&mut token_data.as_mut())?;
            } else {
                return Err(NftError::SignerNotOwnerOrApproval.into());
            }
        }

        Ok(())
    }

    pub fn process_burn(
        program_id: &Pubkey,
        token_account: &AccountInfo,
        token_data_account: &AccountInfo,
        signer: &AccountInfo,
    ) -> ProgramResult {
        validate_program(&program_id, &token_account)?;

        if !signer.is_signer {
            return Err(ProgramError::MissingRequiredSignature);
        }

        {
            let token_data = token_account.try_borrow_mut_data()?;
            let mut data = *token_data as &[u8];
            let token_state = Token::deserialize(&mut data)?;

            if token_state.version != TokenStatus::Initialized {
                return Err(ProgramError::InvalidAccountData);
            }

            let token_data_data = token_data_account.try_borrow_mut_data()?;
            let mut data_data = *token_data_data as &[u8];
            let token_data_state = crate::state::TokenData::deserialize(&mut data_data)?;
            if (token_state.owner == *signer.key
                || (token_state.approval.is_some() && token_state.approval.unwrap() == *signer.key))
                && token_data_state.token == *token_account.key
            {
                let lamports = token_account.try_borrow_mut_lamports()?;
                transfer_lamports(signer, lamports)?;

                let lamports = token_data_account.try_borrow_mut_lamports()?;
                transfer_lamports(signer, lamports)?;
            } else {
                return Err(NftError::SignerNotOwnerOrApproval.into());
            }
        }

        Ok(())
    }

    /// Processes an instruction
    pub fn process_instruction(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        input: &[u8],
    ) -> ProgramResult {
        let instruction =
            NftInstruction::try_from_slice(input).or(Err(ProgramError::InvalidInstructionData))?;
        match instruction {
            NftInstruction::InitializeMint(data) => {
                if let [mint, rent, signer, ..] = accounts {
                    Self::process_initialize_mint(program_id, mint, data, rent, signer)
                } else {
                    Err(ProgramError::NotEnoughAccountKeys)
                }
            }
            NftInstruction::InitializeToken(data) => {
                if let [token, token_data, mint, rent, owner, signer, ..] = accounts {
                    Self::process_initialize_token(
                        program_id, token, token_data, mint, data, owner, rent, signer,
                    )
                } else {
                    Err(ProgramError::NotEnoughAccountKeys)
                }
            }
            NftInstruction::Transfer => {
                if let [token, new_owner, signer, ..] = accounts {
                    Self::process_transfer_token(program_id, token, new_owner, signer)
                } else {
                    Err(ProgramError::NotEnoughAccountKeys)
                }
            }
            NftInstruction::Approve => {
                if let [token, new_approval, signer, ..] = accounts {
                    Self::process_approve(program_id, token, new_approval, signer)
                } else {
                    Err(ProgramError::NotEnoughAccountKeys)
                }
            }
            NftInstruction::Burn => {
                if let [token, token_data, signer, ..] = accounts {
                    Self::process_burn(program_id, token, token_data, signer)
                } else {
                    Err(ProgramError::NotEnoughAccountKeys)
                }
            }
        }
    }
}

fn transfer_lamports(
    signer: &AccountInfo,
    mut lamports: std::cell::RefMut<&mut u64>,
) -> Result<(), ProgramError> {
    let mut thanks_for_cleaning_garbage = signer.try_borrow_mut_lamports()?;
    let value = (**thanks_for_cleaning_garbage)
        .checked_add(**lamports)
        .ok_or(NftError::Overflow)?;
    **thanks_for_cleaning_garbage = value;
    **lamports = 0;
    Ok(())
}

fn validate_program(program_id: &Pubkey, account: &AccountInfo) -> ProgramResult {
    if program_id != account.owner {
        return Err(ProgramError::IncorrectProgramId);
    }

    Ok(())
}
