// //! Program state processor
// use borsh::BorshDeserialize;
// use solana_program::{
//     account_info::AccountInfo, entrypoint::ProgramResult, msg, program_error::ProgramError,
//     program_pack::Pack, pubkey::Pubkey, rent::Rent, sysvar::Sysvar,
// };
// use spl_token::state::{Account, Mint};

// use super::borsh::*;
// use crate::{
//     error::PoolError,
//     instruction::{InitializeAssetInput, Instruction},
//     state::*,
// };

// /// Program state handler.
// pub struct Processor {}
// impl Processor {
//     #[allow(clippy::too_many_arguments)]
//     fn initialize_asset<'a>(
//         program_id: &Pubkey,
//         rent: &AccountInfo<'a>,
//         pool: &AccountInfo<'a>,
//         asset: &AccountInfo<'a>,
//         token: &AccountInfo<'a>,
//         input: &InitializeAssetInput,
//     ) -> ProgramResult {
//         let rent = Rent::from_account_info(rent)?;
//         if rent.is_exempt(asset.lamports(), AssetState::len())
//             && rent.is_exempt(token.lamports(), Account::LEN)
//         {
//             let token = token.try_borrow_data()?;
//             let token = spl_token::state::Account::unpack_from_slice(&token[..])?;
//             let (authority, _) =
//                 Pubkey::find_program_address(&[&asset.key.to_bytes()[..32]], &program_id);
//             if token.owner == authority {
//                 let mut state: AssetState = asset.read_data_with_borsh()?;
//                 if state.version == AssetVersion::Uninitialized {
//                     state.version = AssetVersion::InitializedV1;
//                     state.weight = input.weight;
//                     // consider validation that token of specific token program ownership
//                     //state.token = token.key;
//                     state.pool = *pool.key;
//                     let mut data = asset.try_borrow_mut_data()?;
//                     state.serialize_const(&mut data[..])?;
//                     Ok(())
//                 } else {
//                     Err(ProgramError::AccountAlreadyInitialized)
//                 }
//             } else {
//                 Err(PoolError::TokenMustBeUnderAssetAuthority.into())
//             }
//         } else {
//             Err(ProgramError::AccountNotRentExempt)
//         }
//     }

//     #[allow(clippy::too_many_arguments)]
//     fn initialize_pool<'a>(
//         program_id: &Pubkey,
//         rent: &AccountInfo<'a>,
//         _program_token: &AccountInfo<'a>,
//         pool: &AccountInfo<'a>,
//         pool_mint: &AccountInfo<'a>, // to initialize mint we need pool_token mint_to, and default value of tokens in pool. there are 2 options - create account out of chain and pass or on chain; and authority could be pool or not
//         assets: &'_ [AccountInfo<'_>],
//     ) -> ProgramResult {
//         let rent = Rent::from_account_info(rent)?;
//         if rent.is_exempt(pool.lamports(), PoolState::len())
//             && rent.is_exempt(pool_mint.lamports(), Mint::LEN)
//         {
//             let weight_total = {
//                 let assets: Result<Vec<AssetState>, ProgramError> = assets
//                     .iter()
//                     .map(|x| x.read_data_with_borsh::<AssetState>())
//                     .collect();
//                 let assets = assets?;
//                 if assets
//                     .iter()
//                     .any(|x| x.version == AssetVersion::Uninitialized || x.pool != *pool.key)
//                 {
//                     return Err(ProgramError::UninitializedAccount);
//                 }
//                 assets.iter().map(|x| x.weight).sum()
//             };

//             let seeds: Vec<_> = assets.iter().map(|x| x.key.to_bytes()).collect();

//             let mut state: PoolState = pool.read_data_with_borsh()?;
//             if state.version == PoolVersion::Uninitialized {
//                 state.version = PoolVersion::InitializedV1;
//                 // may consider validating mint is really mint of token program
//                 state.pool_mint = *pool_mint.key;
//                 state.weight_total = weight_total;
//                 // may consider sorting keys before hashing
//                 let seeds: Vec<&[u8]> = seeds.iter().map(|x| &x[..]).collect();
//                 state.assets_hash = Pubkey::find_program_address(&seeds[..], program_id).0;
//                 let mut data = pool.try_borrow_mut_data()?;
//                 state.serialize_const(&mut data[..])?;
//                 Ok(())
//             } else {
//                 Err(ProgramError::AccountAlreadyInitialized)
//             }
//         } else {
//             Err(ProgramError::AccountNotRentExempt)
//         }
//     }

//     /// Processes an instruction
//     pub fn process_instruction(
//         program_id: &Pubkey,
//         accounts: &[AccountInfo],
//         input: &[u8],
//     ) -> ProgramResult {
//         let instruction = Instruction::try_from_slice(&input[0..1])?;
//         match instruction {
//             Instruction::InitializeAsset => {
//                 msg!("Instruction: InitializeAsset");
//                 match accounts {
//                     [rent, pool, asset, token, ..] => {
//                         let input = super::instruction::InitializeAssetInput::deserialize_const(
//                             &input[1..],
//                         )?;

//                         Self::initialize_asset(program_id, rent, pool, asset, token, &input)
//                     }
//                     _ => Err(ProgramError::NotEnoughAccountKeys),
//                 }
//             }
//             Instruction::InitializePool => {
//                 msg!("Instruction: InitializeAsset");
//                 match accounts {
//                     [rent, program_token, pool, pool_mint, _, _, ..] => Self::initialize_pool(
//                         program_id,
//                         rent,
//                         program_token,
//                         pool,
//                         pool_mint,
//                         &accounts[4..],
//                     ),
//                     _ => Err(ProgramError::NotEnoughAccountKeys),
//                 }
//             }
//             _ => todo!(),
//         }
//     }
// }
