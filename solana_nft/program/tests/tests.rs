#![cfg(feature = "test-bpf")]

use borsh::BorshDeserialize;
use solana_program::{instruction::InstructionError, pubkey::Pubkey, system_instruction};
use solana_program_test::*;
use solana_sdk::{
    signature::{Keypair, Signer},
    transaction::{Transaction, TransactionError},
    transport::TransportError,
};
use spl_nft_erc_721::{
    instruction::{self, NftInstruction},
    state::{self, Mint, Token},
};

pub fn program_test() -> ProgramTest {
    ProgramTest::new(
        "spl-nft-erc-721",
        spl_nft_erc_721::id(),
        processor!(spl_nft_erc_721::processor::Processor::process_instruction),
    )
}

#[tokio::test]
async fn initialize_mint_ok() {
    let (mut banks_client, payer, recent_blockhash) = program_test().start().await;
    let mint_account = Keypair::new();
    let data = instruction::MintData::new("KC", "Kitty").unwrap();
    let rent = banks_client.get_rent().await.unwrap();
    let lamports = rent.minimum_balance(Mint::LEN as usize);
    let transaction =
        NftTransactions::create_mint(&payer, &mint_account, lamports, data, recent_blockhash);
    banks_client.process_transaction(transaction).await.unwrap();
}

#[tokio::test]
async fn initialize_mint_not_rent_exempt() {
    let (mut banks_client, payer, recent_blockhash) = program_test().start().await;
    let mint_account = Keypair::new();
    let data = instruction::MintData::new("KC", "Kitty").unwrap();
    let transaction =
        NftTransactions::create_mint(&payer, &mint_account, 0, data, recent_blockhash);
    let result = banks_client
        .process_transaction(transaction)
        .await
        .err()
        .unwrap();

    assert!(matches!(
        result,
        TransportError::TransactionError(TransactionError::InstructionError(
            1,
            InstructionError::InvalidError
        ))
    ));
}

#[tokio::test]
async fn token_flow() {
    // create mint
    let (mut banks_client, payer, recent_blockhash) = program_test().start().await;
    let rent = banks_client.get_rent().await.unwrap();

    let mint_account = Keypair::new();
    let data = instruction::MintData::new("KC", "Kitty").unwrap();
    let lamports = rent.minimum_balance(Mint::LEN as usize);
    let transaction =
        NftTransactions::create_mint(&payer, &mint_account, lamports, data, recent_blockhash);
    banks_client.process_transaction(transaction).await.unwrap();

    // create token of mint
    let asset_owner = Keypair::new();

    let token_account = Keypair::new();
    let url = url::Url::parse("ipfs://bafybeiemxf5abjwjbikoz4mc3a3dla6ual3jsgpdr4cjr3oz3evfyavhwq/wiki/Vincent_van_Gogh.html").unwrap();
    let hash = Pubkey::new_unique();
    let data = instruction::TokenDataArgs::new(hash, url.clone()).unwrap();

    let token_lamports = rent.minimum_balance(Token::LEN as usize);
    let token_data_lamports = rent.minimum_balance(state::TokenData::LEN as usize);

    let (transaction, token_data) = NftTransactions::create_token(
        &payer,
        &token_account,
        asset_owner.pubkey(),
        token_lamports,
        token_data_lamports,
        data,
        mint_account.pubkey(),
        banks_client.get_recent_blockhash().await.unwrap(),
    );
    banks_client.process_transaction(transaction).await.unwrap();
    let token_data_state =
        get_state_with_borsh::<state::TokenData>(&mut banks_client, token_data).await;
    assert_eq!(token_data_state.token, token_account.pubkey());

    // token data was set right
    let token_data_state = banks_client
        .get_account_data_with_borsh::<state::TokenData>(token_data)
        .await
        .unwrap();
    assert_eq!(token_data_state.get_uri(), url);
    assert_eq!(token_data_state.hash, hash);

    let token_state =
        get_state_with_borsh::<state::Token>(&mut banks_client, token_account.pubkey()).await;
    assert_eq!(token_state.owner, asset_owner.pubkey());
    assert_eq!(token_state.mint, mint_account.pubkey());
    assert_eq!(token_state.approval, None);

    let new_asset_owner = Keypair::new();
    // transfer by non owner
    let bad_transfer = NftTransactions::transfer(
        &payer,
        token_account.pubkey(),
        new_asset_owner.pubkey(),
        &new_asset_owner,
        banks_client.get_recent_blockhash().await.unwrap(),
    );
    let bad_transfer_result = banks_client
        .process_transaction(bad_transfer)
        .await
        .err()
        .unwrap();
    assert!(matches!(
        bad_transfer_result,
        TransportError::TransactionError(TransactionError::InstructionError(
            0,
            InstructionError::Custom(0,)
        ))
    ));

    // burn by non owner
    let bad_burn = NftTransactions::burn(
        &payer,
        token_account.pubkey(),
        &new_asset_owner,
        banks_client.get_recent_blockhash().await.unwrap(),
    );
    let bad_burn_result = banks_client
        .process_transaction(bad_burn)
        .await
        .err()
        .unwrap();
    assert!(matches!(
        bad_burn_result,
        TransportError::TransactionError(TransactionError::InstructionError(
            0,
            InstructionError::Custom(0,)
        ))
    ));

    // approve by non owner
    let bad_approve = NftTransactions::approve(
        &payer,
        token_account.pubkey(),
        Pubkey::new_unique(),
        &new_asset_owner,
        banks_client.get_recent_blockhash().await.unwrap(),
    );
    let bad_approve_result = banks_client
        .process_transaction(bad_approve)
        .await
        .err()
        .unwrap();
    assert!(matches!(
        bad_approve_result,
        TransportError::TransactionError(TransactionError::InstructionError(
            0,
            InstructionError::Custom(0,)
        ))
    ));

    // transfer
    let transfer = NftTransactions::transfer(
        &payer,
        token_account.pubkey(),
        new_asset_owner.pubkey(),
        &asset_owner,
        banks_client.get_recent_blockhash().await.unwrap(),
    );
    banks_client.process_transaction(transfer).await.unwrap();
    let token_state =
        get_state_with_borsh::<state::Token>(&mut banks_client, token_account.pubkey()).await;
    assert_eq!(token_state.owner, new_asset_owner.pubkey());

    // burn
    let burn = NftTransactions::burn(
        &payer,
        token_account.pubkey(),
        &new_asset_owner,
        banks_client.get_recent_blockhash().await.unwrap(),
    );
    banks_client.process_transaction(burn).await.unwrap();

    let token_account_info = banks_client
        .get_account(token_account.pubkey())
        .await
        .unwrap();
    assert!(token_account_info.is_none());
}

#[tokio::test]
async fn approval_flow() {
    // setup
    let (mut banks_client, payer, recent_blockhash) = program_test().start().await;
    let rent = banks_client.get_rent().await.unwrap();

    let mint_account = Keypair::new();
    let data = instruction::MintData::new("KC", "Kitty").unwrap();
    let lamports = rent.minimum_balance(Mint::LEN as usize);
    let transaction =
        NftTransactions::create_mint(&payer, &mint_account, lamports, data, recent_blockhash);
    banks_client.process_transaction(transaction).await.unwrap();
    let asset_owner = Keypair::new();
    let token_account = Keypair::new();
    let url = url::Url::parse("ipfs://bafybeiemxf5abjwjbikoz4mc3a3dla6ual3jsgpdr4cjr3oz3evfyavhwq/wiki/Vincent_van_Gogh.html").unwrap();
    let hash = Pubkey::new_unique();
    let data = instruction::TokenDataArgs::new(hash, url.clone()).unwrap();
    let token_lamports = rent.minimum_balance(Token::LEN as usize);
    let token_data_lamports = rent.minimum_balance(state::TokenData::LEN as usize);

    let (transaction, _token_data) = NftTransactions::create_token(
        &payer,
        &token_account,
        asset_owner.pubkey(),
        token_lamports,
        token_data_lamports,
        data,
        mint_account.pubkey(),
        banks_client.get_recent_blockhash().await.unwrap(),
    );
    banks_client.process_transaction(transaction).await.unwrap();

    // approve
    let approval = Keypair::new();
    let approve = NftTransactions::approve(
        &payer,
        token_account.pubkey(),
        approval.pubkey(),
        &asset_owner,
        banks_client.get_recent_blockhash().await.unwrap(),
    );
    banks_client.process_transaction(approve).await.unwrap();
    let token_state =
        get_state_with_borsh::<state::Token>(&mut banks_client, token_account.pubkey()).await;
    assert_eq!(token_state.approval, Some(approval.pubkey()));

    // transfer
    let transfer = NftTransactions::transfer(
        &payer,
        token_account.pubkey(),
        approval.pubkey(),
        &approval,
        banks_client.get_recent_blockhash().await.unwrap(),
    );
    banks_client.process_transaction(transfer).await.unwrap();
    let token_state =
        get_state_with_borsh::<state::Token>(&mut banks_client, token_account.pubkey()).await;
    assert_eq!(token_state.approval, None);
}

async fn get_state_with_borsh<T: BorshDeserialize>(
    banks_client: &mut BanksClient,
    account: Pubkey,
) -> T {
    let state = banks_client.get_account(account).await.unwrap().unwrap();
    T::deserialize(&mut &state.data[..]).unwrap()
}

struct NftTransactions {}

impl NftTransactions {
    fn create_mint(
        payer: &Keypair,
        mint_account: &Keypair,
        lamports: u64,
        data: instruction::MintData,
        recent_blockhash: solana_program::hash::Hash,
    ) -> Transaction {
        let mut transaction = Transaction::new_with_payer(
            &[
                system_instruction::create_account(
                    &payer.pubkey(),
                    &mint_account.pubkey(),
                    lamports,
                    Mint::LEN,
                    &spl_nft_erc_721::id(),
                ),
                NftInstruction::initialize_mint(&mint_account.pubkey(), data, &payer.pubkey()),
            ],
            Some(&payer.pubkey()),
        );
        transaction.sign(&[payer, mint_account], recent_blockhash);
        transaction
    }

    #[allow(clippy::too_many_arguments)]
    fn create_token(
        payer: &Keypair,
        token: &Keypair,
        owner: Pubkey,
        token_lamports: u64,
        token_data_lamports: u64,
        data: instruction::TokenDataArgs,
        mint: Pubkey,
        recent_blockhash: solana_program::hash::Hash,
    ) -> (Transaction, Pubkey) {
        // derive token_data key to be able to find token_data by token key
        let seed = "token_data";
        let token_data =
            Pubkey::create_with_seed(&token.pubkey(), seed, &spl_nft_erc_721::id()).unwrap();

        let mut transaction = Transaction::new_with_payer(
            &[
                system_instruction::create_account(
                    &payer.pubkey(),
                    &token.pubkey(),
                    token_lamports,
                    Token::LEN,
                    &spl_nft_erc_721::id(),
                ),
                system_instruction::create_account_with_seed(
                    &payer.pubkey(),
                    &token_data,
                    &token.pubkey(),
                    seed,
                    token_data_lamports,
                    spl_nft_erc_721::state::TokenData::LEN,
                    &spl_nft_erc_721::id(),
                ),
                NftInstruction::initialize_token(
                    &token.pubkey(),
                    &token_data,
                    &mint,
                    &owner,
                    data,
                    &payer.pubkey(),
                ),
            ],
            Some(&payer.pubkey()),
        );
        transaction.sign(&[payer, token], recent_blockhash);
        (transaction, token_data)
    }

    fn burn(
        payer: &Keypair,
        token: Pubkey,
        approval_or_owner: &Keypair,
        recent_blockhash: solana_program::hash::Hash,
    ) -> Transaction {
        let seed = "token_data";
        let token_data = Pubkey::create_with_seed(&token, seed, &spl_nft_erc_721::id()).unwrap();

        let mut transaction = Transaction::new_with_payer(
            &[NftInstruction::burn(
                token,
                token_data,
                approval_or_owner.pubkey(),
            )],
            Some(&payer.pubkey()),
        );
        transaction.sign(&[payer, approval_or_owner], recent_blockhash);
        transaction
    }

    fn transfer(
        payer: &Keypair,
        token: Pubkey,
        new_owner: Pubkey,
        approval_or_owner: &Keypair,
        recent_blockhash: solana_program::hash::Hash,
    ) -> Transaction {
        let mut transaction = Transaction::new_with_payer(
            &[NftInstruction::transfer(
                token,
                new_owner,
                approval_or_owner.pubkey(),
            )],
            Some(&payer.pubkey()),
        );
        transaction.sign(&[payer, approval_or_owner], recent_blockhash);
        transaction
    }

    fn approve(
        payer: &Keypair,
        token: Pubkey,
        new_approval: Pubkey,
        approval_or_owner: &Keypair,
        recent_blockhash: solana_program::hash::Hash,
    ) -> Transaction {
        let mut transaction = Transaction::new_with_payer(
            &[NftInstruction::approve(
                token,
                new_approval,
                approval_or_owner.pubkey(),
            )],
            Some(&payer.pubkey()),
        );
        transaction.sign(&[payer, approval_or_owner], recent_blockhash);
        transaction
    }
}
