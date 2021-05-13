#![cfg(feature = "test-bpf")]

use borsh::BorshDeserialize;
use satellite_friends::*;
use solana_program::pubkey::Pubkey;
use solana_program_test::*;
use solana_sdk::{
    account::Account,
    signature::{Keypair, Signer},
    transaction::Transaction,
    transport::TransportError,
};

pub fn program_test() -> ProgramTest {
    ProgramTest::new(
        "satellite_friends",
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
    user_address: &Pubkey,
    base_program_address: &Pubkey,
    address_to_create: &Pubkey,
    address_type: instruction::AddressType,
) -> Result<(), TransportError> {
    let mut transaction = Transaction::new_with_payer(
        &[instruction::create_account(
            &id(),
            &program_context.payer.pubkey(),
            user_address,
            base_program_address,
            address_to_create,
            address_type,
        )
        .unwrap()],
        Some(&program_context.payer.pubkey()),
    );
    transaction.sign(&[&program_context.payer], program_context.last_blockhash);
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

pub async fn create_friend_request(
    program_context: &mut ProgramTestContext,
    request_from_to: &Pubkey,
    request_to_from: &Pubkey,
    friend_info_from: &Pubkey,
    friend_info_to: &Pubkey,
    user_from: &Keypair,
) -> Result<(), TransportError> {
    let mut transaction = Transaction::new_with_payer(
        &[instruction::make_request(
            &id(),
            request_from_to,
            request_to_from,
            friend_info_from,
            friend_info_to,
            &user_from.pubkey(),
        )
        .unwrap()],
        Some(&program_context.payer.pubkey()),
    );
    transaction.sign(
        &[&program_context.payer, user_from],
        program_context.last_blockhash,
    );
    program_context
        .banks_client
        .process_transaction(transaction)
        .await?;
    Ok(())
}

pub async fn accept_friend_request(
    program_context: &mut ProgramTestContext,
    request_from_to: &Pubkey,
    request_to_from: &Pubkey,
    last_request_from_to: &Pubkey,
    last_request_to_from: &Pubkey,
    friend_info_from: &Pubkey,
    friend_info_to: &Pubkey,
    friend_to: &Pubkey,
    friend_from: &Pubkey,
    user_to: &Keypair,
    thread_id: [u8; 32],
) -> Result<(), TransportError> {
    let mut transaction = Transaction::new_with_payer(
        &[instruction::accept_request(
            &id(),
            request_from_to,
            request_to_from,
            last_request_from_to,
            last_request_to_from,
            friend_info_from,
            friend_info_to,
            friend_to,
            friend_from,
            &user_to.pubkey(),
            thread_id,
            thread_id,
        )
        .unwrap()],
        Some(&program_context.payer.pubkey()),
    );
    transaction.sign(
        &[&program_context.payer, user_to],
        program_context.last_blockhash,
    );
    program_context
        .banks_client
        .process_transaction(transaction)
        .await?;
    Ok(())
}

pub async fn deny_friend_request(
    program_context: &mut ProgramTestContext,
    request_from_to: &Pubkey,
    request_to_from: &Pubkey,
    last_request_from: &Pubkey,
    last_request_to: &Pubkey,
    friend_info_from: &Pubkey,
    friend_info_to: &Pubkey,
    user_to: &Keypair,
) -> Result<(), TransportError> {
    let mut transaction = Transaction::new_with_payer(
        &[instruction::deny_request(
            &id(),
            request_from_to,
            request_to_from,
            last_request_from,
            last_request_to,
            friend_info_from,
            friend_info_to,
            &user_to.pubkey(),
        )
        .unwrap()],
        Some(&program_context.payer.pubkey()),
    );
    transaction.sign(
        &[&program_context.payer, user_to],
        program_context.last_blockhash,
    );
    program_context
        .banks_client
        .process_transaction(transaction)
        .await?;
    Ok(())
}

pub async fn remove_friend_request(
    program_context: &mut ProgramTestContext,
    request_from_to: &Pubkey,
    request_to_from: &Pubkey,
    last_request_from: &Pubkey,
    last_request_to: &Pubkey,
    friend_info_from: &Pubkey,
    friend_info_to: &Pubkey,
    user_from: &Keypair,
) -> Result<(), TransportError> {
    let mut transaction = Transaction::new_with_payer(
        &[instruction::remove_request(
            &id(),
            request_from_to,
            request_to_from,
            last_request_from,
            last_request_to,
            friend_info_from,
            friend_info_to,
            &user_from.pubkey(),
        )
        .unwrap()],
        Some(&program_context.payer.pubkey()),
    );
    transaction.sign(
        &[&program_context.payer, user_from],
        program_context.last_blockhash,
    );
    program_context
        .banks_client
        .process_transaction(transaction)
        .await?;
    Ok(())
}

pub async fn remove_friend(
    program_context: &mut ProgramTestContext,
    friend_info_first: &Pubkey,
    friend_info_second: &Pubkey,
    friend_first: &Pubkey,
    friend_second: &Pubkey,
    user: &Keypair,
) -> Result<(), TransportError> {
    let mut transaction = Transaction::new_with_payer(
        &[instruction::remove_friend(
            &id(),
            friend_info_first,
            friend_info_second,
            friend_first,
            friend_second,
            &user.pubkey(),
        )
        .unwrap()],
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

    let user = Keypair::new();
    let (base, _) = Pubkey::find_program_address(&[&user.pubkey().to_bytes()[..32]], &id());
    let generated_key =
        Pubkey::create_with_seed(&base, processor::Processor::FRIEND_INFO_SEED, &id()).unwrap();

    create_account(
        &mut program_context,
        &user.pubkey(),
        &base,
        &generated_key,
        instruction::AddressType::FriendInfo,
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

    let user = Keypair::new();
    let (base_program_address, _) =
        Pubkey::find_program_address(&[&user.pubkey().to_bytes()[..32]], &id());
    let address_to_create = Pubkey::create_with_seed(
        &base_program_address,
        processor::Processor::FRIEND_INFO_SEED,
        &id(),
    )
    .unwrap();

    create_account(
        &mut program_context,
        &user.pubkey(),
        &base_program_address,
        &address_to_create,
        instruction::AddressType::FriendInfo,
    )
    .await
    .unwrap();

    let friend_info_data = get_account(&mut program_context, &address_to_create).await;

    assert_eq!(friend_info_data.data.len(), state::FriendInfo::LEN);

    let request_index = 0;

    let outgoing_req_to_create = Pubkey::create_with_seed(
        &base_program_address,
        &format!(
            "{:?}{}",
            request_index,
            processor::Processor::OUTGOING_REQUEST
        ),
        &id(),
    )
    .unwrap();

    create_account(
        &mut program_context,
        &user.pubkey(),
        &base_program_address,
        &outgoing_req_to_create,
        instruction::AddressType::RequestOutgoing(request_index),
    )
    .await
    .unwrap();

    let outgoing_req_info_data = get_account(&mut program_context, &outgoing_req_to_create).await;

    assert_eq!(outgoing_req_info_data.data.len(), state::Request::LEN);

    let user_to = Keypair::new();
    let (base_program_address, _) =
        Pubkey::find_program_address(&[&user_to.pubkey().to_bytes()[..32]], &id());
    let incoming_req_to_create = Pubkey::create_with_seed(
        &base_program_address,
        &format!(
            "{:?}{}",
            request_index,
            processor::Processor::INCOMING_REQUEST
        ),
        &id(),
    )
    .unwrap();

    create_account(
        &mut program_context,
        &user_to.pubkey(),
        &base_program_address,
        &incoming_req_to_create,
        instruction::AddressType::RequestIncoming(request_index),
    )
    .await
    .unwrap();

    let incoming_req_info_data = get_account(&mut program_context, &incoming_req_to_create).await;

    assert_eq!(incoming_req_info_data.data.len(), state::Request::LEN);

    let (base_program_address, _) = Pubkey::find_program_address(
        &[
            &user_to.pubkey().to_bytes()[..32],
            &user.pubkey().to_bytes()[..32],
        ],
        &id(),
    );

    let friend_acc_to_create = Pubkey::create_with_seed(
        &base_program_address,
        processor::Processor::FRIEND_SEED,
        &id(),
    )
    .unwrap();

    create_account(
        &mut program_context,
        &user_to.pubkey(),
        &base_program_address,
        &friend_acc_to_create,
        instruction::AddressType::Friend(user.pubkey()),
    )
    .await
    .unwrap();

    let friend_acc_info_data = get_account(&mut program_context, &friend_acc_to_create).await;

    assert_eq!(friend_acc_info_data.data.len(), state::Friend::LEN);
}

#[tokio::test]
async fn test_make_friend_request() {
    let mut program_context = program_test().start_with_context().await;

    let user_from = Keypair::new();
    let (base_user_from, _) =
        Pubkey::find_program_address(&[&user_from.pubkey().to_bytes()[..32]], &id());
    let user_info_from_key = Pubkey::create_with_seed(
        &base_user_from,
        processor::Processor::FRIEND_INFO_SEED,
        &id(),
    )
    .unwrap();

    let user_to = Keypair::new();
    let (base_user_to, _) =
        Pubkey::find_program_address(&[&user_to.pubkey().to_bytes()[..32]], &id());
    let user_info_to_key =
        Pubkey::create_with_seed(&base_user_to, processor::Processor::FRIEND_INFO_SEED, &id())
            .unwrap();

    // Create account for user who wants to send friend request
    create_account(
        &mut program_context,
        &user_from.pubkey(),
        &base_user_from,
        &user_info_from_key,
        instruction::AddressType::FriendInfo,
    )
    .await
    .unwrap();
    create_friend_info(&mut program_context, &user_info_from_key, &user_from)
        .await
        .unwrap();
    let friend_info_from_data = get_account(&mut program_context, &user_info_from_key).await;
    let friend_info_from =
        state::FriendInfo::try_from_slice(&friend_info_from_data.data.as_slice()).unwrap();

    // Create account for user who will receive friend request
    create_account(
        &mut program_context,
        &user_to.pubkey(),
        &base_user_to,
        &user_info_to_key,
        instruction::AddressType::FriendInfo,
    )
    .await
    .unwrap();
    create_friend_info(&mut program_context, &user_info_to_key, &user_to)
        .await
        .unwrap();
    let friend_info_to_data = get_account(&mut program_context, &user_info_to_key).await;
    let friend_info_to =
        state::FriendInfo::try_from_slice(&friend_info_to_data.data.as_slice()).unwrap();

    // Create request from account
    let (request_from_base, _) =
        Pubkey::find_program_address(&[&user_from.pubkey().to_bytes()[..32]], &id());
    let request_from = Pubkey::create_with_seed(
        &request_from_base,
        &format!(
            "{:?}{}",
            friend_info_from.requests_outgoing,
            processor::Processor::OUTGOING_REQUEST
        ),
        &id(),
    )
    .unwrap();
    create_account(
        &mut program_context,
        &user_from.pubkey(),
        &request_from_base,
        &request_from,
        instruction::AddressType::RequestOutgoing(friend_info_from.requests_outgoing),
    )
    .await
    .unwrap();

    let (request_to_base, _) =
        Pubkey::find_program_address(&[&user_to.pubkey().to_bytes()[..32]], &id());
    let request_to = Pubkey::create_with_seed(
        &request_to_base,
        &format!(
            "{:?}{}",
            friend_info_to.requests_incoming,
            processor::Processor::INCOMING_REQUEST
        ),
        &id(),
    )
    .unwrap();
    create_account(
        &mut program_context,
        &user_to.pubkey(),
        &request_to_base,
        &request_to,
        instruction::AddressType::RequestIncoming(friend_info_to.requests_incoming),
    )
    .await
    .unwrap();

    let friend_info_from_data = get_account(&mut program_context, &user_info_from_key).await;
    let friend_info_from =
        state::FriendInfo::try_from_slice(&friend_info_from_data.data.as_slice()).unwrap();
    let outgoing_requests_before = friend_info_from.requests_outgoing;

    let friend_info_to_data = get_account(&mut program_context, &user_info_to_key).await;
    let friend_info_to =
        state::FriendInfo::try_from_slice(&friend_info_to_data.data.as_slice()).unwrap();
    let incoming_requests_before = friend_info_to.requests_incoming;

    create_friend_request(
        &mut program_context,
        &request_from,
        &request_to,
        &user_info_from_key,
        &user_info_to_key,
        &user_from,
    )
    .await
    .unwrap();

    let request_from_info_data = get_account(&mut program_context, &request_from).await;
    let request_from_info =
        state::Request::try_from_slice(&request_from_info_data.data.as_slice()).unwrap();

    assert!(request_from_info.is_initialized());
    assert_eq!(request_from_info.from, user_from.pubkey());
    assert_eq!(request_from_info.to, user_to.pubkey());

    let request_to_info_data = get_account(&mut program_context, &request_to).await;
    let request_to_info =
        state::Request::try_from_slice(&request_to_info_data.data.as_slice()).unwrap();

    assert!(request_to_info.is_initialized());
    assert_eq!(request_to_info.from, user_from.pubkey());
    assert_eq!(request_to_info.to, user_to.pubkey());

    let friend_info_from_data = get_account(&mut program_context, &user_info_from_key).await;
    let friend_info_from =
        state::FriendInfo::try_from_slice(&friend_info_from_data.data.as_slice()).unwrap();

    let friend_info_to_data = get_account(&mut program_context, &user_info_to_key).await;
    let friend_info_to =
        state::FriendInfo::try_from_slice(&friend_info_to_data.data.as_slice()).unwrap();

    assert_eq!(
        friend_info_from.requests_outgoing,
        outgoing_requests_before + 1
    );
    assert_eq!(
        friend_info_to.requests_incoming,
        incoming_requests_before + 1
    );
}

#[tokio::test]
async fn test_accept_friend_request() {
    let mut program_context = program_test().start_with_context().await;

    let user_from = Keypair::new();
    let (base_user_from, _) =
        Pubkey::find_program_address(&[&user_from.pubkey().to_bytes()[..32]], &id());
    let user_info_from_key = Pubkey::create_with_seed(
        &base_user_from,
        processor::Processor::FRIEND_INFO_SEED,
        &id(),
    )
    .unwrap();

    let user_to = Keypair::new();
    let (base_user_to, _) =
        Pubkey::find_program_address(&[&user_to.pubkey().to_bytes()[..32]], &id());
    let user_info_to_key =
        Pubkey::create_with_seed(&base_user_to, processor::Processor::FRIEND_INFO_SEED, &id())
            .unwrap();

    // Create account for user who wants to send friend request
    create_account(
        &mut program_context,
        &user_from.pubkey(),
        &base_user_from,
        &user_info_from_key,
        instruction::AddressType::FriendInfo,
    )
    .await
    .unwrap();
    create_friend_info(&mut program_context, &user_info_from_key, &user_from)
        .await
        .unwrap();
    let friend_info_from_data = get_account(&mut program_context, &user_info_from_key).await;
    let friend_info_from =
        state::FriendInfo::try_from_slice(&friend_info_from_data.data.as_slice()).unwrap();

    // Create account for user who will receive friend request
    create_account(
        &mut program_context,
        &user_to.pubkey(),
        &base_user_to,
        &user_info_to_key,
        instruction::AddressType::FriendInfo,
    )
    .await
    .unwrap();
    create_friend_info(&mut program_context, &user_info_to_key, &user_to)
        .await
        .unwrap();
    let friend_info_to_data = get_account(&mut program_context, &user_info_to_key).await;
    let friend_info_to =
        state::FriendInfo::try_from_slice(&friend_info_to_data.data.as_slice()).unwrap();

    // Create request from account
    let (request_from_base, _) =
        Pubkey::find_program_address(&[&user_from.pubkey().to_bytes()[..32]], &id());
    let request_from = Pubkey::create_with_seed(
        &request_from_base,
        &format!(
            "{:?}{}",
            friend_info_from.requests_outgoing,
            processor::Processor::OUTGOING_REQUEST
        ),
        &id(),
    )
    .unwrap();
    create_account(
        &mut program_context,
        &user_from.pubkey(),
        &request_from_base,
        &request_from,
        instruction::AddressType::RequestOutgoing(friend_info_from.requests_outgoing),
    )
    .await
    .unwrap();

    let (request_to_base, _) =
        Pubkey::find_program_address(&[&user_to.pubkey().to_bytes()[..32]], &id());
    let request_to = Pubkey::create_with_seed(
        &request_to_base,
        &format!(
            "{:?}{}",
            friend_info_to.requests_incoming,
            processor::Processor::INCOMING_REQUEST
        ),
        &id(),
    )
    .unwrap();
    create_account(
        &mut program_context,
        &user_to.pubkey(),
        &request_to_base,
        &request_to,
        instruction::AddressType::RequestIncoming(friend_info_to.requests_incoming),
    )
    .await
    .unwrap();

    create_friend_request(
        &mut program_context,
        &request_from,
        &request_to,
        &user_info_from_key,
        &user_info_to_key,
        &user_from,
    )
    .await
    .unwrap();

    let (base_friend_to_account, _) = Pubkey::find_program_address(
        &[
            &user_to.pubkey().to_bytes()[..32],
            &user_from.pubkey().to_bytes()[..32],
        ],
        &id(),
    );
    let friend_to_key = Pubkey::create_with_seed(
        &base_friend_to_account,
        processor::Processor::FRIEND_SEED,
        &id(),
    )
    .unwrap();
    create_account(
        &mut program_context,
        &user_to.pubkey(),
        &base_friend_to_account,
        &friend_to_key,
        instruction::AddressType::Friend(user_from.pubkey()),
    )
    .await
    .unwrap();

    let (base_friend_from_account, _) = Pubkey::find_program_address(
        &[
            &user_from.pubkey().to_bytes()[..32],
            &user_to.pubkey().to_bytes()[..32],
        ],
        &id(),
    );
    let friend_from_key = Pubkey::create_with_seed(
        &base_friend_from_account,
        processor::Processor::FRIEND_SEED,
        &id(),
    )
    .unwrap();
    create_account(
        &mut program_context,
        &user_from.pubkey(),
        &base_friend_from_account,
        &friend_from_key,
        instruction::AddressType::Friend(user_to.pubkey()),
    )
    .await
    .unwrap();

    let conv_thread = [1; 32];

    let friend_info_from_data = get_account(&mut program_context, &user_info_from_key).await;
    let friend_info_from =
        state::FriendInfo::try_from_slice(&friend_info_from_data.data.as_slice()).unwrap();
    let outgoing_requests_before = friend_info_from.requests_outgoing;
    let friends_acc_from_before = friend_info_from.friends;

    let friend_info_to_data = get_account(&mut program_context, &user_info_to_key).await;
    let friend_info_to =
        state::FriendInfo::try_from_slice(&friend_info_to_data.data.as_slice()).unwrap();
    let incoming_requests_before = friend_info_to.requests_incoming;
    let friends_acc_to_before = friend_info_to.friends;

    accept_friend_request(
        &mut program_context,
        &request_from,
        &request_to,
        &request_from,
        &request_to,
        &user_info_from_key,
        &user_info_to_key,
        &friend_to_key,
        &friend_from_key,
        &user_to,
        conv_thread,
    )
    .await
    .unwrap();

    let friend_from_info_data = get_account(&mut program_context, &friend_from_key).await;
    let friend_from_info =
        state::Friend::try_from_slice(&friend_from_info_data.data.as_slice()).unwrap();

    assert!(friend_from_info.is_initialized());
    assert_eq!(friend_from_info.user, user_from.pubkey());
    assert_eq!(friend_from_info.friend, user_to.pubkey());
    assert_eq!(friend_from_info.thread_id1, conv_thread);
    assert_eq!(friend_from_info.thread_id2, conv_thread);

    let friend_to_info_data = get_account(&mut program_context, &friend_to_key).await;
    let friend_to_info =
        state::Friend::try_from_slice(&friend_to_info_data.data.as_slice()).unwrap();

    assert!(friend_to_info.is_initialized());
    assert_eq!(friend_to_info.user, user_to.pubkey());
    assert_eq!(friend_to_info.friend, user_from.pubkey());
    assert_eq!(friend_to_info.thread_id1, conv_thread);
    assert_eq!(friend_to_info.thread_id2, conv_thread);

    let friend_info_from_data = get_account(&mut program_context, &user_info_from_key).await;
    let friend_info_from =
        state::FriendInfo::try_from_slice(&friend_info_from_data.data.as_slice()).unwrap();

    let friend_info_to_data = get_account(&mut program_context, &user_info_to_key).await;
    let friend_info_to =
        state::FriendInfo::try_from_slice(&friend_info_to_data.data.as_slice()).unwrap();

    assert_eq!(
        friend_info_from.requests_outgoing,
        outgoing_requests_before - 1
    );
    assert_eq!(
        friend_info_to.requests_incoming,
        incoming_requests_before - 1
    );
    assert_eq!(friend_info_from.friends, friends_acc_from_before + 1);
    assert_eq!(friend_info_to.friends, friends_acc_to_before + 1);
}

#[tokio::test]
async fn test_deny_friend_request() {
    let mut program_context = program_test().start_with_context().await;

    let user_from = Keypair::new();
    let (base_user_from, _) =
        Pubkey::find_program_address(&[&user_from.pubkey().to_bytes()[..32]], &id());
    let user_info_from_key = Pubkey::create_with_seed(
        &base_user_from,
        processor::Processor::FRIEND_INFO_SEED,
        &id(),
    )
    .unwrap();

    let user_to = Keypair::new();
    let (base_user_to, _) =
        Pubkey::find_program_address(&[&user_to.pubkey().to_bytes()[..32]], &id());
    let user_info_to_key =
        Pubkey::create_with_seed(&base_user_to, processor::Processor::FRIEND_INFO_SEED, &id())
            .unwrap();

    // Create account for user who wants to send friend request
    create_account(
        &mut program_context,
        &user_from.pubkey(),
        &base_user_from,
        &user_info_from_key,
        instruction::AddressType::FriendInfo,
    )
    .await
    .unwrap();
    create_friend_info(&mut program_context, &user_info_from_key, &user_from)
        .await
        .unwrap();
    let friend_info_from_data = get_account(&mut program_context, &user_info_from_key).await;
    let friend_info_from =
        state::FriendInfo::try_from_slice(&friend_info_from_data.data.as_slice()).unwrap();

    // Create account for user who will receive friend request
    create_account(
        &mut program_context,
        &user_to.pubkey(),
        &base_user_to,
        &user_info_to_key,
        instruction::AddressType::FriendInfo,
    )
    .await
    .unwrap();
    create_friend_info(&mut program_context, &user_info_to_key, &user_to)
        .await
        .unwrap();
    let friend_info_to_data = get_account(&mut program_context, &user_info_to_key).await;
    let friend_info_to =
        state::FriendInfo::try_from_slice(&friend_info_to_data.data.as_slice()).unwrap();

    // Create request from account
    let (request_from_base, _) =
        Pubkey::find_program_address(&[&user_from.pubkey().to_bytes()[..32]], &id());
    let request_from = Pubkey::create_with_seed(
        &request_from_base,
        &format!(
            "{:?}{}",
            friend_info_from.requests_outgoing,
            processor::Processor::OUTGOING_REQUEST
        ),
        &id(),
    )
    .unwrap();
    create_account(
        &mut program_context,
        &user_from.pubkey(),
        &request_from_base,
        &request_from,
        instruction::AddressType::RequestOutgoing(friend_info_from.requests_outgoing),
    )
    .await
    .unwrap();

    let (request_to_base, _) =
        Pubkey::find_program_address(&[&user_to.pubkey().to_bytes()[..32]], &id());
    let request_to = Pubkey::create_with_seed(
        &request_to_base,
        &format!(
            "{:?}{}",
            friend_info_to.requests_incoming,
            processor::Processor::INCOMING_REQUEST
        ),
        &id(),
    )
    .unwrap();
    create_account(
        &mut program_context,
        &user_to.pubkey(),
        &request_to_base,
        &request_to,
        instruction::AddressType::RequestIncoming(friend_info_to.requests_incoming),
    )
    .await
    .unwrap();

    create_friend_request(
        &mut program_context,
        &request_from,
        &request_to,
        &user_info_from_key,
        &user_info_to_key,
        &user_from,
    )
    .await
    .unwrap();

    let friend_info_from_data = get_account(&mut program_context, &user_info_from_key).await;
    let friend_info_from =
        state::FriendInfo::try_from_slice(&friend_info_from_data.data.as_slice()).unwrap();
    let outgoing_requests_before = friend_info_from.requests_outgoing;
    let friends_acc_from_before = friend_info_from.friends;

    let friend_info_to_data = get_account(&mut program_context, &user_info_to_key).await;
    let friend_info_to =
        state::FriendInfo::try_from_slice(&friend_info_to_data.data.as_slice()).unwrap();
    let incoming_requests_before = friend_info_to.requests_incoming;
    let friends_acc_to_before = friend_info_to.friends;

    deny_friend_request(
        &mut program_context,
        &request_from,
        &request_to,
        &request_from,
        &request_to,
        &user_info_from_key,
        &user_info_to_key,
        &user_to,
    )
    .await
    .unwrap();

    let request_from_info_data = get_account(&mut program_context, &request_from).await;
    let request_from_info =
        state::Request::try_from_slice(&request_from_info_data.data.as_slice()).unwrap();

    assert_eq!(request_from_info.is_initialized(), false);

    let request_to_info_data = get_account(&mut program_context, &request_to).await;
    let request_to_info =
        state::Request::try_from_slice(&request_to_info_data.data.as_slice()).unwrap();

    assert_eq!(request_to_info.is_initialized(), false);

    let friend_info_from_data = get_account(&mut program_context, &user_info_from_key).await;
    let friend_info_from =
        state::FriendInfo::try_from_slice(&friend_info_from_data.data.as_slice()).unwrap();

    let friend_info_to_data = get_account(&mut program_context, &user_info_to_key).await;
    let friend_info_to =
        state::FriendInfo::try_from_slice(&friend_info_to_data.data.as_slice()).unwrap();

    assert_eq!(
        friend_info_from.requests_outgoing,
        outgoing_requests_before - 1
    );
    assert_eq!(
        friend_info_to.requests_incoming,
        incoming_requests_before - 1
    );
    assert_eq!(friend_info_from.friends, friends_acc_from_before);
    assert_eq!(friend_info_to.friends, friends_acc_to_before);
}

#[tokio::test]
async fn test_remove_friend_request() {
    let mut program_context = program_test().start_with_context().await;

    let user_from = Keypair::new();
    let (base_user_from, _) =
        Pubkey::find_program_address(&[&user_from.pubkey().to_bytes()[..32]], &id());
    let user_info_from_key = Pubkey::create_with_seed(
        &base_user_from,
        processor::Processor::FRIEND_INFO_SEED,
        &id(),
    )
    .unwrap();

    let user_to = Keypair::new();
    let (base_user_to, _) =
        Pubkey::find_program_address(&[&user_to.pubkey().to_bytes()[..32]], &id());
    let user_info_to_key =
        Pubkey::create_with_seed(&base_user_to, processor::Processor::FRIEND_INFO_SEED, &id())
            .unwrap();

    // Create account for user who wants to send friend request
    create_account(
        &mut program_context,
        &user_from.pubkey(),
        &base_user_from,
        &user_info_from_key,
        instruction::AddressType::FriendInfo,
    )
    .await
    .unwrap();
    create_friend_info(&mut program_context, &user_info_from_key, &user_from)
        .await
        .unwrap();
    let friend_info_from_data = get_account(&mut program_context, &user_info_from_key).await;
    let friend_info_from =
        state::FriendInfo::try_from_slice(&friend_info_from_data.data.as_slice()).unwrap();

    // Create account for user who will receive friend request
    create_account(
        &mut program_context,
        &user_to.pubkey(),
        &base_user_to,
        &user_info_to_key,
        instruction::AddressType::FriendInfo,
    )
    .await
    .unwrap();
    create_friend_info(&mut program_context, &user_info_to_key, &user_to)
        .await
        .unwrap();
    let friend_info_to_data = get_account(&mut program_context, &user_info_to_key).await;
    let friend_info_to =
        state::FriendInfo::try_from_slice(&friend_info_to_data.data.as_slice()).unwrap();

    // Create request from account
    let (request_from_base, _) =
        Pubkey::find_program_address(&[&user_from.pubkey().to_bytes()[..32]], &id());
    let request_from = Pubkey::create_with_seed(
        &request_from_base,
        &format!(
            "{:?}{}",
            friend_info_from.requests_outgoing,
            processor::Processor::OUTGOING_REQUEST
        ),
        &id(),
    )
    .unwrap();
    create_account(
        &mut program_context,
        &user_from.pubkey(),
        &request_from_base,
        &request_from,
        instruction::AddressType::RequestOutgoing(friend_info_from.requests_outgoing),
    )
    .await
    .unwrap();

    let (request_to_base, _) =
        Pubkey::find_program_address(&[&user_to.pubkey().to_bytes()[..32]], &id());
    let request_to = Pubkey::create_with_seed(
        &request_to_base,
        &format!(
            "{:?}{}",
            friend_info_to.requests_incoming,
            processor::Processor::INCOMING_REQUEST
        ),
        &id(),
    )
    .unwrap();
    create_account(
        &mut program_context,
        &user_to.pubkey(),
        &request_to_base,
        &request_to,
        instruction::AddressType::RequestIncoming(friend_info_to.requests_incoming),
    )
    .await
    .unwrap();

    create_friend_request(
        &mut program_context,
        &request_from,
        &request_to,
        &user_info_from_key,
        &user_info_to_key,
        &user_from,
    )
    .await
    .unwrap();

    let friend_info_from_data = get_account(&mut program_context, &user_info_from_key).await;
    let friend_info_from =
        state::FriendInfo::try_from_slice(&friend_info_from_data.data.as_slice()).unwrap();
    let outgoing_requests_before = friend_info_from.requests_outgoing;

    let friend_info_to_data = get_account(&mut program_context, &user_info_to_key).await;
    let friend_info_to =
        state::FriendInfo::try_from_slice(&friend_info_to_data.data.as_slice()).unwrap();
    let incoming_requests_before = friend_info_to.requests_incoming;

    remove_friend_request(
        &mut program_context,
        &request_from,
        &request_to,
        &request_from,
        &request_to,
        &user_info_from_key,
        &user_info_to_key,
        &user_from,
    )
    .await
    .unwrap();

    let request_from_info_data = get_account(&mut program_context, &request_from).await;
    let request_from_info =
        state::Request::try_from_slice(&request_from_info_data.data.as_slice()).unwrap();

    assert_eq!(request_from_info.is_initialized(), false);

    let request_to_info_data = get_account(&mut program_context, &request_to).await;
    let request_to_info =
        state::Request::try_from_slice(&request_to_info_data.data.as_slice()).unwrap();

    assert_eq!(request_to_info.is_initialized(), false);

    let friend_info_from_data = get_account(&mut program_context, &user_info_from_key).await;
    let friend_info_from =
        state::FriendInfo::try_from_slice(&friend_info_from_data.data.as_slice()).unwrap();

    let friend_info_to_data = get_account(&mut program_context, &user_info_to_key).await;
    let friend_info_to =
        state::FriendInfo::try_from_slice(&friend_info_to_data.data.as_slice()).unwrap();

    assert_eq!(
        friend_info_from.requests_outgoing,
        outgoing_requests_before - 1
    );
    assert_eq!(
        friend_info_to.requests_incoming,
        incoming_requests_before - 1
    );
}

#[tokio::test]
async fn test_remove_friend() {
    let mut program_context = program_test().start_with_context().await;

    let user_from = Keypair::new();
    let (base_user_from, _) =
        Pubkey::find_program_address(&[&user_from.pubkey().to_bytes()[..32]], &id());
    let user_info_from_key = Pubkey::create_with_seed(
        &base_user_from,
        processor::Processor::FRIEND_INFO_SEED,
        &id(),
    )
    .unwrap();

    let user_to = Keypair::new();
    let (base_user_to, _) =
        Pubkey::find_program_address(&[&user_to.pubkey().to_bytes()[..32]], &id());
    let user_info_to_key =
        Pubkey::create_with_seed(&base_user_to, processor::Processor::FRIEND_INFO_SEED, &id())
            .unwrap();

    // Create account for user who wants to send friend request
    create_account(
        &mut program_context,
        &user_from.pubkey(),
        &base_user_from,
        &user_info_from_key,
        instruction::AddressType::FriendInfo,
    )
    .await
    .unwrap();
    create_friend_info(&mut program_context, &user_info_from_key, &user_from)
        .await
        .unwrap();
    let friend_info_from_data = get_account(&mut program_context, &user_info_from_key).await;
    let friend_info_from =
        state::FriendInfo::try_from_slice(&friend_info_from_data.data.as_slice()).unwrap();

    // Create account for user who will receive friend request
    create_account(
        &mut program_context,
        &user_to.pubkey(),
        &base_user_to,
        &user_info_to_key,
        instruction::AddressType::FriendInfo,
    )
    .await
    .unwrap();
    create_friend_info(&mut program_context, &user_info_to_key, &user_to)
        .await
        .unwrap();
    let friend_info_to_data = get_account(&mut program_context, &user_info_to_key).await;
    let friend_info_to =
        state::FriendInfo::try_from_slice(&friend_info_to_data.data.as_slice()).unwrap();

    // Create request from account
    let (request_from_base, _) =
        Pubkey::find_program_address(&[&user_from.pubkey().to_bytes()[..32]], &id());
    let request_from = Pubkey::create_with_seed(
        &request_from_base,
        &format!(
            "{:?}{}",
            friend_info_from.requests_outgoing,
            processor::Processor::OUTGOING_REQUEST
        ),
        &id(),
    )
    .unwrap();
    create_account(
        &mut program_context,
        &user_from.pubkey(),
        &request_from_base,
        &request_from,
        instruction::AddressType::RequestOutgoing(friend_info_from.requests_outgoing),
    )
    .await
    .unwrap();

    let (request_to_base, _) =
        Pubkey::find_program_address(&[&user_to.pubkey().to_bytes()[..32]], &id());
    let request_to = Pubkey::create_with_seed(
        &request_to_base,
        &format!(
            "{:?}{}",
            friend_info_to.requests_incoming,
            processor::Processor::INCOMING_REQUEST
        ),
        &id(),
    )
    .unwrap();
    create_account(
        &mut program_context,
        &user_to.pubkey(),
        &request_to_base,
        &request_to,
        instruction::AddressType::RequestIncoming(friend_info_to.requests_incoming),
    )
    .await
    .unwrap();

    create_friend_request(
        &mut program_context,
        &request_from,
        &request_to,
        &user_info_from_key,
        &user_info_to_key,
        &user_from,
    )
    .await
    .unwrap();

    let (base_friend_to_account, _) = Pubkey::find_program_address(
        &[
            &user_to.pubkey().to_bytes()[..32],
            &user_from.pubkey().to_bytes()[..32],
        ],
        &id(),
    );
    let friend_to_key = Pubkey::create_with_seed(
        &base_friend_to_account,
        processor::Processor::FRIEND_SEED,
        &id(),
    )
    .unwrap();
    create_account(
        &mut program_context,
        &user_to.pubkey(),
        &base_friend_to_account,
        &friend_to_key,
        instruction::AddressType::Friend(user_from.pubkey()),
    )
    .await
    .unwrap();

    let (base_friend_from_account, _) = Pubkey::find_program_address(
        &[
            &user_from.pubkey().to_bytes()[..32],
            &user_to.pubkey().to_bytes()[..32],
        ],
        &id(),
    );
    let friend_from_key = Pubkey::create_with_seed(
        &base_friend_from_account,
        processor::Processor::FRIEND_SEED,
        &id(),
    )
    .unwrap();
    create_account(
        &mut program_context,
        &user_from.pubkey(),
        &base_friend_from_account,
        &friend_from_key,
        instruction::AddressType::Friend(user_to.pubkey()),
    )
    .await
    .unwrap();

    let conv_thread = [1; 32];

    accept_friend_request(
        &mut program_context,
        &request_from,
        &request_to,
        &request_from,
        &request_to,
        &user_info_from_key,
        &user_info_to_key,
        &friend_to_key,
        &friend_from_key,
        &user_to,
        conv_thread,
    )
    .await
    .unwrap();

    let friend_info_from_data = get_account(&mut program_context, &user_info_from_key).await;
    let friend_info_from =
        state::FriendInfo::try_from_slice(&friend_info_from_data.data.as_slice()).unwrap();
    let friends_acc_from_before = friend_info_from.friends;

    let friend_info_to_data = get_account(&mut program_context, &user_info_to_key).await;
    let friend_info_to =
        state::FriendInfo::try_from_slice(&friend_info_to_data.data.as_slice()).unwrap();
    let friends_acc_to_before = friend_info_to.friends;

    remove_friend(
        &mut program_context,
        &user_info_from_key,
        &user_info_to_key,
        &friend_from_key,
        &friend_to_key,
        &user_from,
    )
    .await
    .unwrap();

    let friend_from_info_data = get_account(&mut program_context, &friend_from_key).await;
    let friend_from_info =
        state::Friend::try_from_slice(&friend_from_info_data.data.as_slice()).unwrap();

    assert_eq!(friend_from_info.is_initialized(), false);

    let friend_to_info_data = get_account(&mut program_context, &friend_to_key).await;
    let friend_to_info =
        state::Friend::try_from_slice(&friend_to_info_data.data.as_slice()).unwrap();

    assert_eq!(friend_to_info.is_initialized(), false);

    let friend_info_from_data = get_account(&mut program_context, &user_info_from_key).await;
    let friend_info_from =
        state::FriendInfo::try_from_slice(&friend_info_from_data.data.as_slice()).unwrap();

    let friend_info_to_data = get_account(&mut program_context, &user_info_to_key).await;
    let friend_info_to =
        state::FriendInfo::try_from_slice(&friend_info_to_data.data.as_slice()).unwrap();

    assert_eq!(friend_info_from.friends, friends_acc_from_before - 1);
    assert_eq!(friend_info_to.friends, friends_acc_to_before - 1);
}
