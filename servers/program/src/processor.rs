//! Program state processor

use solana_program::{
    account_info::AccountInfo, entrypoint::ProgramResult, msg, program_error::ProgramError,
    pubkey::Pubkey, rent::Rent, system_program, sysvar::Sysvar,
};

use super::borsh::*;

use crate::{
    borsh::{AccountWithBorsh, BorshSerializeConst},
    error::Error,
    instruction::*,
    program::{create_index_with_seed, create_seeded_rent_except_account, swap_accounts},
    state::*,
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
            state.name = input.name;
            state.photo_hash = input.hash;
            state.status = input.status;
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
                state.name = input.name;
                state.serialize_const(&mut data)?;
                Ok(())
            } else {
                Err(ProgramError::UninitializedAccount)
            }
        } else {
            Err(ProgramError::MissingRequiredSignature)
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
                state.photo_hash = input.hash;
                state.serialize_const(&mut data)?;
                Ok(())
            } else {
                Err(ProgramError::UninitializedAccount)
            }
        } else {
            Err(ProgramError::MissingRequiredSignature)
        }
    }

    fn set_server_name<'a>(
        program_id: &Pubkey,
        dweller_administrator: &AccountInfo<'a>,
        server_administrator: &AccountInfo<'a>,
        server: &AccountInfo<'a>,
        input: &SetNameInput,
    ) -> ProgramResult {
        require_admin(
            program_id,
            dweller_administrator,
            server,
            server_administrator,
        )?;
        let mut data = server.try_borrow_mut_data()?;
        let mut state = Server::deserialize_const(&data)?;
        if state.version == StateVersion::V1 {
            state.name = input.name;
            state.serialize_const(&mut data)?;
            Ok(())
        } else {
            Err(ProgramError::UninitializedAccount)
        }
    }

    fn set_server_db<'a>(
        program_id: &Pubkey,
        dweller_administrator: &AccountInfo<'a>,
        server_administrator: &AccountInfo<'a>,
        server: &AccountInfo<'a>,
        input: &SetHashInput,
    ) -> ProgramResult {
        require_admin(
            program_id,
            dweller_administrator,
            server,
            server_administrator,
        )?;
        let mut data = server.try_borrow_mut_data()?;
        let mut state = Server::deserialize_const(&data)?;
        if state.version == StateVersion::V1 {
            state.db_hash = input.hash;
            state.serialize_const(&mut data)?;
            Ok(())
        } else {
            Err(ProgramError::UninitializedAccount)
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
                state.status = input.status;
                state.serialize_const(&mut data)?;
                Ok(())
            } else {
                Err(ProgramError::UninitializedAccount)
            }
        } else {
            Err(ProgramError::MissingRequiredSignature)
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
            server_state.name = input.name;
            let server_member_key = create_index_with_seed(
                program_id,
                ServerMember::SEED,
                server.key,
                server_state.members,
            )?;

            let dweller_server_key = create_index_with_seed(
                program_id,
                DwellerServer::SEED,
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
                server_member_state.index = server_state.members;

                server_member_state.serialize_const(&mut server_member_data)?;

                let mut dweller_server_data = dweller_server.try_borrow_mut_data()?;
                let mut dweller_server_state =
                    DwellerServer::deserialize_const(&dweller_server_data)?;

                dweller_server_state.version = StateVersion::V1;
                dweller_server_state.server = *server.key;
                dweller_server_state.container = *dweller_owner.key;
                dweller_server_state.index = dweller_state.servers;
                dweller_server_state.serialize_const(&mut dweller_server_data)?;

                dweller_state.servers = dweller_state.servers.error_increment()?;
                dweller_state.serialize_const(&mut dweller_data)?;

                server_state.version = StateVersion::V1;
                server_state.owner = *dweller_owner.key;
                server_state.members = server_state.members.error_increment()?;
                server_state.name = input.name;
                server_state.serialize_const(&mut server_data)?;

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
        dweller_administrator: &AccountInfo<'a>,
        server_administrator: &AccountInfo<'a>,
        server: &AccountInfo<'a>,
        server_channel: &AccountInfo<'a>,
        input: &AddChannelInput,
    ) -> ProgramResult {
        require_admin(
            program_id,
            dweller_administrator,
            server,
            server_administrator,
        )?;

        let (mut server_data, mut server_state) = server.read_data_with_borsh_mut::<Server>()?;

        let server_channel_key = create_index_with_seed(
            program_id,
            ServerChannel::SEED,
            server.key,
            server_state.channels,
        )?;

        if server_channel_key == *server_channel.key {
            let (mut channel_data, mut channel_state) =
                server_channel.read_data_with_borsh_mut::<ServerChannel>()?;

            channel_state.version = StateVersion::V1;
            channel_state.container = *server.key;
            channel_state.type_id = input.type_id;
            channel_state.name = input.name;
            channel_state.index = server_state.channels;

            server_state.channels = server_state.channels.error_increment()?;

            channel_state.serialize_const(&mut channel_data)?;
            server_state.serialize_const(&mut server_data)?;

            Ok(())
        } else {
            Err(Error::InvalidDerivedServerChannelAddress.into())
        }
    }

    fn create_group<'a>(
        program_id: &Pubkey,
        dweller_administrator: &AccountInfo<'a>,
        server_administrator: &AccountInfo<'a>,
        server: &AccountInfo<'a>,
        server_group: &AccountInfo<'a>,
        input: &CreateGroupInput,
    ) -> ProgramResult {
        require_admin(
            program_id,
            dweller_administrator,
            server,
            server_administrator,
        )?;
        let (mut server_data, mut server_state) = server.read_data_with_borsh_mut::<Server>()?;

        let (mut group_data, mut group_state) =
            server_group.read_data_with_borsh_mut::<ServerGroup>()?;

        let server_group_key = create_index_with_seed(
            program_id,
            ServerGroup::SEED,
            server.key,
            server_state.groups,
        )?;

        if server_group_key == *server_group.key {
            group_state.container = *server.key;
            group_state.name = input.name;
            group_state.version = StateVersion::V1;
            group_state.index = server_state.groups;

            server_state.groups = server_state.groups.error_increment()?;

            group_state.serialize_const(&mut group_data)?;
            server_state.serialize_const(&mut server_data)?;

            Ok(())
        } else {
            Err(Error::InvalidDerivedServerGroupAddress.into())
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
            let (mut server_data, mut server_state) =
                server.read_data_with_borsh_mut::<Server>()?;
            require_owner(&server_state, owner)?;

            let administrator_key = create_index_with_seed(
                program_id,
                ServerAdministrator::SEED,
                server.key,
                server_state.administrators,
            )?;

            if administrator_key == *server_administrator.key {
                let (mut server_administrator_data, mut server_administrator_state) =
                    server_administrator.read_data_with_borsh_mut::<ServerAdministrator>()?;
                if server_administrator_state.version == StateVersion::Uninitialized {
                    server_administrator_state.container = *server.key;
                    server_administrator_state.dweller = *dweller.key;
                    server_administrator_state.version = StateVersion::V1;
                    server_administrator_state.index = server_state.administrators;
                    server_administrator_state.serialize_const(&mut server_administrator_data)?;

                    server_state.administrators = server_state.administrators.error_increment()?;
                    server_state.serialize_const(&mut server_data)?;

                    Ok(())
                } else {
                    Err(ProgramError::AccountAlreadyInitialized)
                }
            } else {
                Err(Error::InvalidDerivedServerAdministratorAddress.into())
            }
        } else {
            Err(ProgramError::MissingRequiredSignature)
        }
    }

    fn remove_admin<'a>(
        _program_id: &Pubkey,
        owner: &AccountInfo<'a>,
        server: &AccountInfo<'a>,
        server_admin: &AccountInfo<'a>,
        server_admin_last: &AccountInfo<'a>,
    ) -> ProgramResult {
        let (mut server_data, mut server_state) = server.read_data_with_borsh_mut::<Server>()?;
        if server_state.owner == *owner.key && owner.is_signer {
            let server_admin_state = server_admin.read_data_with_borsh::<ServerAdministrator>()?;
            let server_admin_key = crate::program::create_index_with_seed(
                &crate::id(),
                ServerAdministrator::SEED,
                server.key,
                server_admin_state.index,
            )?;

            let server_admin_last_key = crate::program::create_index_with_seed(
                &crate::id(),
                ServerAdministrator::SEED,
                server.key,
                server_state.administrators.error_decrement()?,
            )?;

            if server_admin_last_key == *server_admin_last.key
                && server_admin_key == *server_admin.key
            {
                crate::program::swap_accounts::<ServerAdministrator>(
                    server_admin,
                    server_admin_last,
                )?;

                server_state.administrators = server_state.administrators.error_decrement()?;
                server_state.serialize_const(&mut server_data)?;

                Ok(())
            } else {
                Err(Error::InvalidDerivedServerAdministratorAddress.into())
            }
        } else {
            Err(ProgramError::MissingRequiredSignature)
        }
    }

    fn revoke_invite_server<'a>(
        program_id: &Pubkey,
        server: &AccountInfo<'a>,
        dweller_administrator: &AccountInfo<'a>,
        server_administrator: &AccountInfo<'a>,
        member_status: &AccountInfo<'a>,
        member_status_last: &AccountInfo<'a>,
    ) -> ProgramResult {
        require_admin(
            program_id,
            dweller_administrator,
            server,
            server_administrator,
        )?;

        let (mut server_data, mut server_state) = server.read_data_with_borsh_mut::<Server>()?;
        let member_status_state = member_status.read_data_with_borsh::<ServerMemberStatus>()?;

        let member_status_last_key = create_index_with_seed(
            program_id,
            ServerMemberStatus::SEED,
            server.key,
            server_state.member_statuses.error_decrement()?,
        )?;

        let member_status_key = create_index_with_seed(
            program_id,
            ServerMemberStatus::SEED,
            server.key,
            member_status_state.index,
        )?;

        if *member_status.key == member_status_key
            && *member_status_last.key == member_status_last_key
        {
            crate::program::swap_accounts::<ServerMemberStatus>(member_status, member_status_last)?;

            server_state.member_statuses = server_state.member_statuses.error_decrement()?;
            server_state.serialize_const(&mut server_data)?;
            Ok(())
        } else {
            Err(Error::InvalidDerivedServerMemberStatusAddress.into())
        }
    }

    fn invite_to_server<'a>(
        program_id: &Pubkey,
        server: &AccountInfo<'a>,
        dweller_administrator: &AccountInfo<'a>,
        server_administrator: &AccountInfo<'a>,
        dweller: &AccountInfo<'a>,
        member_status: &AccountInfo<'a>,
    ) -> ProgramResult {
        require_admin(
            program_id,
            dweller_administrator,
            server,
            server_administrator,
        )?;

        let (mut server_data, mut server_state) = server.read_data_with_borsh_mut::<Server>()?;

        let member_status_key = create_index_with_seed(
            program_id,
            ServerMemberStatus::SEED,
            server.key,
            server_state.member_statuses,
        )?;

        if member_status_key == *member_status.key {
            let (mut member_status_data, mut member_status_state) =
                member_status.read_data_with_borsh_mut::<ServerMemberStatus>()?;

            member_status_state.container = *server.key;
            member_status_state.version = StateVersion::V1;
            member_status_state.dweller = *dweller.key;
            member_status_state.index = server_state.member_statuses;

            server_state.member_statuses = server_state.member_statuses.error_increment()?;

            server_state.serialize_const(&mut server_data)?;
            member_status_state.serialize_const(&mut member_status_data)?;

            Ok(())
        } else {
            Err(Error::InvalidDerivedServerMemberStatusAddress.into())
        }
    }

    /// Create derived
    #[allow(clippy::too_many_arguments)]
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
            AddressTypeInput::ServerAdministrator(index) => create_seeded_rent_except_account(
                ServerAdministrator::SEED,
                owner_account_info,
                index,
                base_account_info,
                account_to_create_info,
                payer_account_info,
                rent,
                ServerAdministrator::LEN,
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
            AddressTypeInput::GroupChannel(index) => create_seeded_rent_except_account(
                GroupChannel::SEED,
                owner_account_info,
                index,
                base_account_info,
                account_to_create_info,
                payer_account_info,
                rent,
                GroupChannel::LEN,
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
                    [dweller_administrator, server_administrator, server, server_channel, ..] => {
                        let input =
                            super::instruction::AddChannelInput::deserialize_const(&input[1..])?;

                        Self::add_channel(
                            program_id,
                            dweller_administrator,
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
                    [server, dweller_administrator, server_administrator, dweller, member_status, ..] => {
                        Self::invite_to_server(
                            program_id,
                            server,
                            dweller_administrator,
                            server_administrator,
                            dweller,
                            member_status,
                        )
                    }
                    _ => Err(ProgramError::NotEnoughAccountKeys),
                }
            }

            Instruction::RemoveAdmin => {
                msg!("Instruction: RemoveAdmin");
                match accounts {
                    [owner, server, server_admin, server_admin_last, ..] => Self::remove_admin(
                        program_id,
                        owner,
                        server,
                        server_admin,
                        server_admin_last,
                    ),
                    _ => Err(ProgramError::NotEnoughAccountKeys),
                }
            }

            Instruction::RevokeInviteServer => {
                msg!("Instruction: RevokeInviteServer");
                match accounts {
                    [server, dweller_administrator, server_administrator, member_status, member_status_last, ..] => {
                        Self::revoke_invite_server(
                            program_id,
                            server,
                            dweller_administrator,
                            server_administrator,
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
                    [dweller_administrator, server_administrator, server, ..] => {
                        let input =
                            super::instruction::SetNameInput::deserialize_const(&input[1..])?;

                        Self::set_server_name(
                            program_id,
                            dweller_administrator,
                            server_administrator,
                            server,
                            &input,
                        )
                    }
                    _ => Err(ProgramError::NotEnoughAccountKeys),
                }
            }
            Instruction::SetServerDb => {
                msg!("Instruction: SetServerDb");
                match accounts {
                    [dweller_administrator, server_administrator, server, ..] => {
                        let input =
                            super::instruction::SetHashInput::deserialize_const(&input[1..])?;

                        Self::set_server_db(
                            program_id,
                            dweller_administrator,
                            server_administrator,
                            server,
                            &input,
                        )
                    }
                    _ => Err(ProgramError::NotEnoughAccountKeys),
                }
            }
            Instruction::JoinServer => {
                msg!("Instruction: JoinServer");
                match accounts {
                    [server, server_member, server_member_status, dweller, dweller_server, ..] => {
                        Self::join_server(
                            program_id,
                            server,
                            server_member,
                            server_member_status,
                            dweller,
                            dweller_server,
                        )
                    }
                    _ => Err(ProgramError::NotEnoughAccountKeys),
                }
            }

            Instruction::LeaveServer => {
                msg!("Instruction: LeaveServer");
                match accounts {
                    [server, server_member, server_member_last, dweller, dweller_server, dweller_server_last, ..] => {
                        Self::leave_server(
                            program_id,
                            server,
                            server_member,
                            server_member_last,
                            dweller,
                            dweller_server,
                            dweller_server_last,
                        )
                    }
                    _ => Err(ProgramError::NotEnoughAccountKeys),
                }
            }

            Instruction::AddChannelToGroup => {
                msg!("Instruction: AddChannelToGroup");
                match accounts {
                    [server, dweller_administrator, server_administrator, server_channel, server_group, group_channel, ..] => {
                        Self::add_channel_to_group(
                            program_id,
                            server,
                            dweller_administrator,
                            server_administrator,
                            server_channel,
                            server_group,
                            group_channel,
                        )
                    }
                    _ => Err(ProgramError::NotEnoughAccountKeys),
                }
            }

            Instruction::RemoveChannelFromGroup => {
                msg!("Instruction: RemoveChannelFromGroup");
                match accounts {
                    [server, dweller_administrator, server_administrator, server_group, group_channel, group_channel_last, ..] => {
                        Self::remove_channel_from_group(
                            program_id,
                            server,
                            dweller_administrator,
                            server_administrator,
                            server_group,
                            group_channel,
                            group_channel_last,
                        )
                    }
                    _ => Err(ProgramError::NotEnoughAccountKeys),
                }
            }

            Instruction::DeleteChannel => {
                msg!("Instruction: DeleteChannel");
                // ISSUE: in original Solidity contract channels are not deleted from groups
                match accounts {
                    [dweller, server_administrator, server, server_channel, server_channel_last, ..] => {
                        Self::delete_channel(
                            program_id,
                            dweller,
                            server_administrator,
                            server,
                            server_channel,
                            server_channel_last,
                        )
                    }
                    _ => Err(ProgramError::NotEnoughAccountKeys),
                }
            }

            Instruction::DeleteGroup => {
                msg!("Instruction: DeleteGroup");
                match accounts {
                    [dweller_administrator, server_administrator, server, server_group, server_group_last, ..] =>
                    {
                        let group_channels = &accounts[5..];
                        Self::delete_group(
                            program_id,
                            dweller_administrator,
                            server_administrator,
                            server,
                            server_group,
                            server_group_last,
                            group_channels,
                        )
                    }
                    _ => Err(ProgramError::NotEnoughAccountKeys),
                }
            }
        }
    }

    fn delete_group<'a>(
        program_id: &Pubkey,
        dweller_administrator: &AccountInfo<'a>,
        server_administrator: &AccountInfo<'a>,
        server: &AccountInfo<'a>,
        server_group: &AccountInfo<'a>,
        server_group_last: &AccountInfo<'a>,
        group_channels: &[AccountInfo<'a>],
    ) -> ProgramResult {
        require_admin(
            program_id,
            dweller_administrator,
            server,
            server_administrator,
        )?;

        let (mut data, mut server_state) = server.read_data_with_borsh_mut::<Server>()?;

        let server_group_state = server_group.read_data_with_borsh::<ServerGroup>()?;
        let server_group_key = create_index_with_seed(
            program_id,
            ServerGroup::SEED,
            server.key,
            server_group_state.index,
        )?;

        let server_group_last_key = create_index_with_seed(
            program_id,
            ServerGroup::SEED,
            server.key,
            server_state.groups.error_decrement()?,
        )?;

        if server_group_key == *server_group.key && server_group_last_key == *server_group_last.key
        {
            for child in group_channels {
                let child_state = server_group.read_data_with_borsh::<GroupChannel>()?;
                let child_key = create_index_with_seed(
                    program_id,
                    GroupChannel::SEED,
                    &server_group_key,
                    child_state.index,
                )?;

                if child_key == *child.key {
                    swap_accounts::<GroupChannel>(child, child)?;
                } else {
                    return Err(Error::InvalidDerivedGroupChannelAddress.into());
                }
            }

            swap_accounts::<ServerGroup>(server_group, server_group_last)?;

            server_state.groups = server_state.groups.error_decrement()?;
            server_state.serialize_const(&mut data)?;

            return Ok(());
        }

        Err(Error::Failed.into())
    }

    fn delete_channel<'a>(
        program_id: &Pubkey,
        dweller: &AccountInfo<'a>,
        server_administrator: &AccountInfo<'a>,
        server: &AccountInfo<'a>,
        server_channel: &AccountInfo<'a>,
        server_channel_last: &AccountInfo<'a>,
    ) -> ProgramResult {
        require_admin(program_id, dweller, server, server_administrator)?;

        let (mut server_data, mut server_state) = server.read_data_with_borsh_mut::<Server>()?;
        let channel_state = server_channel.read_data_with_borsh::<ServerChannel>()?;

        let server_channel_key = create_index_with_seed(
            program_id,
            ServerChannel::SEED,
            server.key,
            channel_state.index,
        )?;

        let server_channel_last_key = create_index_with_seed(
            program_id,
            ServerChannel::SEED,
            server.key,
            server_state.channels.error_decrement()?,
        )?;

        if server_channel_key == *server_channel.key
            && server_channel_last_key == *server_channel_last.key
        {
            swap_accounts::<ServerChannel>(server_channel, server_channel_last)?;

            server_state.channels = server_state.channels.error_decrement()?;
            server_state.serialize_const(&mut server_data)?;

            Ok(())
        } else {
            Err(Error::InvalidDerivedServerChannelAddress.into())
        }
    }

    #[allow(clippy::too_many_arguments)]
    fn remove_channel_from_group<'a>(
        program_id: &Pubkey,
        server: &AccountInfo<'a>,
        dweller_administrator: &AccountInfo<'a>,
        server_administrator: &AccountInfo<'a>,
        server_group: &AccountInfo<'a>,
        group_channel: &AccountInfo<'a>,
        group_channel_last: &AccountInfo<'a>,
    ) -> ProgramResult {
        require_admin(
            program_id,
            dweller_administrator,
            server,
            server_administrator,
        )?;
        let (mut group_data, mut group_state) =
            server_group.read_data_with_borsh_mut::<ServerGroup>()?;

        let group_channel_data: GroupChannel = group_channel.read_data_with_borsh()?;
        let group_channel_key = create_index_with_seed(
            program_id,
            GroupChannel::SEED,
            server_group.key,
            group_channel_data.index,
        )?;

        let group_channel_last_key = create_index_with_seed(
            program_id,
            GroupChannel::SEED,
            server_group.key,
            group_state.channels.error_decrement()?,
        )?;

        if group_channel_key == *group_channel.key
            && group_channel_last_key == *group_channel_last.key
        {
            swap_accounts::<GroupChannel>(group_channel, group_channel_last)?;

            group_state.channels = group_state.channels.error_decrement()?;
            group_state.serialize_const(&mut group_data)?;

            Ok(())
        } else {
            Err(Error::InvalidDerivedGroupChannelAddress.into())
        }
    }

    fn add_channel_to_group<'a>(
        program_id: &Pubkey,
        server: &AccountInfo<'a>,
        dweller_administrator: &AccountInfo<'a>,
        server_administrator: &AccountInfo<'a>,
        server_channel: &AccountInfo<'a>,
        server_group: &AccountInfo<'a>,
        group_channel: &AccountInfo<'a>,
    ) -> ProgramResult {
        require_admin(
            program_id,
            dweller_administrator,
            server,
            server_administrator,
        )?;

        let (mut server_group_data, mut server_group_state) =
            server_group.read_data_with_borsh_mut::<ServerGroup>()?;

        let group_channel_key = create_index_with_seed(
            program_id,
            GroupChannel::SEED,
            server_group.key,
            server_group_state.channels,
        )?;

        if group_channel_key == *group_channel.key {
            let (mut group_channel_data, mut group_channel_state) =
                group_channel.read_data_with_borsh_mut::<GroupChannel>()?;

            if group_channel_state.version == StateVersion::Uninitialized {
                group_channel_state.version = StateVersion::V1;
                group_channel_state.container = *server_group.key;
                group_channel_state.index = server_group_state.channels;
                group_channel_state.channel = *server_channel.key;
                group_channel_state.serialize_const(&mut group_channel_data)?;

                server_group_state.channels = server_group_state.channels.error_increment()?;
                server_group_state.serialize_const(&mut server_group_data)?;

                Ok(())
            } else {
                Err(ProgramError::AccountAlreadyInitialized)
            }
        } else {
            Err(Error::InvalidDerivedServerChannelAddress.into())
        }
    }

    fn leave_server<'a>(
        program_id: &Pubkey,
        server: &AccountInfo<'a>,
        server_member: &AccountInfo<'a>,
        server_member_last: &AccountInfo<'a>,
        dweller: &AccountInfo<'a>,
        dweller_server: &AccountInfo<'a>,
        dweller_server_last: &AccountInfo<'a>,
    ) -> ProgramResult {
        if dweller.is_signer {
            remove_dweller_server(program_id, dweller, dweller_server, dweller_server_last)?;
            remove_server_member(program_id, server, server_member, server_member_last)?;
            Ok(())
        } else {
            Err(ProgramError::MissingRequiredSignature)
        }
    }

    fn join_server<'a>(
        program_id: &Pubkey,
        server: &AccountInfo<'a>,
        server_member: &AccountInfo<'a>,
        server_member_status: &AccountInfo<'a>,
        dweller: &AccountInfo<'a>,
        dweller_server: &AccountInfo<'a>,
    ) -> ProgramResult {
        if dweller.is_signer {
            let mut dweller_data = dweller.try_borrow_mut_data()?;
            let mut dweller_state = Dweller::deserialize_const(&dweller_data)?;

            let dweller_server_key = create_index_with_seed(
                program_id,
                DwellerServer::SEED,
                dweller.key,
                dweller_state.servers,
            )?;

            if dweller_server_key == *dweller_server.key {
                let (mut dweller_server_data, mut dweller_server_state) =
                    dweller_server.read_data_with_borsh_mut::<DwellerServer>()?;

                if dweller_server_state.version == StateVersion::Uninitialized {
                    let server_member_status_state: ServerMemberStatus =
                        server_member_status.read_data_with_borsh()?;
                    if server_member.owner == program_id
                        && server_member_status_state.dweller == *dweller.key
                        && server_member_status_state.container == *server.key
                    {
                        let (mut server_data, mut server_state) =
                            server.read_data_with_borsh_mut::<Server>()?;
                        let (mut server_member_data, mut server_member_state) =
                            server_member.read_data_with_borsh_mut::<ServerMember>()?;

                        let server_member_key = create_index_with_seed(
                            program_id,
                            ServerMember::SEED,
                            server.key,
                            server_state.members,
                        )?;

                        if server_member_key == *server_member.key {
                            if server_member_state.version == StateVersion::Uninitialized {
                                server_member_state.version = StateVersion::V1;
                                server_member_state.container = *server.key;
                                server_member_state.index = server_state.members;
                                server_member_state.dweller = *dweller.key;
                                server_member_state.serialize_const(&mut server_member_data)?;

                                dweller_server_state.container = *dweller.key;
                                dweller_server_state.index = dweller_state.servers;
                                dweller_server_state.version = StateVersion::V1;
                                dweller_server_state.server = *server.key;
                                dweller_server_state.serialize_const(&mut dweller_server_data)?;

                                dweller_state.servers = dweller_state.servers.error_increment()?;
                                dweller_state.serialize_const(&mut dweller_data)?;

                                server_state.members = server_state.members.error_increment()?;
                                server_state.serialize_const(&mut server_data)?;

                                Ok(())
                            } else {
                                Err(ProgramError::AccountAlreadyInitialized)
                            }
                        } else {
                            Err(Error::InvalidDerivedServerMemberAddress.into())
                        }
                    } else {
                        Err(Error::InvalidDerivedServerMemberStatusAddress.into())
                    }
                } else {
                    Err(ProgramError::AccountAlreadyInitialized)
                }
            } else {
                Err(Error::InvalidDerivedDwellerServerAddress.into())
            }
        } else {
            Err(ProgramError::MissingRequiredSignature)
        }
    }
}

fn require_owner<'a>(server_state: &Server, owner: &AccountInfo<'a>) -> ProgramResult {
    if owner.is_signer {
        if server_state.owner == *owner.key {
            Ok(())
        } else {
            Err(Error::ProvidedDwellerIsNotTheOwnerOfTheServer.into())
        }
    } else {
        Err(ProgramError::MissingRequiredSignature)
    }
}

fn require_admin(
    program_id: &Pubkey,
    dweller_administrator: &AccountInfo,
    server: &AccountInfo,
    server_administrator: &AccountInfo,
) -> ProgramResult {
    if server_administrator.owner != program_id {
        return Err(ProgramError::IncorrectProgramId);
    }

    if dweller_administrator.is_signer {
        let server_administrator_state: ServerAdministrator =
            server_administrator.read_data_with_borsh()?;
        server_administrator_state.is_initialized()?;

        if server_administrator_state.container == *server.key {
            if server_administrator_state.dweller == *dweller_administrator.key {
                Ok(())
            } else {
                Err(Error::InvalidDerivedServerAdministratorAddress.into())
            }
        } else {
            Err(Error::InvalidDerivedAddressWrongServer.into())
        }
    } else {
        Err(ProgramError::MissingRequiredSignature)
    }
}

fn remove_server_member<'a>(
    program_id: &Pubkey,
    server: &AccountInfo<'a>,
    server_member: &AccountInfo<'a>,
    server_member_last: &AccountInfo<'a>,
) -> ProgramResult {
    let (mut server_data, mut server_state) = server.read_data_with_borsh_mut::<Server>()?;

    let server_member_data: GroupChannel = server_member.read_data_with_borsh()?;
    let server_member_key = create_index_with_seed(
        program_id,
        ServerMember::SEED,
        server.key,
        server_member_data.index,
    )?;

    let server_member_last_key = create_index_with_seed(
        program_id,
        ServerMember::SEED,
        server.key,
        server_state.members.error_decrement()?,
    )?;

    if server_member_last_key == *server_member_last.key && server_member_key == *server_member.key
    {
        crate::program::swap_accounts::<ServerMember>(server_member, server_member_last)?;

        server_state.members = server_state.members.error_decrement()?;
        server_state.serialize_const(&mut server_data)?;

        Ok(())
    } else {
        Err(Error::InvalidDerivedServerMemberAddress.into())
    }
}

fn remove_dweller_server<'a>(
    program_id: &Pubkey,
    dweller: &AccountInfo<'a>,
    dweller_server: &AccountInfo<'a>,
    dweller_server_last: &AccountInfo<'a>,
) -> ProgramResult {
    let (mut dweller_data, mut dweller_state) = dweller.read_data_with_borsh_mut::<Dweller>()?;

    let dweller_server_data: DwellerServer = dweller_server.read_data_with_borsh()?;
    let dweller_server_key = create_index_with_seed(
        program_id,
        DwellerServer::SEED,
        dweller.key,
        dweller_server_data.index,
    )?;

    let dweller_server_last_key = create_index_with_seed(
        program_id,
        DwellerServer::SEED,
        dweller.key,
        dweller_state.servers.error_decrement()?,
    )?;

    if dweller_server_key == *dweller_server.key
        && dweller_server_last_key == *dweller_server_last.key
    {
        crate::program::swap_accounts::<DwellerServer>(dweller_server, dweller_server_last)?;

        dweller_state.servers = dweller_state.servers.error_decrement()?;
        dweller_state.serialize_const(&mut dweller_data)?;

        Ok(())
    } else {
        Err(Error::InvalidDerivedDwellerServerAddress.into())
    }
}
