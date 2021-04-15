#![cfg(feature = "test-bpf")]

use borsh::{BorshDeserialize, BorshSerialize};
use sattelite_friends::*;
use solana_program::{pubkey::Pubkey, system_instruction};
use solana_program_test::*;
use solana_sdk::{
    account::Account,
    signature::{Keypair, Signer},
    transaction::Transaction,
    transport::TransportError,
};

pub fn program_test() -> ProgramTest {
    ProgramTest::new(
        "sattelite_friends",
        id(),
        processor!(processor::Processor::process_instruction),
    )
}

pub async fn get_account(program_context: &mut ProgramTestContext, pubkey: &Pubkey) -> Account {
    program_context
        .banks_client
        .get_account(*pubkey)
        .await
        .expect("account not found")
        .expect("account empty")
}

pub async fn create_account(
    program_context: &mut ProgramTestContext,
    account: &Pubkey,
    base: &Keypair,
    seed: &str,
    rent: u64,
    space: u64,
    owner: &Pubkey,
) -> Result<(), TransportError> {
    let mut transaction = Transaction::new_with_payer(
        &[system_instruction::create_account_with_seed(
            &program_context.payer.pubkey(),
            account,
            &base.pubkey(),
            seed,
            rent,
            space,
            owner,
        )],
        Some(&program_context.payer.pubkey()),
    );

    transaction.sign(
        &[&program_context.payer, base],
        program_context.last_blockhash,
    );
    program_context
        .banks_client
        .process_transaction(transaction)
        .await?;
    Ok(())
}

pub async fn create_friend_info(
    program_context: &mut ProgramTestContext,
    friend_info_acc: &Pubkey,
    user: &Keypair,
) -> Result<(), TransportError> {
    let mut transaction = Transaction::new_with_payer(
        &[instruction::init(&id(), friend_info_acc, &user.pubkey()).unwrap()],
        Some(&program_context.payer.pubkey()),
    );
    transaction.sign(
        &[&program_context.payer, user],
        program_context.last_blockhash,
    );
    program_context
        .banks_client
        .process_transaction(transaction)
        .await?;
    Ok(())
}

#[tokio::test]
async fn test_init_friend_info() {
    let mut program_context = program_test().start_with_context().await;
    let rent = program_context.banks_client.get_rent().await.unwrap();

    let user = Keypair::new();
    let generated_key = Pubkey::create_with_seed(
        &user.pubkey(),
        processor::Processor::FRIEND_INFO_SEED,
        &id(),
    )
    .unwrap();

    let account_min_rent = rent.minimum_balance(state::FriendInfo::LEN);
    create_account(
        &mut program_context,
        &generated_key,
        &user,
        processor::Processor::FRIEND_INFO_SEED,
        account_min_rent,
        state::FriendInfo::LEN as u64,
        &id(),
    )
    .await
    .unwrap();

    create_friend_info(&mut program_context, &generated_key, &user)
        .await
        .unwrap();

    let friend_info_data = get_account(&mut program_context, &generated_key).await;
    let friend_info = state::FriendInfo::try_from_slice(&friend_info_data.data.as_slice()).unwrap();

    assert!(friend_info.is_initialized());

    assert_eq!(friend_info.user, user.pubkey());
}

#[tokio::test]
async fn test_create_address() {
    let mut program_context = program_test().start_with_context().await;
    let rent = program_context.banks_client.get_rent().await.unwrap();

    let user = Keypair::new();
    let (base_program_address, bump_seed) = Pubkey::find_program_address(
        &[&user.pubkey().to_bytes()[..32]],
        &id(),
    );
    let address_to_create = Pubkey::create_with_seed(&base_program_address, processor::Processor::FRIEND_INFO_SEED, &id()).unwrap();

    let mut transaction = Transaction::new_with_payer(
        &[instruction::create_account(
            &id(),
            &program_context.payer.pubkey(),
            &user.pubkey(),
            &base_program_address,
            &address_to_create,
            instruction::AddressType::FriendInfo,
        ).unwrap()],
        Some(&program_context.payer.pubkey()));
    transaction.sign(
        &[&program_context.payer,],
        program_context.last_blockhash,
    );
    program_context
        .banks_client
        .process_transaction(transaction)
        .await.unwrap();
    
    let friend_info_data = get_account(&mut program_context, &address_to_create).await;

    assert_eq!(friend_info_data.data.len(), state::FriendInfo::LEN);
}

// #[tokio::test]
// async fn test_make_friend_request() {
//     let mut program_context = program_test().start_with_context().await;
//     let rent = program_context.banks_client.get_rent().await.unwrap();

//     let user_from = Keypair::new();
//     let user_info_from_key = Pubkey::create_with_seed(&user_from.pubkey(), processor::Processor::FRIEND_INFO_SEED, &id()).unwrap();

//     let user_to = Keypair::new();
//     let user_info_to_key = Pubkey::create_with_seed(&user_to.pubkey(), processor::Processor::FRIEND_INFO_SEED, &id()).unwrap();

//     let friend_info_min_rent = rent.minimum_balance(state::FriendInfo::LEN);

//     // Create account for user who wants to send friend request
//     create_account(&mut program_context, &user_info_from_key, &user_from, processor::Processor::FRIEND_INFO_SEED, friend_info_min_rent, state::FriendInfo::LEN as u64, &id()).await.unwrap();
//     create_friend_info(&mut program_context, &user_info_from_key, &user_from).await.unwrap();
//     let friend_info_from_data = get_account(&mut program_context, &user_info_from_key).await;
//     let friend_info_from = state::FriendInfo::try_from_slice(&friend_info_from_data.data.as_slice()).unwrap();

//     // Create account for user who will receive friend request
//     create_account(&mut program_context, &user_info_to_key, &user_to, processor::Processor::FRIEND_INFO_SEED, friend_info_min_rent, state::FriendInfo::LEN as u64, &id()).await.unwrap();
//     create_friend_info(&mut program_context, &user_info_to_key, &user_to).await.unwrap();
//     let friend_info_to_data = get_account(&mut program_context, &user_info_to_key).await;
//     let friend_info_to = state::FriendInfo::try_from_slice(&friend_info_to_data.data.as_slice()).unwrap();

//     let friend_request_min_rent = rent.minimum_balance(state::Request::LEN);

//     let request_from = Pubkey::create_with_seed(&user_from.pubkey(), format!("{:?}{:?}", friend_info_from.requests_outgoing, processor::Processor::OUTGOING_REQUEST), &id()).unwrap();
//     create_account(&mut program_context, &request_from, &user_from, seed: &str, rent: u64, space: u64, owner: &Pubkey)

//     let request_to = Pubkey::create_with_seed(&user_to.pubkey(), format!("{:?}{:?}", friend_info_to.requests_incoming, processor::Processor::INCOMING_REQUEST), &id()).unwrap();
// }
