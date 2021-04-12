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
    ///   - writeable  dweller
    ///   - writeable  dweller_server
    ///   - writeable  server_member
    ///   - writable   server 
    JoinServer,
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
