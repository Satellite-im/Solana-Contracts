// //! Instruction types

use borsh::{BorshDeserialize, BorshSchema, BorshSerialize};
use num_derive::ToPrimitive;
use solana_program::{instruction::AccountMeta, program_error::ProgramError, pubkey::Pubkey};

/// Instructions
#[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug, Clone, BorshSchema, ToPrimitive)]
pub enum Instruction {
    /// [initialize_dweller]
    /// accounts
    /// - signer, write     dweller
    ///
    /// Input:
    ///  [InitializeDwellerInput]
    InitializeDweller,

    /// accounts
    /// - signer, write     dweller_owner  will join server during init
    /// - signer, write     server
    /// - write             dweller_server
    /// - write             server_member
    /// Input: [InitializeServerInput]
    InitializeServer,

    /// Change dweller's display name
    ///
    /// Accounts:
    /// - write, signer     dweller
    /// Input: [SetNameInput]
    SetDwellerName,

    /// Change dweller's display photo. Consider using PNG or JPEG photos for usability.
    /// Accounts:
    /// - signer, write   dweller
    ///
    /// Input: [SetHashInput]
    SetDwellerPhoto,

    /// Update the users status
    ///
    /// Accounts:
    /// - signer, write   dweller Dweller who ows account
    ///
    /// Input: [SetDwellerStatusInput]
    SetDwellerStatus,

    /// Initialize channel and add it to server.
    ///
    /// Accounts:
    /// - signer    dweller
    /// - read      server_administrator with dweller
    /// - write     server
    /// - write     channel
    ///
    /// Input:
    /// [AddChannelInput]
    AddChannel,

    /// - signer  admin
    /// - write   server
    /// - write   channel
    /// - write   what about channel group mapping account? reuse place holder with SOL? require more sol?
    /// - write   channel_last
    DeleteChannel,

    /// Accounts:
    /// - write  server
    /// - signer admin
    /// - write  group
    /// Input:
    /// - [CreateGroupInput]
    CreateGroup,

    /// Accounts:
    /// - write     server
    /// - signer    admin
    /// - write     group
    /// - write     group_last
    /// - write     group_channel
    /// - write     group_channel_last
    DeleteGroup,

    /// Accounts:
    /// - write     server
    /// - signer    admin
    /// - write     group_channel
    /// - read      channel
    AddChannelToGroup,

    /// Accounts:
    /// - write     server
    /// - signer    admin
    /// - read      channel
    /// - write     group_channel
    /// - write     group_channel_last
    RemoveChannelFromGroup,

    /// Accounts:
    /// - signer    owner
    /// - write     server
    /// - write     admin
    AddAdmin,

    /// Accounts:
    /// - signer    owner
    /// - write     server
    /// - write     admin
    /// - write     admin_last
    RemoveAdmin,

    ///   - writeable         server     
    ///   - writeable         dweller
    ///   - writeable         dweller_server
    ///   - writeable         member
    JoinServer,

    /// accounts:
    ///
    /// - write    server
    /// - write    member
    /// - write    member_last
    /// - write    dweller_server
    /// - write    dweller_server_last
    LeaveServer,

    /// accounts:
    /// - write     server
    /// - write     member_status
    InviteToServer,

    /// accounts:
    /// - write     server
    /// - write     member_status
    /// - write     member_status_last
    RevokeInviteServer,

    /// accounts:
    /// - signer admin
    /// - write  server
    ///
    /// Input: [SetNameInput]
    SetServerName,

    /// accounts:
    /// - signer admin
    /// - write  server
    ///
    /// Input: [SetHashInput]        
    SetServerDb,
}

#[repr(C)]
#[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug, Clone, BorshSchema)]
pub struct CreateGroupInput {
    pub name: [u8; 32],
}

#[repr(C)]
#[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug, Clone, BorshSchema)]
pub struct AddChannelInput {
    pub type_id: u8,
    pub name: [u8; 32],
}

#[repr(C)]
#[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug, Clone, BorshSchema)]
pub struct SetNameInput {
    pub name: [u8; 32],
}

#[repr(C)]
#[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug, Clone, BorshSchema)]
pub struct SetDwellerStatusInput {
    pub status: [u8; 32],
}

/// IPFS hash
#[repr(C)]
#[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug, Clone, BorshSchema)]
pub struct SetHashInput {
    pub hash: [u8; 64],
}

#[repr(C)]
#[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug, Clone, BorshSchema)]
pub struct InitializeDwellerInput {
    pub name: [u8; 32],
}

#[repr(C)]
#[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug, Clone, BorshSchema)]
pub struct InitializeServerInput {
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
