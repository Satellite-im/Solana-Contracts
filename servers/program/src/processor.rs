//! Program state processor

use std::mem;

use super::borsh::*;
use crate::{
    borsh::{AccountWithBorsh, BorshSerializeConst},
    error::Error,
    instruction::*,
    program::{create_base_index_with_seed, create_index_with_seed},
    state::*,
};
use borsh::{BorshSchema, BorshSerialize};
use solana_program::{
    account_info::AccountInfo, entrypoint::ProgramResult, msg, nonce::State,
    program_error::ProgramError, pubkey::Pubkey, rent::Rent, system_program, sysvar::Sysvar,
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

    fn set_server_name<'a>(
        _program_id: &Pubkey,
        admin: &AccountInfo<'a>,
        server: &AccountInfo<'a>,
        input: &SetNameInput,
    ) -> ProgramResult {
        if admin.is_signer {
            let mut data = server.try_borrow_mut_data()?;
            let mut state = Server::deserialize_const(&data)?;
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

    fn set_server_db<'a>(
        _program_id: &Pubkey,
        admin: &AccountInfo<'a>,
        server: &AccountInfo<'a>,
        input: &SetHashInput,
    ) -> ProgramResult {
        if admin.is_signer {
            let mut data = server.try_borrow_mut_data()?;
            let mut state = Server::deserialize_const(&data)?;
            if state.version == StateVersion::V1 {
                state.version = StateVersion::V1;
                state.db_hash = input.hash.clone();
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
                create_index_with_seed(program_id, "Server", server.key, server_state.members)?;

            let dweller_server_key = create_index_with_seed(
                program_id,
                "Dweller",
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
                create_index_with_seed(program_id, "Server", server.key, server_state.channels)?;

            if channel_key == *server_channel.key {
                let server_administrator_state =
                    server_administrator.read_data_with_borsh::<ServerAdministrator>()?;
                if server_administrator_state.dweller == *dweller.key {
                    let administrator_member_key = create_index_with_seed(
                        program_id,
                        "Server",
                        server.key,
                        server_administrator_state.index,
                    )?;
                    if administrator_member_key == *server_administrator.key {
                        let mut channel_data = server_channel.try_borrow_mut_data()?;
                        let mut channel_state = ServerChannel::deserialize_const(&channel_data)?;
                        let channel_member_key = create_index_with_seed(
                            program_id,
                            "Server",
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
                create_index_with_seed(program_id, "Server", server.key, server_state.groups)?;

            if group_key == *server_group.key {
                let server_administrator_state: ServerAdministrator =
                    server_administrator.read_data_with_borsh()?;
                if server_administrator_state.dweller == *dweller.key {
                    let administrator_member_key = create_index_with_seed(
                        program_id,
                        "Server",
                        server.key,
                        server_administrator_state.index,
                    )?;
                    if administrator_member_key == *server_administrator.key {
                        let mut group_data = server_group.try_borrow_mut_data()?;
                        let mut group_state = ServerGroup::deserialize_const(&group_data)?;
                        let channel_member_key = create_index_with_seed(
                            program_id,
                            "Server",
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
                "Server",
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

    fn remove_admin<'a>(
        program_id: &Pubkey,
        owner: &AccountInfo<'a>,
        server: &AccountInfo<'a>,
        admin: &AccountInfo<'a>,
        admin_last: &AccountInfo<'a>,
    ) -> ProgramResult {
        let mut server_data = server.try_borrow_mut_data()?;
        let mut server_state = Server::deserialize_const(&server_data)?;
        if server_state.owner == *owner.key && owner.is_signer {
            let last_key = crate::program::create_index_with_seed(
                &crate::id(),
                ServerAdministrator::SEED,
                server.key,
                server_state.administrators,
            )?;
            if last_key == *admin_last.key {
                swap_last::<ServerAdministrator>(admin, admin_last)?;
                server_state.administrators = server_state.administrators.error_decrement()?;
                return Ok(());
            }
        }

        return Err(ProgramError::MissingRequiredSignature);
    }

    fn revoke_invite_server<'a>(
        program_id: &Pubkey,
        admin: &AccountInfo<'a>,
        server: &AccountInfo<'a>,
        member_status: &AccountInfo<'a>,
        member_status_last: &AccountInfo<'a>,
    ) -> ProgramResult {
        // TODO: validate admin is admin of the server and signer
        // TODO: validate derived addresses are correct (read to get index etc)

        let mut server_data = server.try_borrow_mut_data()?;
        let mut server_state = Server::deserialize_const(&server_data)?;

        swap_last::<ServerMemberStatus>(member_status, member_status_last)?;
        server_state.member_statuses = server_state.member_statuses.error_decrement()?;

        Ok(())
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
                "Server",
                server.key,
                server_administrator_state.index,
            )?;

            if administrator_key == *server_administrator.key {
                let mut server_data = server.try_borrow_mut_data()?;
                let mut server_state = Server::deserialize_const(&server_data)?;

                let member_status_key = create_index_with_seed(
                    program_id,
                    "Server",
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
        if system_program::id() != *_system_program.key {
            return Err(ProgramError::InvalidSeeds);
        }
        match input {
            AddressTypeInput::DwellerServer(index) => create_seeded_rent_except_account(
                DwellerServer::SEED,
                owner_account_info,
                index,
                base_account_info,
                account_to_create_info,
                payer_account_info,
                rent,
                DwellerServer::LEN,
                program_id,
            ),
            AddressTypeInput::ServerMemberStatus(index) => create_seeded_rent_except_account(
                ServerMemberStatus::SEED,
                owner_account_info,
                index,
                base_account_info,
                account_to_create_info,
                payer_account_info,
                rent,
                ServerMemberStatus::LEN,
                program_id,
            ),
            AddressTypeInput::ServerMember(index) => create_seeded_rent_except_account(
                ServerMember::SEED,
                owner_account_info,
                index,
                base_account_info,
                account_to_create_info,
                payer_account_info,
                rent,
                ServerMember::LEN,
                program_id,
            ),
            AddressTypeInput::ServerChannel(index) => create_seeded_rent_except_account(
                ServerChannel::SEED,
                owner_account_info,
                index,
                base_account_info,
                account_to_create_info,
                payer_account_info,
                rent,
                ServerChannel::LEN,
                program_id,
            ),
            AddressTypeInput::ServerGroup(index) => create_seeded_rent_except_account(
                ServerGroup::SEED,
                owner_account_info,
                index,
                base_account_info,
                account_to_create_info,
                payer_account_info,
                rent,
                ServerGroup::LEN,
                program_id,
            ),
            AddressTypeInput::ServerGroupChannel(index) => create_seeded_rent_except_account(
                ServerGroupChannel::SEED,
                owner_account_info,
                index,
                base_account_info,
                account_to_create_info,
                payer_account_info,
                rent,
                ServerGroupChannel::LEN,
                program_id,
            ),
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
            Instruction::CreateDerivedAccount => {
                msg!("Instruction: CreateDerivedAccount");
                match accounts {
                    [payer_account_info, owner_account_info, base_account_info, account_to_create_info, rent_account_info, system_program, ..] =>
                    {
                        let input =
                            super::instruction::AddressTypeInput::deserialize_const(&input[1..])?;
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

            Instruction::DeleteChannel => {
                msg!("Instruction: DeleteChannel");

                todo!()
            }

            Instruction::DeleteGroup => {
                msg!("Instruction: DeleteGroup");
                todo!()
            }

            Instruction::AddChannelToGroup => todo!(),
            Instruction::RemoveChannelFromGroup => todo!(),
            Instruction::RemoveAdmin => {
                msg!("Instruction: RemoveAdmin");
                match accounts {
                    [owner, server, admin, admin_last, ..] => {
                        Self::remove_admin(program_id, owner, server, admin, admin_last)
                    }
                    _ => Err(ProgramError::NotEnoughAccountKeys),
                }
            }
            Instruction::JoinServer => todo!(),
            Instruction::LeaveServer => todo!(),
            Instruction::RevokeInviteServer => {
                msg!("Instruction: RevokeInviteServer");
                match accounts {
                    [admin, server, member_status, member_status_last, ..] => {
                        Self::revoke_invite_server(
                            program_id,
                            admin,
                            server,
                            member_status,
                            member_status_last,
                        )
                    }
                    _ => Err(ProgramError::NotEnoughAccountKeys),
                }
            }

            Instruction::SetServerName => {
                msg!("Instruction: SetServerName");
                match accounts {
                    [admin, server, ..] => {
                        let input =
                            super::instruction::SetNameInput::deserialize_const(&input[1..])?;

                        Self::set_server_name(program_id, admin, server, &input)
                    }
                    _ => Err(ProgramError::NotEnoughAccountKeys),
                }
            }
            Instruction::SetServerDb => {
                msg!("Instruction: SetServerDb");
                match accounts {
                    [admin, server, ..] => {
                        let input =
                            super::instruction::SetHashInput::deserialize_const(&input[1..])?;

                        Self::set_server_db(program_id, admin, server, &input)
                    }
                    _ => Err(ProgramError::NotEnoughAccountKeys),
                }
            }
        }
    }
}

fn create_seeded_rent_except_account<'a>(
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

/// swaps provided member with last, erases last
fn swap_last<T: Default + BorshSerialize>(
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
