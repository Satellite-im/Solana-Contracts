use solana_program::{
    account_info::AccountInfo,
    entrypoint::ProgramResult,
    msg,
    nonce::State,
    program_error::ProgramError,
    program_pack::Pack,
    pubkey::{Pubkey, PubkeyError},
    rent::Rent,
    sysvar::Sysvar,
};

/// implements program seed public key address as indexed list pattern
pub fn create_index_with_seed(
    program_id: &Pubkey,
    type_name: &[u8],
    owner_id: &Pubkey,
    index: u64,
) -> Result<Pubkey, PubkeyError> {
    Pubkey::create_with_seed(
        owner_id,
        &format!("{:?}{:?}", type_name, index),
        &program_id,
    )
}
