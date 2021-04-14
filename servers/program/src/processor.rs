//! Program state processor

use super::borsh::*;
use crate::{borsh::BorshSerializeConst, error::Error, instruction::*, state::*};
use solana_program::{account_info::AccountInfo, entrypoint::ProgramResult, msg, nonce::State, program_error::ProgramError, program_pack::Pack, pubkey::Pubkey, rent::Rent, sysvar::Sysvar};

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
        } else {
            Err(ProgramError::MissingRequiredSignature.into())
        }
    }

    fn set_dweller_photo<'a>(
        program_id: &Pubkey,
        dweller: &AccountInfo<'a>,
        input: &SetHashInput,
    ) -> ProgramResult {
        if dweller.is_signer {
            let mut data = dweller.try_borrow_mut_data()?;
            let mut state = Dweller::deserialize_const(&data)?;
            if state.version == StateVersion::V1 {
                state.version = StateVersion::V1;
                state.photo_hash = input.hash.clone();
                state.serialize_const(&mut data);
                Ok(())
            } else {
                Err(ProgramError::UninitializedAccount)
            }
        } else {
            Err(ProgramError::MissingRequiredSignature.into())
        }
    }

    fn set_dweller_status<'a>(
        program_id: &Pubkey,
        dweller: &AccountInfo<'a>,
        input: &SetDwellerStatusInput,
    ) -> ProgramResult {
        if dweller.is_signer {
            let mut data = dweller.try_borrow_mut_data()?;
            let mut state = Dweller::deserialize_const(&data)?;
            if state.version == StateVersion::V1 {
                state.version = StateVersion::V1;
                state.status = input.status.clone();
                state.serialize_const(&mut data);
                Ok(())
            } else {
                Err(ProgramError::UninitializedAccount)
            }
        } else {
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

    fn add_channel<'a>(
        program_id: &Pubkey,
        dweller: &AccountInfo<'a>,
        server_administrator: &AccountInfo<'a>,
        server: &AccountInfo<'a>,
        server_channel: &AccountInfo<'a>,
        input: &AddChannelInput,
    ) -> ProgramResult {
        if dweller.is_signer {
            let mut server_data = server.try_borrow_mut_data()?;
            let mut server_state = Server::deserialize_const(&server_data)?;

            let channel_key = Pubkey::create_program_address(
                &[
                    &server.key.to_bytes()[..32],
                    &server_state.channels.to_le_bytes()[..8],
                    b"Server",
                ],
                program_id,
            )?;

            if channel_key == *server_channel.key {
                let server_administrator_data = server_administrator.try_borrow_data()?;
                let server_administrator_state =
                    ServerAdministrator::deserialize_const(&server_data)?;
                if server_administrator_state.dweller == *dweller.key {
                    let administrator_member_key = Pubkey::create_program_address(
                        &[
                            &server.key.to_bytes()[..32],
                            &server_administrator_state.index.to_le_bytes()[..8],
                            b"Server",
                        ],
                        program_id,
                    )?;
                    if administrator_member_key == *server_administrator.key {
                        let mut channel_data = server_channel.try_borrow_mut_data()?;
                        let mut channel_state = ServerChannel::deserialize_const(&channel_data)?;
                        let channel_member_key = Pubkey::create_program_address(
                            &[
                                &server.key.to_bytes()[..32],
                                &server_state.channels.to_le_bytes()[..8],
                                b"Server",
                            ],
                            program_id,
                        )?;

                        if channel_member_key == *server_channel.key {
                            channel_state.version = StateVersion::V1;
                            channel_state.server = *server.key;
                            channel_state.name = input.name.clone();
                            server_state.channels =
                                server_state
                                    .channels
                                    .checked_add(1)
                                    .ok_or::<ProgramError>(Error::Overflow.into())?;

                            channel_state.index = server_state.groups;                                        
                            channel_state.serialize_const(&mut channel_data)?;
                            server_state.serialize_const(&mut server_data)?;

                            Ok(())
                        } else {
                            Err(Error::InvalidDerivedAddress.into())
                        }
                    } else {
                        Err(Error::InvalidDerivedAddress.into())
                    }
                } else {
                    Err(Error::InvalidDerivedAddress.into())
                }
            } else {
                Err(Error::InvalidDerivedAddress.into())
            }
        } else {
            Err(ProgramError::MissingRequiredSignature)
        }
    }

    fn create_group<'a>(
        program_id: &Pubkey,
        dweller: &AccountInfo<'a>,
        server_administrator: &AccountInfo<'a>,
        server: &AccountInfo<'a>,
        server_group: &AccountInfo<'a>,
        input: &CreateGroupInput,
    ) -> ProgramResult {
        if dweller.is_signer {
            let mut server_data = server.try_borrow_mut_data()?;
            let mut server_state = Server::deserialize_const(&server_data)?;

            let group_key = Pubkey::create_program_address(
                &[
                    &server.key.to_bytes()[..32],
                    &server_state.groups.to_le_bytes()[..8],
                    b"Server",
                ],
                program_id,
            )?;

            if group_key == *server_group.key {
                let server_administrator_data = server_administrator.try_borrow_data()?;
                let server_administrator_state =
                    ServerAdministrator::deserialize_const(&server_data)?;
                if server_administrator_state.dweller == *dweller.key {
                    let administrator_member_key = Pubkey::create_program_address(
                        &[
                            &server.key.to_bytes()[..32],
                            &server_administrator_state.index.to_le_bytes()[..8],
                            b"Server",
                        ],
                        program_id,
                    )?;
                    if administrator_member_key == *server_administrator.key {
                        let mut group_data = server_group.try_borrow_mut_data()?;
                        let mut group_state = ServerGroup::deserialize_const(&group_data)?;
                        let channel_member_key = Pubkey::create_program_address(
                            &[
                                &server.key.to_bytes()[..32],
                                &server_state.groups.to_le_bytes()[..8],
                                b"Server",
                            ],
                            program_id,
                        )?;

                        if channel_member_key == *server_group.key {
                            group_state.server = *server.key;
                            group_state.name = input.name.clone();
                            group_state.version = StateVersion::V1;
                            server_state.groups =
                                server_state
                                    .groups
                                    .checked_add(1)
                                    .ok_or::<ProgramError>(Error::Overflow.into())?;

                            group_state.index = server_state.groups;                                    
                            group_state.serialize_const(&mut group_data)?;
                            server_state.serialize_const(&mut server_data)?;

                            Ok(())
                        } else {
                            Err(Error::InvalidDerivedAddress.into())
                        }
                    } else {
                        Err(Error::InvalidDerivedAddress.into())
                    }
                } else {
                    Err(Error::InvalidDerivedAddress.into())
                }
            } else {
                Err(Error::InvalidDerivedAddress.into())
            }
        } else {
            Err(ProgramError::MissingRequiredSignature)
        }
    }
    

    fn add_admin<'a>(
        program_id: &Pubkey,
        owner: &AccountInfo<'a>,
        dweller: &AccountInfo<'a>,
        server: &AccountInfo<'a>,
        server_administrator: &AccountInfo<'a>,
    ) -> ProgramResult {
        if owner.is_signer {
            let mut server_data = server.try_borrow_mut_data()?;
            let mut server_state = Server::deserialize_const(&server_data)?;

            let administrator__key = Pubkey::create_program_address(
                &[
                    &server.key.to_bytes()[..32],
                    &server_state.administrators.to_le_bytes()[..8],
                    b"Server",
                ],
                program_id,
            )?;

            if administrator__key == *server_administrator.key {
                let mut server_administrator_data = server_administrator.try_borrow_mut_data()?;
                let mut server_administrator_state =
                    ServerAdministrator::deserialize_const(&server_administrator_data)?;
                if server_administrator_state.dweller == *dweller.key {                    
                    server_administrator_state.server = *server.key;
                    server_administrator_state.version = StateVersion::V1;
                            server_state.administrators =
                                server_state
                                    .administrators
                                    .checked_add(1)
                                    .ok_or::<ProgramError>(Error::Overflow.into())?;

                                    server_administrator_state.index = server_state.administrators;                                    
                                    server_administrator_state.serialize_const(&mut server_administrator_data)?;
                            server_state.serialize_const(&mut server_data)?;

                            Ok(())
                        
                } else {
                    Err(Error::InvalidDerivedAddress.into())
                }
            } else {
                Err(Error::InvalidDerivedAddress.into())
            }
        } else {
            Err(ProgramError::MissingRequiredSignature)
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
                        let input =
                            super::instruction::SetNameInput::deserialize_const(&input[1..])?;

                        Self::set_dweller_name(program_id, dweller, &input)
                    }
                    _ => Err(ProgramError::NotEnoughAccountKeys),
                }
            }
            Instruction::SetDwellerPhoto => {
                msg!("Instruction: SetDwellerPhoto");
                match accounts {
                    [dweller, ..] => {
                        let input =
                            super::instruction::SetHashInput::deserialize_const(&input[1..])?;

                        Self::set_dweller_photo(program_id, dweller, &input)
                    }
                    _ => Err(ProgramError::NotEnoughAccountKeys),
                }
            }
            Instruction::SetDwellerStatus => {
                msg!("Instruction: SetDwellerStatus");
                match accounts {
                    [dweller, ..] => {
                        let input = super::instruction::SetDwellerStatusInput::deserialize_const(
                            &input[1..],
                        )?;

                        Self::set_dweller_status(program_id, dweller, &input)
                    }
                    _ => Err(ProgramError::NotEnoughAccountKeys),
                }
            }
            Instruction::AddChannel => {
                msg!("Instruction: AddChannel");
                match accounts {
                    [dweller, server_administrator, server, server_channel, ..] => {
                        let input =
                            super::instruction::AddChannelInput::deserialize_const(&input[1..])?;

                        Self::add_channel(
                            program_id,
                            dweller,
                            server_administrator,
                            server,
                            server_channel,
                            &input,
                        )
                    }
                    _ => Err(ProgramError::NotEnoughAccountKeys),
                }
            }
            Instruction::CreateGroup => {
                msg!("Instruction: CreateGroup");
                match accounts {
                    [dweller, server_administrator, server, server_group, ..] => {
                        let input =
                            super::instruction::CreateGroupInput::deserialize_const(&input[1..])?;

                        Self::create_group(
                            program_id,
                            dweller,
                            server_administrator,
                            server,
                            server_group,
                            &input,
                        )
                    }
                    _ => Err(ProgramError::NotEnoughAccountKeys),
                }
            }
            Instruction::AddAdmin => {
                msg!("Instruction: AddAdmin");
                match accounts {
                    [owner, dweller, server, server_administrator, ..] => {
                        
                        Self::add_admin(
                            program_id,
                            owner, dweller, server, server_administrator
                        )
                    }
                    _ => Err(ProgramError::NotEnoughAccountKeys),
                }
            }
    
            _ => todo!(),
        }
    }
}
