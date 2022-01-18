//! Instruction types

use borsh::{BorshDeserialize, BorshSchema, BorshSerialize};
use num_derive::ToPrimitive;
use solana_program::{
    instruction::AccountMeta, program_error::ProgramError, pubkey::Pubkey, system_program, sysvar,
};

/// Instructions
#[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug, Clone, BorshSchema, ToPrimitive)]
pub enum Instruction {
    /// Create derived account
    ///
    /// Input: [AddressTypeInput]
    CreateDerivedAccount,

    /// [initialize_dweller]
    /// accounts
    /// - signer, write     dweller
    ///
    /// Input:
    ///  [InitializeDwellerInput]
    InitializeDweller,

    /// Initializes server and joins dweller_owner
    /// accounts
    /// - signer,  write          dweller_owner
    /// - signer,  write          server
    /// - derived, write          dweller_server
    /// - derived, write          server_member
    /// Input: [InitializeServerInput]
    InitializeServer,

    /// Change dweller's display name
    ///
    /// Accounts:
    /// - write, signer     dweller
    /// Input: [SetNameInput]
    SetDwellerName,

    /// Change dweller's display photo. Consider using PNG or JPEG photos for usability.
    ///
    /// Accounts:
    /// - signer, write   dweller
    ///
    /// Input: [SetHashInput]
    SetDwellerPhoto,

    /// Update the users status
    ///
    /// Accounts:
    /// - signer, write   dweller owner
    ///
    /// Input: [SetDwellerStatusInput]
    SetDwellerStatus,

    /// Initialize channel and add it to server.
    ///
    /// Accounts:
    /// - signer             dweller_administrator
    /// - read, derived      server_administrator for dweller_administrator
    /// - write              server
    /// - write, derived     server_channel
    ///
    /// Input:
    /// [AddChannelInput]
    AddChannel,

    /// Accounts:
    /// - signer                 dweller_administrator
    /// - read, derived          server_administrator for dweller_administrator
    /// - write                  server
    /// - write, derived         server_channel
    /// - write, derived         server_channel_last
    DeleteChannel,

    /// Initialize group and add to server.
    ///
    /// Accounts:
    /// - signer            dweller_administrator
    /// - read, derived     server_administrator for `dweller_administrator`
    /// - write             server
    /// - write, derived    server_group
    ///
    /// Input:
    /// - [CreateGroupInput]
    CreateGroup,

    /// Accounts:
    /// - signer             dweller_administrator    
    /// - read, derived      server_administrator
    /// - write              server
    /// - write, derived     server_group
    /// - write, derived     server_group_last
    /// - write, derived     [group_channel] all channels in group
    DeleteGroup,

    /// Accounts:
    /// - read, write        server
    /// - read, signer       dweller_administrator
    /// - read, derived      server_administrator
    /// - read               server_channel
    //  - write, derived     server_group
    /// - write, derived     group_channel
    AddChannelToGroup,

    /// Accounts:
    /// - write              server
    /// - signer             dweller_administrator
    /// - read, derived      server_administrator
    /// - read               server_channel
    /// - write, derived     group_channel
    /// - write, derived     group_channel_last
    RemoveChannelFromGroup,

    /// Accounts:
    ///
    /// - signer             owner of server
    /// - read               dweller to become admin
    /// - write              server
    /// - write, derived     server_administrator
    AddAdmin,

    /// Accounts:
    /// - read, signer       owner
    /// - write              server
    /// - write, derived     server_administrator
    /// - write, derived     server_administrator_last
    RemoveAdmin,

    /// Accounts:
    ///   - writeable                  server     
    ///   - writeable, derived         server_member
    ///   - read, derived              server_member_status
    ///   - writeable signer           dweller
    ///   - writeable, derived         dweller_server
    JoinServer,

    /// Accounts:
    ///
    /// - write                      server
    /// - write, derived             server_member
    /// - write, derived             server_member_last
    /// - write, signer              dweller
    /// - write, derived             dweller_server
    /// - write, derived             dweller_server_last
    LeaveServer,

    /// Accounts:
    /// - write                 server
    /// - read, signer          dweller_administrator
    /// - read, derived         server_administrator
    /// - read                  dweller
    /// - write, derived        member_status
    InviteToServer,

    /// Accounts:
    /// - write              server
    /// - read, signer       dweller_administrator
    /// - read, derived      server_administrator
    /// - write, derived     server_member_status
    /// - write, derived     server_member_status_last
    RevokeInviteServer,

    /// Accounts:
    /// - read, signer       dweller_administrator
    /// - read, derived      server_administrator
    /// - write              server
    ///
    /// Input: [SetNameInput]
    SetServerName,

    /// Accounts:
    /// - read, signer       dweller_administrator
    /// - read, derived      server_administrator
    /// - write              server
    ///
    /// Input: [SetHashInput]        
    SetServerDb,
}

/// Address type
#[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug, Clone, BorshSchema)]
pub enum AddressTypeInput {
    /// type
    DwellerServer(u64),
    /// type
    ServerMemberStatus(u64),
    /// type
    ServerMember(u64),
    /// type
    ServerAdministrator(u64),
    /// type
    ServerChannel(u64),
    /// type
    ServerGroup(u64),
    /// type
    GroupChannel(u64),
}

/// instruction data
#[repr(C)]
#[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug, Clone, BorshSchema)]
pub struct CreateGroupInput {
    /// name
    pub name: [u8; 32],
}

/// instruction data
#[repr(C)]
#[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug, Clone, BorshSchema)]
pub struct AddChannelInput {
    /// type
    pub type_id: u8,
    /// name
    pub name: [u8; 32],
}

/// instruction data
#[repr(C)]
#[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug, Clone, BorshSchema)]
pub struct SetNameInput {
    /// name
    pub name: [u8; 32],
}

/// instruction data
#[repr(C)]
#[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug, Clone, BorshSchema)]
pub struct SetDwellerStatusInput {
    /// status
    pub status: [u8; 128],
}

/// instruction data
/// IPFS hash
#[repr(C)]
#[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug, Clone, BorshSchema)]
pub struct SetHashInput {
    /// IPFS
    pub hash: [u8; 64],
}

/// instruction data
#[repr(C)]
#[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug, Clone, BorshSchema)]
pub struct InitializeDwellerInput {
    /// name
    pub name: [u8; 32],
    /// IPFS hash
    pub hash: [u8; 64],
    /// status
    pub status: [u8; 128],
}

/// instruction data
#[repr(C)]
#[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug, Clone, BorshSchema)]
pub struct InitializeServerInput {
    /// name
    pub name: [u8; 32],
}

/// [Instruction::InitializeDweller]
#[allow(clippy::too_many_arguments)]
pub fn initialize_dweller(
    dweller: &Pubkey,
    input: InitializeDwellerInput,
) -> Result<solana_program::instruction::Instruction, ProgramError> {
    let mut data = Instruction::InitializeDweller.try_to_vec()?;
    let mut input = input.try_to_vec()?;
    data.append(&mut input);
    let accounts = vec![AccountMeta::new(*dweller, true)];
    Ok(solana_program::instruction::Instruction {
        program_id: crate::id(),
        accounts,
        data,
    })
}

/// Create [Instruction::CreateDerivedAccount] instruction
pub fn create_derived_account(
    program_id: &Pubkey,
    payer: &Pubkey,
    owner_address: &Pubkey,
    base_program_address: &Pubkey,
    account_to_create: &Pubkey,
    input: AddressTypeInput,
) -> Result<solana_program::instruction::Instruction, ProgramError> {
    let mut data = Instruction::CreateDerivedAccount.try_to_vec()?;
    let mut input = input.try_to_vec()?;
    data.append(&mut input);
    let accounts = vec![
        AccountMeta::new(*payer, true),
        AccountMeta::new_readonly(*owner_address, false),
        AccountMeta::new_readonly(*base_program_address, false),
        AccountMeta::new(*account_to_create, false),
        AccountMeta::new_readonly(sysvar::rent::id(), false),
        AccountMeta::new_readonly(system_program::id(), false),
    ];
    Ok(solana_program::instruction::Instruction {
        program_id: *program_id,
        accounts,
        data,
    })
}

/// [Instruction::InitializeServer]
#[allow(clippy::too_many_arguments)]
pub fn initialize_server(
    dweller_owner: &Pubkey,
    server: &Pubkey,
    dweller_server: &Pubkey,
    server_member: &Pubkey,
    input: InitializeServerInput,
) -> Result<solana_program::instruction::Instruction, ProgramError> {
    let mut data = Instruction::InitializeServer.try_to_vec()?;
    let mut input = input.try_to_vec()?;
    data.append(&mut input);
    let accounts = vec![
        AccountMeta::new(*dweller_owner, true),
        AccountMeta::new(*server, true),
        AccountMeta::new(*dweller_server, false),
        AccountMeta::new(*server_member, false),
    ];

    Ok(solana_program::instruction::Instruction {
        program_id: crate::id(),
        accounts,
        data,
    })
}

/// [Instruction::SetDwellerName]
pub fn set_dweller_name(
    dweller: &Pubkey,
    input: &SetNameInput,
) -> Result<solana_program::instruction::Instruction, ProgramError> {
    let mut data = Instruction::SetDwellerName.try_to_vec()?;
    let mut input = input.try_to_vec()?;
    data.append(&mut input);
    let accounts = vec![AccountMeta::new(*dweller, true)];

    Ok(solana_program::instruction::Instruction {
        program_id: crate::id(),
        accounts,
        data,
    })
}

/// [Instruction::SetDwellerPhoto]
pub fn set_dweller_photo(
    dweller: &Pubkey,
    input: &SetHashInput,
) -> Result<solana_program::instruction::Instruction, ProgramError> {
    let mut data = Instruction::SetDwellerPhoto.try_to_vec()?;
    let mut input = input.try_to_vec()?;
    data.append(&mut input);
    let accounts = vec![AccountMeta::new(*dweller, true)];

    Ok(solana_program::instruction::Instruction {
        program_id: crate::id(),
        accounts,
        data,
    })
}

/// [Instruction::SetDwellerStatus]
pub fn set_dweller_status(
    dweller: &Pubkey,
    input: &SetDwellerStatusInput,
) -> Result<solana_program::instruction::Instruction, ProgramError> {
    let mut data = Instruction::SetDwellerStatus.try_to_vec()?;
    let mut input = input.try_to_vec()?;
    data.append(&mut input);
    let accounts = vec![AccountMeta::new(*dweller, true)];

    Ok(solana_program::instruction::Instruction {
        program_id: crate::id(),
        accounts,
        data,
    })
}

/// [Instruction::AddChannel]
pub fn add_channel(
    dweller_administrator: &Pubkey,
    server_administrator: &Pubkey,
    server: &Pubkey,
    server_channel: &Pubkey,
    input: &AddChannelInput,
) -> Result<solana_program::instruction::Instruction, ProgramError> {
    let mut data = Instruction::AddChannel.try_to_vec()?;
    let mut input = input.try_to_vec()?;
    data.append(&mut input);
    let accounts = vec![
        AccountMeta::new(*dweller_administrator, true),
        AccountMeta::new_readonly(*server_administrator, false),
        AccountMeta::new(*server, false),
        AccountMeta::new(*server_channel, false),
    ];

    Ok(solana_program::instruction::Instruction {
        program_id: crate::id(),
        accounts,
        data,
    })
}

/// [Instruction::DeleteChannel]
pub fn delete_channel(
    dweller_administrator: &Pubkey,
    server_administrator: &Pubkey,
    server: &Pubkey,
    server_channel: &Pubkey,
    server_channel_last: &Pubkey,
) -> Result<solana_program::instruction::Instruction, ProgramError> {
    let data = Instruction::DeleteChannel.try_to_vec()?;
    let accounts = vec![
        AccountMeta::new_readonly(*dweller_administrator, true),
        AccountMeta::new_readonly(*server_administrator, false),
        AccountMeta::new(*server, false),
        AccountMeta::new(*server_channel, false),
        AccountMeta::new(*server_channel_last, false),
    ];

    Ok(solana_program::instruction::Instruction {
        program_id: crate::id(),
        accounts,
        data,
    })
}

/// [Instruction::CreateGroup]
pub fn create_group(
    dweller_administrator: &Pubkey,
    server_administrator: &Pubkey,
    server: &Pubkey,
    server_group: &Pubkey,
    input: &CreateGroupInput,
) -> Result<solana_program::instruction::Instruction, ProgramError> {
    let mut data = Instruction::CreateGroup.try_to_vec()?;
    let mut input = input.try_to_vec()?;
    data.append(&mut input);
    let accounts = vec![
        AccountMeta::new(*dweller_administrator, true),
        AccountMeta::new_readonly(*server_administrator, false),
        AccountMeta::new(*server, false),
        AccountMeta::new(*server_group, false),
    ];

    Ok(solana_program::instruction::Instruction {
        program_id: crate::id(),
        accounts,
        data,
    })
}

/// [Instruction::DeleteGroup]
pub fn delete_group(
    dweller_administrator: &Pubkey,
    server_administrator: &Pubkey,
    server: &Pubkey,
    server_group: &Pubkey,
    server_group_last: &Pubkey,
    group_channels: &[&Pubkey],
) -> Result<solana_program::instruction::Instruction, ProgramError> {
    let data = Instruction::DeleteGroup.try_to_vec()?;
    let mut accounts = vec![
        AccountMeta::new(*dweller_administrator, true),
        AccountMeta::new_readonly(*server_administrator, false),
        AccountMeta::new(*server, false),
        AccountMeta::new(*server_group, false),
        AccountMeta::new(*server_group_last, false),
    ];

    for account in group_channels {
        accounts.push(AccountMeta::new(**account, false));
    }

    Ok(solana_program::instruction::Instruction {
        program_id: crate::id(),
        accounts,
        data,
    })
}

/// [Instruction::AddChannelToGroup]
pub fn add_channel_to_group(
    server: &Pubkey,
    dweller_administrator: &Pubkey,
    server_administrator: &Pubkey,
    server_channel: &Pubkey,
    server_group: &Pubkey,
    group_channel: &Pubkey,
) -> Result<solana_program::instruction::Instruction, ProgramError> {
    let data = Instruction::AddChannelToGroup.try_to_vec()?;
    let accounts = vec![
        AccountMeta::new_readonly(*server, false),
        AccountMeta::new_readonly(*dweller_administrator, true),
        AccountMeta::new_readonly(*server_administrator, false),
        AccountMeta::new_readonly(*server_channel, false),
        AccountMeta::new(*server_group, false),
        AccountMeta::new(*group_channel, false),
    ];

    Ok(solana_program::instruction::Instruction {
        program_id: crate::id(),
        accounts,
        data,
    })
}

/// [Instruction::RemoveChannelFromGroup]
pub fn remove_channel_from_group(
    server: &Pubkey,
    dweller_administrator: &Pubkey,
    server_administrator: &Pubkey,
    server_group: &Pubkey,
    group_channel: &Pubkey,
    group_channel_last: &Pubkey,
) -> Result<solana_program::instruction::Instruction, ProgramError> {
    let data = Instruction::RemoveChannelFromGroup.try_to_vec()?;
    let accounts = vec![
        AccountMeta::new(*server, false),
        AccountMeta::new_readonly(*dweller_administrator, true),
        AccountMeta::new_readonly(*server_administrator, false),
        AccountMeta::new_readonly(*server_group, false),
        AccountMeta::new(*group_channel, false),
        AccountMeta::new(*group_channel_last, false),
    ];

    Ok(solana_program::instruction::Instruction {
        program_id: crate::id(),
        accounts,
        data,
    })
}

/// [Instruction::AddAdmin]
pub fn add_admin(
    owner: &Pubkey,
    dweller: &Pubkey,
    server: &Pubkey,
    server_administrator: &Pubkey,
) -> Result<solana_program::instruction::Instruction, ProgramError> {
    let data = Instruction::AddAdmin.try_to_vec()?;
    let accounts = vec![
        AccountMeta::new(*owner, true),
        AccountMeta::new(*dweller, false),
        AccountMeta::new(*server, false),
        AccountMeta::new(*server_administrator, false),
    ];

    Ok(solana_program::instruction::Instruction {
        program_id: crate::id(),
        accounts,
        data,
    })
}

/// [Instruction::RemoveAdmin]
pub fn remove_admin(
    owner: &Pubkey,
    server: &Pubkey,
    server_administrator: &Pubkey,
    server_administrator_last: &Pubkey,
) -> Result<solana_program::instruction::Instruction, ProgramError> {
    let data = Instruction::RemoveAdmin.try_to_vec()?;
    let accounts = vec![
        AccountMeta::new_readonly(*owner, true),
        AccountMeta::new(*server, false),
        AccountMeta::new(*server_administrator, false),
        AccountMeta::new(*server_administrator_last, false),
    ];

    Ok(solana_program::instruction::Instruction {
        program_id: crate::id(),
        accounts,
        data,
    })
}

/// [Instruction::JoinServer]
pub fn join_server(
    server: &Pubkey,
    server_member: &Pubkey,
    server_member_status: &Pubkey,
    dweller: &Pubkey,
    dweller_server: &Pubkey,
) -> Result<solana_program::instruction::Instruction, ProgramError> {
    let data = Instruction::JoinServer.try_to_vec()?;
    let accounts = vec![
        AccountMeta::new(*server, false),
        AccountMeta::new(*server_member, false),
        AccountMeta::new_readonly(*server_member_status, false),
        AccountMeta::new(*dweller, true),
        AccountMeta::new(*dweller_server, false),
    ];

    Ok(solana_program::instruction::Instruction {
        program_id: crate::id(),
        accounts,
        data,
    })
}

/// [Instruction::LeaveServer]
pub fn leave_server(
    server: &Pubkey,
    server_member: &Pubkey,
    server_member_last: &Pubkey,
    dweller: &Pubkey,
    dweller_server: &Pubkey,
    dweller_server_last: &Pubkey,
) -> Result<solana_program::instruction::Instruction, ProgramError> {
    let data = Instruction::LeaveServer.try_to_vec()?;
    let accounts = vec![
        AccountMeta::new(*server, false),
        AccountMeta::new(*server_member, false),
        AccountMeta::new(*server_member_last, false),
        AccountMeta::new(*dweller, true),
        AccountMeta::new(*dweller_server, false),
        AccountMeta::new(*dweller_server_last, false),
    ];

    Ok(solana_program::instruction::Instruction {
        program_id: crate::id(),
        accounts,
        data,
    })
}

/// [Instruction::InviteToServer]
pub fn invite_to_server(
    server: &Pubkey,
    dweller_administrator: &Pubkey,
    server_administrator: &Pubkey,
    dweller: &Pubkey,
    member_status: &Pubkey,
) -> Result<solana_program::instruction::Instruction, ProgramError> {
    let data = Instruction::InviteToServer.try_to_vec()?;
    let accounts = vec![
        AccountMeta::new(*server, false),
        AccountMeta::new_readonly(*dweller_administrator, true),
        AccountMeta::new_readonly(*server_administrator, false),
        AccountMeta::new_readonly(*dweller, false),
        AccountMeta::new(*member_status, false),
    ];

    Ok(solana_program::instruction::Instruction {
        program_id: crate::id(),
        accounts,
        data,
    })
}

/// [Instruction::RevokeInviteServer]
pub fn revoke_invite_server(
    server: &Pubkey,
    dweller_administrator: &Pubkey,
    server_administrator: &Pubkey,
    server_member_status: &Pubkey,
    server_member_status_last: &Pubkey,
) -> Result<solana_program::instruction::Instruction, ProgramError> {
    let data = Instruction::RevokeInviteServer.try_to_vec()?;
    let accounts = vec![
        AccountMeta::new(*server, false),
        AccountMeta::new_readonly(*dweller_administrator, true),
        AccountMeta::new_readonly(*server_administrator, false),
        AccountMeta::new(*server_member_status, false),
        AccountMeta::new(*server_member_status_last, false),
    ];

    Ok(solana_program::instruction::Instruction {
        program_id: crate::id(),
        accounts,
        data,
    })
}

/// [Instruction::SetServerName]
pub fn set_server_name(
    server: &Pubkey,
    dweller_administrator: &Pubkey,
    server_administrator: &Pubkey,
    input: &SetNameInput,
) -> Result<solana_program::instruction::Instruction, ProgramError> {
    let mut data = Instruction::SetServerName.try_to_vec()?;
    let mut input = input.try_to_vec()?;
    data.append(&mut input);
    let accounts = vec![
        AccountMeta::new(*server, true),
        AccountMeta::new_readonly(*dweller_administrator, true),
        AccountMeta::new_readonly(*server_administrator, true),
    ];

    Ok(solana_program::instruction::Instruction {
        program_id: crate::id(),
        accounts,
        data,
    })
}

/// [Instruction::SetServerDb]
pub fn set_server_db(
    server: &Pubkey,
    dweller_administrator: &Pubkey,
    server_administrator: &Pubkey,
    input: &SetHashInput,
) -> Result<solana_program::instruction::Instruction, ProgramError> {
    let mut data = Instruction::SetServerDb.try_to_vec()?;
    let mut input = input.try_to_vec()?;
    data.append(&mut input);
    let accounts = vec![
        AccountMeta::new(*server, true),
        AccountMeta::new_readonly(*dweller_administrator, true),
        AccountMeta::new_readonly(*server_administrator, true),
    ];

    Ok(solana_program::instruction::Instruction {
        program_id: crate::id(),
        accounts,
        data,
    })
}
