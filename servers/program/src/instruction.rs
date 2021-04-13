// //! Instruction types

use borsh::{BorshDeserialize, BorshSchema, BorshSerialize};
use num_derive::ToPrimitive;
use solana_program::{instruction::AccountMeta, program_error::ProgramError, pubkey::Pubkey};

// #[repr(C)]
// #[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug, Clone, BorshSchema)]
// pub struct InitializeAssetInput {
//     pub weight: u64,
// }

/// Instructions
#[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug, Clone, ToPrimitive)]
pub enum Instruction {
    /// Joins dweller to server.
    ///
    /// Inputs:
    ///  InitializeAssetInput
    ///  
    /// Accounts:
    ///   how registry will approve dweller to join?
    ///   - writeable         dweller
    ///   - writeable         dweller_server
    ///   - writeable         server_member
    ///   - writeable          server 
    JoinServer,

    /// Accounts: 
    ///   - signer registry_or_owner
    LeaveServer,

    /// Change dweller's display name
    SetDwellerName,

    /// Change dweller's display photo. Consider using PNG or JPEG photos for usability.
    SetPhoto,

    /// Update the users status
    /// Accounts:
    /// - signer  owner Dweller who ows account 
    SetStatus,

    // - signer  admin
    // - write   channel
    // input:
    // - type_id u8
    // - name  [u8; 32] 
    AddChannel,

    // - signer  admin
    // - write   server
    // - write   channel
    // - write   what about channel group mapping account? reuse place holder with SOL? require more sol?
    DeleteChannel,

    // signer admin
    // write group
    // write [] group_channels
    // input:
    // name [u8;32]
    // 
    CreateGroup,

    AddChannelToGroup,
}

// /// Create `InitializeAsset` instruction
// #[allow(clippy::too_many_arguments)]
// pub fn initialize_asset(
//     rent: &Pubkey,
//     pool: &Pubkey,
//     asset: &Pubkey,
//     token: &Pubkey,
//     input: InitializeAssetInput,
// ) -> Result<solana_program::instruction::Instruction, ProgramError> {
//     let mut data = Instruction::InitializeAsset.try_to_vec()?;
//     let mut input = input.try_to_vec()?;
//     data.append(&mut input);
//     let accounts = vec![
//         AccountMeta::new_readonly(*rent, false),
//         AccountMeta::new_readonly(*pool, false), // makes sure prepare in same transaction
//         AccountMeta::new(*asset, false),
//         AccountMeta::new(*token, false),
//     ];
//     Ok(solana_program::instruction::Instruction {
//         program_id: crate::id(),
//         accounts,
//         data,
//     })
// }

// /// Create `InitializePool` instruction
// #[allow(clippy::too_many_arguments)]
// pub fn initialize_pool(
//     rent: &Pubkey,
//     program_token: &Pubkey,
//     pool: &Pubkey,
//     pool_mint: &Pubkey,
//     assets: &[Pubkey],
// ) -> Result<solana_program::instruction::Instruction, ProgramError> {
//     let data = Instruction::InitializePool.try_to_vec()?;
//     let mut accounts = vec![
//         AccountMeta::new_readonly(*rent, false),
//         AccountMeta::new_readonly(*program_token, false), // makes sure prepare in same transaction
//         AccountMeta::new_readonly(*pool, false),          // makes sure prepare in same transaction
//         AccountMeta::new(*pool_mint, false),
//     ];

//     for asset in assets {
//         accounts.push(AccountMeta::new(*asset, false));
//     }

//     Ok(solana_program::instruction::Instruction {
//         program_id: crate::id(),
//         accounts,
//         data,
//     })
// }
