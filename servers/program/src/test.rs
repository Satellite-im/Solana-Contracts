// //! Helper function for testing.
// #![cfg(feature = "test-bpf")]
// #![allow(dead_code)] // for future setups

// use std::marker::PhantomData;

// use solana_program::{
//     program_pack::Pack,
//     pubkey::Pubkey,
//     rent::Rent,
//     system_instruction,
//     sysvar::{self},
// };
// use solana_program_test::*;
// use solana_sdk::{
//     signature::{Keypair, Signer},
//     transaction::Transaction,
// };

// use crate::instruction::InitializeAssetInput;

// /// derived alternative of Keypair not on elliptic curve
// #[derive(Debug)]
// pub struct ProgramAuthority {
//     pub account: Keypair,
//     pub authority: Pubkey,
//     pub bump_seed: u8,
// }

// impl ProgramAuthority {
//     pub fn new(account: Keypair, program_id: Pubkey) -> Self {
//         let (authority, bump_seed) =
//             Pubkey::find_program_address(&[&account.pubkey().to_bytes()[..32]], &program_id);
//         Self {
//             account,
//             authority,
//             bump_seed,
//         }
//     }

//     pub fn pubkey(&self) -> Pubkey {
//         self.account.pubkey()
//     }
// }

// pub struct Mint<T> {
//     pub account: Keypair,
//     pub phantom_data: PhantomData<T>,
// }

// pub struct Token<T> {
//     pub account: Keypair,
//     pub phantom_data: PhantomData<T>,
// }

// pub struct Asset {
//     pub account: Keypair,
// }

// impl Asset {
//     pub fn pubkey(&self) -> Pubkey {
//         self.account.pubkey()
//     }
// }

// pub struct CreateAssetTransaction {
//     transaction: Transaction,
//     result: Asset,
// }

// impl<T> Token<T> {
//     pub fn pubkey(&self) -> Pubkey {
//         self.account.pubkey()
//     }
// }

// impl<T> Mint<T> {
//     pub fn pubkey(&self) -> Pubkey {
//         self.account.pubkey()
//     }
// }

// pub struct BankInfo {
//     pub rent: Rent,
//     pub hash: solana_program::hash::Hash,
// }

// pub struct CreateMintTransaction<T> {
//     transaction: Transaction,
//     mint: Mint<T>,
// }

// impl<T> CreateMintTransaction<T> {
//     pub async fn process_transaction(self, banks_client: &mut BanksClient) -> Mint<T> {
//         banks_client
//             .process_transaction(self.transaction)
//             .await
//             .unwrap();
//         self.mint
//     }
// }

// pub struct CreateTokenTransaction<T> {
//     transaction: Transaction,
//     token: Token<T>,
// }

// impl<T> CreateTokenTransaction<T> {
//     pub async fn process_transaction(self, banks_client: &mut BanksClient) -> Token<T> {
//         banks_client
//             .process_transaction(self.transaction)
//             .await
//             .unwrap();
//         self.token
//     }
// }

// impl CreateAssetTransaction {
//     pub async fn process_transaction(self, banks_client: &mut BanksClient) -> Asset {
//         banks_client
//             .process_transaction(self.transaction)
//             .await
//             .unwrap();
//         self.result
//     }
// }

// pub fn create_mint<M: Send + Sync>(
//     bank: &BankInfo,
//     payer: &Keypair,
//     owner: &Pubkey,
// ) -> CreateMintTransaction<M> {
//     let account = Keypair::new();
//     let rent = bank.rent.minimum_balance(spl_token::state::Mint::LEN);
//     let mut transaction = Transaction::new_with_payer(
//         &[
//             system_instruction::create_account(
//                 &payer.pubkey(),
//                 &account.pubkey(),
//                 rent,
//                 spl_token::state::Mint::LEN as u64,
//                 &spl_token::id(),
//             ),
//             spl_token::instruction::initialize_mint(
//                 &spl_token::id(),
//                 &account.pubkey(),
//                 &owner,
//                 None,
//                 0,
//             )
//             .unwrap(),
//         ],
//         Some(&payer.pubkey()),
//     );
//     transaction.sign(&[payer, &account], bank.hash);
//     CreateMintTransaction {
//         transaction,
//         mint: Mint {
//             account,
//             phantom_data: PhantomData::default(),
//         },
//     }
// }

// pub fn create_token_account<T>(
//     bank: &BankInfo,
//     payer: &Keypair,
//     mint: &Mint<T>,
//     owner: &Pubkey,
//     amount: u64,
// ) -> CreateTokenTransaction<T> {
//     let account = Keypair::new();
//     let rent = bank.rent.minimum_balance(spl_token::state::Account::LEN);
//     let mut instructions = vec![
//         system_instruction::create_account(
//             &payer.pubkey(),
//             &account.pubkey(),
//             rent,
//             spl_token::state::Account::LEN as u64,
//             &spl_token::id(),
//         ),
//         spl_token::instruction::initialize_account(
//             &spl_token::id(),
//             &account.pubkey(),
//             &mint.pubkey(),
//             &owner,
//         )
//         .unwrap(),
//     ];

//     if amount > 0 {
//         instructions.push(
//             spl_token::instruction::mint_to(
//                 &spl_token::id(),
//                 &mint.pubkey(),
//                 &account.pubkey(),
//                 &payer.pubkey(),
//                 &[&payer.pubkey()],
//                 amount,
//             )
//             .unwrap(),
//         )
//     }

//     let mut transaction = Transaction::new_with_payer(&instructions, Some(&payer.pubkey()));
//     transaction.sign(&[payer, &account], bank.hash);
//     CreateTokenTransaction {
//         transaction,
//         token: Token {
//             account,
//             phantom_data: PhantomData::default(),
//         },
//     }
// }

// pub struct CreateMegaSwapTransaction<A> {
//     transaction: Transaction,
//     result: TokenMegaSwap<A>,
// }

// impl<T> CreateMegaSwapTransaction<T> {
//     pub async fn process_transaction(self, banks_client: &mut BanksClient) -> TokenMegaSwap<T> {
//         banks_client
//             .process_transaction(self.transaction)
//             .await
//             .unwrap();
//         self.result
//     }
// }

// pub struct TokenMegaSwap<A> {
//     pub pool: ProgramAuthority,
//     pub mint: Mint<A>,
// }

// impl<A> TokenMegaSwap<A> {
//     pub fn pubkey(&self) -> Pubkey {
//         self.pool.pubkey()
//     }
// }

// pub struct Amount<T> {
//     pub amount: u64,
//     pub phantom_data: PhantomData<T>,
// }

// impl<T> Amount<T> {
//     pub fn new(amount: u64) -> Self {
//         Self {
//             amount,
//             phantom_data: Default::default(),
//         }
//     }
// }

// pub fn create_asset<T>(
//     bank: &BankInfo,
//     payer: &Keypair,
//     pool: &Pubkey,
//     asset: ProgramAuthority,
//     token: &Token<T>,
//     input: InitializeAssetInput,
// ) -> CreateAssetTransaction {
//     let rent = bank.rent.minimum_balance(crate::state::AssetState::len());
//     let instructions = vec![
//         system_instruction::create_account(
//             &payer.pubkey(),
//             &asset.pubkey(),
//             rent,
//             crate::state::AssetState::len() as u64,
//             &crate::id(),
//         ),
//         crate::instruction::initialize_asset(
//             &sysvar::rent::id(),
//             &pool,
//             &asset.pubkey(),
//             &token.pubkey(),
//             input,
//         )
//         .unwrap(),
//     ];

//     let mut transaction = Transaction::new_with_payer(&instructions, Some(&payer.pubkey()));
//     transaction.sign(&[payer, &asset.account], bank.hash);
//     CreateAssetTransaction {
//         transaction,
//         result: Asset {
//             account: asset.account,
//         },
//     }
// }

// pub fn create_mega_swap<T>(
//     bank: &BankInfo,
//     payer: &Keypair,
//     pool: ProgramAuthority,
//     pool_mint: Mint<T>,
//     assets: &[Pubkey],
// ) -> CreateMegaSwapTransaction<T> {
//     let rent = bank.rent.minimum_balance(crate::state::PoolState::len());
//     let instructions = vec![
//         system_instruction::create_account(
//             &payer.pubkey(),
//             &pool.pubkey(),
//             rent,
//             crate::state::PoolState::len() as u64,
//             &crate::id(),
//         ),
//         crate::instruction::initialize_pool(
//             &sysvar::rent::id(),
//             &spl_token::id(),
//             &pool.pubkey(),
//             &pool_mint.pubkey(),
//             &assets,
//         )
//         .unwrap(),
//     ];

//     let mut transaction = Transaction::new_with_payer(&instructions, Some(&payer.pubkey()));
//     transaction.sign(&[payer, &pool.account], bank.hash);
//     CreateMegaSwapTransaction {
//         transaction,
//         result: TokenMegaSwap {
//             mint: pool_mint,
//             pool,
//         },
//     }
// }

// pub async fn get_token_balance(banks_client: &mut BanksClient, token: &Pubkey) -> u64 {
//     let token_account = banks_client.get_account(*token).await.unwrap().unwrap();
//     let account_info: spl_token::state::Account =
//         spl_token::state::Account::unpack_from_slice(token_account.data.as_slice()).unwrap();
//     account_info.amount
// }
