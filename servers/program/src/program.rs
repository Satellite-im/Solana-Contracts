use solana_program::{account_info::AccountInfo, entrypoint::ProgramResult, msg, nonce::State, program_error::ProgramError, program_pack::Pack, pubkey::{Pubkey, PubkeyError}, rent::Rent, system_instruction, sysvar::Sysvar};

/// implements program seed public key address as indexed list pattern
pub fn create_index_with_seed(
    program_id: &Pubkey,
    type_name: &[u8],
    base: &Pubkey,
    index: u64,
) -> Result<Pubkey, PubkeyError> {
    Pubkey::create_with_seed(
        base,
        &format!("{:?}{:?}", type_name, index),
        &program_id,
    )
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