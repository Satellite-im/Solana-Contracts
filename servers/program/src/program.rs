//! In program helpers

use std::mem;

use borsh::BorshSerialize;
use solana_program::{
    account_info::AccountInfo,
    entrypoint::ProgramResult,
    program_error::ProgramError,
    pubkey::{Pubkey, PubkeyError},
    rent::Rent,
    system_instruction,
};

/// implements program seed public key address as indexed list pattern
/// not optimal calling on chain, could store bump in state
pub fn create_base_index_with_seed(
    program_id: &Pubkey,
    type_name: &str,
    seed_key: &Pubkey,
    index: u64,
) -> Result<(Pubkey, Pubkey, u8, String), PubkeyError> {
    let (base, bump) = Pubkey::find_program_address(&[&seed_key.to_bytes()[..32]], program_id);
    let seed = format!("{}{:?}", type_name, index,);
    Ok((
        Pubkey::create_with_seed(&base, &seed, program_id)?,
        base,
        bump,
        seed,
    ))
}

/// validation shortcut
pub fn create_index_with_seed(
    program_id: &Pubkey,
    type_name: &str,
    seed_key: &Pubkey,
    index: u64,
) -> Result<Pubkey, PubkeyError> {
    let (create, ..) = create_base_index_with_seed(program_id, type_name, seed_key, index)?;
    Ok(create)
}

/// in program invoke to create program signed seeded account
#[allow(clippy::too_many_arguments)]
pub fn create_derived_account<'a>(
    payer: AccountInfo<'a>,
    account_to_create: AccountInfo<'a>,
    base: AccountInfo<'a>,
    seed: &str,
    required_lamports: u64,
    space: u64,
    owner: &Pubkey,
    signer_seeds: &[&[u8]],
) -> ProgramResult {
    solana_program::program::invoke_signed(
        &system_instruction::create_account_with_seed(
            &payer.key,
            &account_to_create.key,
            &base.key,
            seed,
            required_lamports,
            space,
            owner,
        ),
        &[payer.clone(), account_to_create.clone(), base.clone()],
        &[&signer_seeds],
    )
}

/// swaps two accounts data
/// panics if accounts are borrowedy
pub fn swap_accounts<'a, T: Default + BorshSerialize>(
    current: &AccountInfo<'a>,
    last: &AccountInfo<'a>,
) -> Result<(), ProgramError> {
    let mut last_data = last.data.try_borrow_mut().unwrap();
    if current.key != last.key {
        let mut current_data = current.data.try_borrow_mut().unwrap();
        mem::swap(&mut *current_data, &mut *last_data);
    }
    T::default().serialize(&mut *last_data)?;
    Ok(())
}

/// helper to create seeded index collection pattern
#[allow(clippy::too_many_arguments)]
pub fn create_seeded_rent_except_account<'a>(
    seed: &str,
    owner_account_info: &AccountInfo<'a>,
    index: &u64,
    base_account_info: &AccountInfo<'a>,
    account_to_create_info: &AccountInfo<'a>,
    payer_account_info: &AccountInfo<'a>,
    rent: &Rent,
    len: u64,
    program_id: &Pubkey,
) -> Result<(), ProgramError> {
    let (address_to_create, program_address, bump_seed, seed) =
        create_base_index_with_seed(program_id, seed, owner_account_info.key, *index)?;
    if program_address != *base_account_info.key {
        return Err(ProgramError::InvalidSeeds);
    }
    if address_to_create != *account_to_create_info.key {
        return Err(ProgramError::InvalidSeeds);
    }
    let signature = &[&owner_account_info.key.to_bytes()[..32], &[bump_seed]];
    crate::program::create_derived_account(
        payer_account_info.clone(),
        account_to_create_info.clone(),
        base_account_info.clone(),
        &seed,
        rent.minimum_balance(len as usize),
        len as u64,
        program_id,
        signature,
    )?;
    Ok(())
}
