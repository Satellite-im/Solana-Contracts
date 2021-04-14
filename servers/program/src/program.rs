
use solana_program::{account_info::AccountInfo, entrypoint::ProgramResult, msg, nonce::State, program_error::ProgramError, program_pack::Pack, pubkey::{Pubkey, PubkeyError}, rent::Rent, sysvar::Sysvar};


/// implements program derived address as indexed list pattern
pub fn create_program_index(program_id:&Pubkey, type_name:&[u8], owner_id:Pubkey, index:u64) -> Result<Pubkey, PubkeyError> {
    Pubkey::create_program_address(
    &[
        &owner_id.to_bytes()[..32],
        &index.to_le_bytes()[..8],
        type_name,
    ],
    &program_id)
}

