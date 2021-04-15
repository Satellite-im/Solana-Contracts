//! Program entrypoint definitions

#![cfg(all(target_arch = "bpf", not(feature = "no-entrypoint")))]

use crate::{error::Error, processor};
use solana_program::{
    account_info::AccountInfo, entrypoint, entrypoint::ProgramResult,
    program_error::PrintProgramError, pubkey::Pubkey,
};

entrypoint!(process_instruction);
fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    if let Err(error) =
        processor::Processor::process_instruction(program_id, accounts, instruction_data)
    {
        error.print::<Error>();
        return Err(error);
    }
    Ok(())
}
