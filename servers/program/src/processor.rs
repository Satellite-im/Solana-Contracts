//! Program state processor

use super::borsh::*;
use crate::{
    borsh::{AccountWithBorsh, BorshSerializeConst},
    error::Error,
    instruction::*,
    program::create_index_with_seed,
    state::*,
};
use solana_program::{
    account_info::AccountInfo, entrypoint::ProgramResult, msg, nonce::State,
    program_error::ProgramError, pubkey::Pubkey, rent::Rent, sysvar::Sysvar,
};

use super::prelude::*;

/// Program state handler.
pub struct Processor {}
impl Processor {
    fn initialize_dweller<'a>(
        _program_id: &Pubkey,
        dweller: &AccountInfo<'a>,
        input: &InitializeDwellerInput,
    ) -> ProgramResult {
        let mut data = dweller.try_borrow_mut_data()?;
        let mut state = Dweller::deserialize_const(&data)?;
        if state.version == StateVersion::Uninitialized {
            state.version = StateVersion::V1;
            state.name = input.name.clone();
            state.serialize_const(&mut data)?;
            Ok(())
        } else {
            Err(ProgramError::AccountAlreadyInitialized)
        }
    }

    fn set_dweller_name<'a>(
        _program_id: &Pubkey,
        dweller: &AccountInfo<'a>,
        input: &SetNameInput,
    ) -> ProgramResult {
        if dweller.is_signer {
            let mut data = dweller.try_borrow_mut_data()?;
            let mut state = Dweller::deserialize_const(&data)?;
            if state.version == StateVersion::V1 {
                state.version = StateVersion::V1;
                state.name = input.name.clone();
                state.serialize_const(&mut data)?;
                Ok(())
            } else {
                Err(ProgramError::UninitializedAccount)
            }
        } else {
            Err(ProgramError::MissingRequiredSignature.into())
        }
    }

    fn set_dweller_photo<'a>(
        _program_id: &Pubkey,
        dweller: &AccountInfo<'a>,
        input: &SetHashInput,
    ) -> ProgramResult {
        if dweller.is_signer {
            let mut data = dweller.try_borrow_mut_data()?;
            let mut state = Dweller::deserialize_const(&data)?;
            if state.version == StateVersion::V1 {
                state.version = StateVersion::V1;
                state.photo_hash = input.hash.clone();
                state.serialize_const(&mut data)?;
                Ok(())
            } else {
                Err(ProgramError::UninitializedAccount)
            }
        } else {
            Err(ProgramError::MissingRequiredSignature.into())
        }
    }

    fn set_dweller_status<'a>(
        _program_id: &Pubkey,
        dweller: &AccountInfo<'a>,
        input: &SetDwellerStatusInput,
    ) -> ProgramResult {
        if dweller.is_signer {
            let mut data = dweller.try_borrow_mut_data()?;
            let mut state = Dweller::deserialize_const(&data)?;
            if state.version == StateVersion::V1 {
                state.version = StateVersion::V1;
                state.status = input.status.clone();
                state.serialize_const(&mut data)?;
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
            let server_member_key =
                create_index_with_seed(program_id, b"Server", server.key, server_state.members)?;

            let dweller_server_key = create_index_with_seed(
                program_id,
                b"Dweller",
                dweller_owner.key,
                dweller_state.servers,
            )?;

            if *server_member.key == server_member_key && dweller_server_key == *dweller_server.key
            {
                let mut server_member_data = server_member.try_borrow_mut_data()?;
                let mut server_member_state = ServerMember::deserialize_const(&server_member_data)?;

                server_member_state.version = StateVersion::V1;
                server_member_state.container = *server.key;
                server_member_state.dweller = *dweller_owner.key;
                server_state.owner = *dweller_owner.key;

                let mut dweller_server_data = dweller_server.try_borrow_mut_data()?;
                let mut dweller_server_state =
                    DwellerServer::deserialize_const(&dweller_server_data)?;

                dweller_server_state.version = StateVersion::V1;
                dweller_server_state.server = *server.key;
                dweller_server_state.container = *dweller_owner.key;

                server_state.members = server_state.members.error_increment()?;

                server_state.name = input.name.clone();

                dweller_state.servers = dweller_state.servers.error_increment()?;

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

            let channel_key =
                create_index_with_seed(program_id, b"Server", server.key, server_state.channels)?;

            if channel_key == *server_channel.key {
                let server_administrator_state =
                    server_administrator.read_data_with_borsh::<ServerAdministrator>()?;
                if server_administrator_state.dweller == *dweller.key {
                    let administrator_member_key = create_index_with_seed(
                        program_id,
                        b"Server",
                        server.key,
                        server_administrator_state.index,
                    )?;
                    if administrator_member_key == *server_administrator.key {
                        let mut channel_data = server_channel.try_borrow_mut_data()?;
                        let mut channel_state = ServerChannel::deserialize_const(&channel_data)?;
                        let channel_member_key = create_index_with_seed(
                            program_id,
                            b"Server",
                            server.key,
                            server_state.channels,
                        )?;

                        if channel_member_key == *server_channel.key {
                            channel_state.version = StateVersion::V1;
                            channel_state.container = *server.key;
                            channel_state.name = input.name.clone();
                            server_state.channels = server_state.channels.error_increment()?;

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

            let group_key =
                create_index_with_seed(program_id, b"Server", server.key, server_state.groups)?;

            if group_key == *server_group.key {
                let server_administrator_state: ServerAdministrator =
                    server_administrator.read_data_with_borsh()?;
                if server_administrator_state.dweller == *dweller.key {
                    let administrator_member_key = create_index_with_seed(
                        program_id,
                        b"Server",
                        server.key,
                        server_administrator_state.index,
                    )?;
                    if administrator_member_key == *server_administrator.key {
                        let mut group_data = server_group.try_borrow_mut_data()?;
                        let mut group_state = ServerGroup::deserialize_const(&group_data)?;
                        let channel_member_key = create_index_with_seed(
                            program_id,
                            b"Server",
                            server.key,
                            server_state.groups,
                        )?;

                        if channel_member_key == *server_group.key {
                            group_state.container = *server.key;
                            group_state.name = input.name.clone();
                            group_state.version = StateVersion::V1;
                            server_state.groups = server_state.groups.error_increment()?;

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

            let administrator_key = create_index_with_seed(
                program_id,
                b"Server",
                server.key,
                server_state.administrators,
            )?;

            if administrator_key == *server_administrator.key {
                let mut server_administrator_data = server_administrator.try_borrow_mut_data()?;
                let mut server_administrator_state =
                    ServerAdministrator::deserialize_const(&server_administrator_data)?;
                if server_administrator_state.dweller == *dweller.key {
                    server_administrator_state.container = *server.key;
                    server_administrator_state.version = StateVersion::V1;
                    server_state.administrators = server_state.administrators.error_increment()?;

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

    fn invite_to_server<'a>(
        program_id: &Pubkey,
        server: &AccountInfo<'a>,
        dweller_admin: &AccountInfo<'a>,
        server_administrator: &AccountInfo<'a>,
        dweller: &AccountInfo<'a>,
        member_status: &AccountInfo<'a>,
    ) -> ProgramResult {
        if dweller_admin.is_signer {
            let server_administrator_state =
                server_administrator.read_data_with_borsh::<ServerAdministrator>()?;

            let administrator_key = create_index_with_seed(
                program_id,
                b"Server",
                server.key,
                server_administrator_state.index,
            )?;

            if administrator_key == *server_administrator.key {
                let mut server_data = server.try_borrow_mut_data()?;
                let mut server_state = Server::deserialize_const(&server_data)?;

                let member_status_key = create_index_with_seed(
                    program_id,
                    b"Server",
                    server.key,
                    server_state.member_statuses,
                )?;

                if member_status_key == *member_status.key {
                    let member_status_data = member_status.try_borrow_mut_data()?;
                    let mut member_status_state =
                        ServerMemberStatus::deserialize_const(&member_status_data)?;

                    member_status_state.container = *server.key;
                    member_status_state.version = StateVersion::V1;
                    member_status_state.dweller = *dweller.key;
                    member_status_state.invited = true;
                    server_state.member_statuses =
                        server_state.member_statuses.error_increment()?;

                    member_status_state.index = server_state.member_statuses;

                    server_state.serialize_const(&mut server_data)?;

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

    /// Create derived address
    pub fn create_derived_address<'a>(
        program_id: &Pubkey,
        payer_account_info: &AccountInfo<'a>,
        owner_account_info: &AccountInfo<'a>,
        base_account_info: &AccountInfo<'a>,
        account_to_create_info: &AccountInfo<'a>,
        rent_account_info: &AccountInfo<'a>,
        _system_program: &AccountInfo<'a>,
        input: &AddressTypeInput,
    ) -> ProgramResult {
        let rent = &Rent::from_account_info(rent_account_info)?;
        match input {
            AddressTypeInput::DwellerServer(index) => {
                msg!("dw");
                let (program_address, bump_seed) = Pubkey::find_program_address(
                    &[&owner_account_info.key.to_bytes()[..32]],
                    program_id,
                );
                if program_address != *base_account_info.key {
                    return Err(ProgramError::InvalidSeeds);
                }

                let address_to_create =
                    Pubkey::create_with_seed(&program_address, DwellerServer::SEED, program_id)?;

                if address_to_create != *account_to_create_info.key {
                    return Err(ProgramError::InvalidSeeds);
                }
                let signature = &[&owner_account_info.key.to_bytes()[..32], &[bump_seed]];
                msg!("DSAAD");
                crate::program::create_derived_account(
                    payer_account_info.clone(),
                    account_to_create_info.clone(),
                    base_account_info.clone(),
                    DwellerServer::SEED,
                    rent.minimum_balance(DwellerServer::LEN as usize),
                    DwellerServer::LEN as u64,
                    program_id,
                    signature,
                )?;
                msg!("adsasdasd");
            }
            _ => todo!(),
        }
        Ok(())
    }

    /// Processes an instruction
    pub fn process_instruction(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        input: &[u8],
    ) -> ProgramResult {
        let instruction = Instruction::try_from_slice(&input[0..1])?;
        match instruction {
            Instruction::CreateDerivedAccount => {
                msg!("Instruction: CreateDerivedAccount");
                match accounts {
                    [payer_account_info, owner_account_info, base_account_info, account_to_create_info, rent_account_info, system_program, ..] =>
                    {
                        msg!("Got");
                        let input =
                            super::instruction::AddressTypeInput::deserialize_const(&input[1..])?;
                        msg!("Akk");
                        Self::create_derived_address(
                            program_id,
                            payer_account_info,
                            owner_account_info,
                            base_account_info,
                            account_to_create_info,
                            rent_account_info,
                            system_program,
                            &input,
                        )
                    }
                    _ => Err(ProgramError::NotEnoughAccountKeys),
                }
            }
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
                        Self::add_admin(program_id, owner, dweller, server, server_administrator)
                    }
                    _ => Err(ProgramError::NotEnoughAccountKeys),
                }
            }
            Instruction::InviteToServer => {
                msg!("Instruction: InviteToServer");
                match accounts {
                    [server, dweller_admin, server_administrator, dweller, member_status, ..] => {
                        Self::invite_to_server(
                            program_id,
                            server,
                            dweller_admin,
                            server_administrator,
                            dweller,
                            member_status,
                        )
                    }
                    _ => Err(ProgramError::NotEnoughAccountKeys),
                }
            }
            _ => todo!(),
        }
    }
}
