use std::mem;

use borsh::BorshSerialize;
use solana_program::{
    account_info::AccountInfo,
    entrypoint::ProgramResult,
    msg,
    nonce::State,
    program_error::ProgramError,
    program_pack::Pack,
    pubkey::{Pubkey, PubkeyError},
    rent::Rent,
    system_instruction,
    sysvar::Sysvar,
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
    let seed = format!("{:?}{:?}", type_name, index,);
    Ok((
        Pubkey::create_with_seed(&base, &seed, program_id)?,
        base,
        bump,
        seed,
    ))
}

pub fn create_index_with_seed(
    program_id: &Pubkey,
    type_name: &str,
    seed_key: &Pubkey,
    index: u64,
) -> Result<Pubkey, PubkeyError> {
    let (create, ..) = create_base_index_with_seed(program_id, type_name, seed_key, index)?;
    Ok(create)
}

pub fn create_derived_account<'a>(
    funder: AccountInfo<'a>,
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

/// swaps provided member with last, erases last
pub fn swap_last_to_default<T: Default + BorshSerialize>(
    current: &AccountInfo,
    last: &AccountInfo,
) -> Result<(), ProgramError> {
    let mut current_data = last.data.borrow_mut();
    let mut last_data = last.data.borrow_mut();
    if current.key != last.key {
        mem::swap(&mut *current_data, &mut *last_data);
    }
    T::default().serialize(&mut *last_data)?;
    Ok(())
}

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
        create_base_index_with_seed(&crate::id(), seed, owner_account_info.key, *index)?;
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
