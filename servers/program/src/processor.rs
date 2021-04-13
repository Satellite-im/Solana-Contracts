//! Program state processor

use super::borsh::*;
use crate::{
    error::Error,
    instruction::{InitializeDwellerInput, Instruction},
    state::*,
};
use solana_program::{
    account_info::AccountInfo, entrypoint::ProgramResult, msg, program_error::ProgramError,
    program_pack::Pack, pubkey::Pubkey, rent::Rent, sysvar::Sysvar,
};

use super::prelude::*;

/// Program state handler.
pub struct Processor {}
impl Processor {
    fn initialize_dweller<'a>(
        program_id: &Pubkey,
        dweller: &AccountInfo<'a>,
        input: &InitializeDwellerInput,
    ) -> ProgramResult {
        let mut data = dweller.try_borrow_mut_data()?;
        let mut state = Dweller::deserialize_const(&data)?;
        if state.version == StateVersion::Uninitialized {
            state.version = StateVersion::V1;
            state.name = input.name.clone();
            state.serialize_const(&mut data);
            Ok(())
        } else {
            Err(ProgramError::AccountAlreadyInitialized)
        }

        // let rent = Rent::from_account_info(rent)?;
        // if rent.is_exempt(asset.lamports(), AssetState::len())
        //     && rent.is_exempt(token.lamports(), Account::LEN)
        // {
        //     let (authority, _) =
        //         Pubkey::find_program_address(&[&asset.key.to_bytes()[..32]], &program_id);
    }

    /// Processes an instruction
    pub fn process_instruction(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        input: &[u8],
    ) -> ProgramResult {
        let instruction = Instruction::try_from_slice(&input[0..1])?;
        match instruction {
            Instruction::InitializeDweller => {
                msg!("Instruction: InitializeDweller");
                match accounts {
                    [dweller, ..] => {
                        let input = super::instruction::InitializeDwellerInput::deserialize_const(
                            &input[1..],
                        )?;

                        Self::initialize_dweller(program_id, dweller, &input)
                    }
                    _ => Err(ProgramError::NotEnoughAccountKeys),
                }
            }
            _ => todo!(),
        }
    }
}
