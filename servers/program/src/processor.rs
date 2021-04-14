//! Program state processor

use super::borsh::*;
use crate::{error::Error, instruction::*, state::*};
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
    }

    fn set_dweller_name<'a>(
        program_id: &Pubkey,
        dweller: &AccountInfo<'a>,
        input: &SetNameInput,
    ) -> ProgramResult {
        if dweller.is_signer {
            let mut data = dweller.try_borrow_mut_data()?;
            let mut state = Dweller::deserialize_const(&data)?;
            if state.version == StateVersion::V1 {
                state.version = StateVersion::V1;
                state.name = input.name.clone();
                state.serialize_const(&mut data);
                Ok(())
            } else {
                Err(ProgramError::UninitializedAccount)
            }    
        }
        else {
            Err(ProgramError::MissingRequiredSignature.into())
        }
    }

    fn initialize_server<'a>(
        program_id: &Pubkey,
        dweller_owner: &AccountInfo<'a>,
        server: &AccountInfo<'a>,
        dweller_server: &AccountInfo<'a>,
        server_member: &AccountInfo<'a>,
        input: &InitializeServerInput,
    ) -> ProgramResult {
        let mut dweller_data = dweller_owner.try_borrow_mut_data()?;
        let mut dweller_state = Dweller::deserialize_const(&dweller_data)?;

        let mut server_data = server.try_borrow_mut_data()?;
        let mut server_state = Server::deserialize_const(&server_data)?;

        if server_state.version == StateVersion::Uninitialized {
            server_state.name = input.name.clone();
            let server_member_key = Pubkey::create_program_address(
                &[
                    &server.key.to_bytes()[..32],
                    &server_state.members.to_le_bytes()[..8],
                    b"Server",
                ],
                program_id,
            )?;

            let dweller_server_key = Pubkey::create_program_address(
                &[
                    &dweller_owner.key.to_bytes()[..32],
                    &dweller_state.servers.to_le_bytes()[..8],
                    b"Dweller",
                ],
                program_id,
            )?;

            if *server_member.key == server_member_key && dweller_server_key == *dweller_server.key
            {
                let mut server_member_data = server_member.try_borrow_mut_data()?;
                let mut server_member_state = ServerMember::deserialize_const(&server_member_data)?;

                server_member_state.version = StateVersion::V1;
                server_member_state.server = *server.key;
                server_member_state.dweller = *dweller_owner.key;
                server_state.owner = *dweller_owner.key;

                let mut dweller_server_data = dweller_server.try_borrow_mut_data()?;
                let mut dweller_server_state =
                    DwellerServer::deserialize_const(&dweller_server_data)?;

                dweller_server_state.version = StateVersion::V1;
                dweller_server_state.server = *server.key;
                dweller_server_state.dweller = *dweller_owner.key;

                server_state.members = server_state
                    .members
                    .checked_add(1)
                    .ok_or::<ProgramError>(Error::Overflow.into())?;

                server_state.name = input.name.clone();

                dweller_state.servers = dweller_state
                    .servers
                    .checked_add(1)
                    .ok_or::<ProgramError>(Error::Overflow.into())?;

                dweller_state.serialize_const(&mut dweller_data)?;
                server_state.serialize_const(&mut server_data)?;
                server_member_state.serialize_const(&mut server_member_data)?;
                dweller_server_state.serialize_const(&mut dweller_server_data)?;

                Ok(())
            } else {
                Err(Error::InvalidDerivedAddress.into())
            }
        } else {
            Err(ProgramError::AccountAlreadyInitialized)
        }
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
            Instruction::InitializeServer => {
                msg!("Instruction: InitializeServer");
                match accounts {
                    [dweller_owner, server, dweller_server, server_member, ..] => {
                        let input = super::instruction::InitializeServerInput::deserialize_const(
                            &input[1..],
                        )?;

                        Self::initialize_server(
                            program_id,
                            dweller_owner,
                            server,
                            dweller_server,
                            server_member,
                            &input,
                        )
                    }
                    _ => Err(ProgramError::NotEnoughAccountKeys),
                }
            }
            Instruction::SetDwellerName => {
                msg!("Instruction: SetDwellerName");
                match accounts {
                    [dweller, ..] => {
                        let input = super::instruction::SetNameInput::deserialize_const(
                            &input[1..],
                        )?;

                        Self::set_dweller_name(
                            program_id,
                            dweller,                            
                            &input,
                        )
                    }
                    _ => Err(ProgramError::NotEnoughAccountKeys),
                }
            }
            _ => todo!(),
        }
    }
}
