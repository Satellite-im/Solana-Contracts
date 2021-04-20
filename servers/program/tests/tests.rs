//#![cfg(feature = "test-bpf")]

use borsh::BorshDeserialize;
use solana_program::{pubkey::Pubkey, system_instruction};
use solana_program_test::*;
use solana_sdk::{
    account::Account,
    signature::{Keypair, Signer},
    transaction::Transaction,
    transport::TransportError,
};

use satellite_servers::{
    id,
    instruction::{
        self, AddChannelInput, CreateGroupInput, InitializeDwellerInput, InitializeServerInput,
    },
    processor,
    sdk::{
        add_channel_to_group_transaction, add_channel_transaction, add_invite_transaction,
        create_group_transaction, delete_channel_transaction, delete_group_transaction,
        join_server_transaction, leave_server_transaction, remove_admin_transaction,
        remove_channel_from_group_transaction, revoke_invite_server_transaction,
    },
    state::*,
};

pub fn program_test() -> ProgramTest {
    ProgramTest::new(
        "satellite_servers",
        id(),
        processor!(processor::Processor::process_instruction),
    )
}

pub async fn test_create_derived_account(
    program_context: &mut ProgramTestContext,
    owner_address: &Pubkey,
    base_program_address: &Pubkey,
    address_to_create: &Pubkey,
    address_type: instruction::AddressTypeInput,
) -> Result<(), TransportError> {
    let mut transaction = Transaction::new_with_payer(
        &[instruction::create_derived_account(
            &id(),
            &program_context.payer.pubkey(),
            owner_address,
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

#[tokio::test]
async fn dweller_flow() {
    let mut program_context = program_test().start_with_context().await;
    let rent = program_context.banks_client.get_rent().await.unwrap();

    let dweller = Keypair::new();

    test_initialize_dweller(
        &program_context.payer,
        &dweller,
        rent,
        program_context.last_blockhash,
        &mut program_context.banks_client,
    )
    .await;

    let index = 0;
    let (address_to_create, base_program_address, ..) =
        satellite_servers::program::create_base_index_with_seed(
            &id(),
            DwellerServer::SEED,
            &dweller.pubkey(),
            index,
        )
        .unwrap();

    test_create_derived_account(
        &mut program_context,
        &dweller.pubkey(),
        &base_program_address,
        &address_to_create,
        instruction::AddressTypeInput::DwellerServer(index),
    )
    .await
    .unwrap();

    let dweller_server_info_data = get_account(&mut program_context, &address_to_create).await;

    assert_eq!(
        dweller_server_info_data.data.len(),
        DwellerServer::LEN as usize
    );
}

#[tokio::test]
async fn positive_add_remove_flow() {
    let mut blockchain = program_test().start_with_context().await;
    let rent = blockchain.banks_client.get_rent().await.unwrap();

    // create_dwellers
    let dwellers = [
        Keypair::new(),
        Keypair::new(),
        Keypair::new(),
        Keypair::new(),
        Keypair::new(),
        Keypair::new(),
        Keypair::new(),
    ];
    let mut dweller_servers = Vec::new();

    for dweller in dwellers.iter() {
        let index = 0;
        let address_type = instruction::AddressTypeInput::DwellerServer(index);
        let seed = DwellerServer::SEED;

        test_initialize_dweller(
            &blockchain.payer,
            &dweller,
            rent,
            blockchain.last_blockhash,
            &mut blockchain.banks_client,
        )
        .await;

        let address_to_create = create_derived_account_index(
            &mut blockchain,
            &dweller.pubkey(),
            seed,
            index,
            address_type,
        )
        .await;

        let dweller_server: DwellerServer =
            get_account_data(&mut blockchain, &address_to_create).await;

        assert_eq!(dweller_server.container, Pubkey::default(),);

        dweller_servers.push(address_to_create);
    }

    let [dweller_owner, dweller_admin_1, _dweller_admin_2, _dweller_admin_3, dweller_1, _dweller_2, _dweller_3] =
        dwellers;

    // create server
    let server = Keypair::new();

    let mut server_members = Vec::new();
    for index in 0u64..3 {
        let address_type = instruction::AddressTypeInput::ServerMember(index);
        let seed = ServerMember::SEED;

        let address_to_create = create_derived_account_index(
            &mut blockchain,
            &server.pubkey(),
            seed,
            index,
            address_type,
        )
        .await;
        server_members.push(address_to_create);
        let account_state: ServerMember =
            get_account_data(&mut blockchain, &address_to_create).await;

        assert_eq!(account_state.container, Pubkey::default(),);
        assert_eq!(account_state.version, StateVersion::Uninitialized);
    }

    test_initialize_server(
        &blockchain.payer,
        &dweller_owner,
        &server,
        &dweller_servers[0],
        &server_members[0],
        rent,
        blockchain.last_blockhash,
        &mut blockchain.banks_client,
    )
    .await;

    let server_state: Server = get_account_data(&mut blockchain, &server.pubkey()).await;
    assert_eq!(server_state.owner, dweller_owner.pubkey());
    assert_eq!(server_state.members, 1);

    // administrators and members
    let mut server_administrators = Vec::new();
    for index in 0u64..3 {
        let address_type = instruction::AddressTypeInput::ServerAdministrator(index);
        let seed = ServerAdministrator::SEED;
        let address_to_create = create_derived_account_index(
            &mut blockchain,
            &server.pubkey(),
            seed,
            index,
            address_type,
        )
        .await;
        server_administrators.push(address_to_create);
        let account_state: ServerAdministrator =
            get_account_data(&mut blockchain, &address_to_create).await;
        assert_eq!(account_state.container, Pubkey::default(),);
    }

    test_add_administrator(
        &blockchain.payer,
        &dweller_owner,
        &dweller_admin_1.pubkey(),
        &server.pubkey(),
        &server_administrators[0],
        blockchain.last_blockhash,
        &mut blockchain.banks_client,
    )
    .await;

    let account_state: Server = get_account_data(&mut blockchain, &server.pubkey()).await;
    assert!(account_state.administrators > 0);
    let account_state: ServerAdministrator =
        get_account_data(&mut blockchain, &server_administrators[0]).await;
    assert_eq!(account_state.container, server.pubkey());

    let mut server_member_statuses = Vec::new();
    for index in 0u64..3 {
        let address_type = instruction::AddressTypeInput::ServerMemberStatus(index);
        let seed = ServerMemberStatus::SEED;
        let address_to_create = create_derived_account_index(
            &mut blockchain,
            &server.pubkey(),
            seed,
            index,
            address_type,
        )
        .await;
        server_member_statuses.push(address_to_create);
        let account_state: ServerMemberStatus =
            get_account_data(&mut blockchain, &address_to_create).await;
        assert_eq!(account_state.container, Pubkey::default(),);
    }

    let trx = add_invite_transaction(
        &blockchain.payer,
        &server.pubkey(),
        &dweller_admin_1,
        &server_administrators[0],
        &dweller_1.pubkey(),
        &server_member_statuses[0],
        blockchain.last_blockhash,
    );
    blockchain
        .banks_client
        .process_transaction(trx)
        .await
        .unwrap();

    let account_state: ServerMemberStatus =
        get_account_data(&mut blockchain, &server_member_statuses[0]).await;
    assert_eq!(account_state.container, server.pubkey());
    assert_eq!(account_state.dweller, dweller_1.pubkey());
    assert_eq!(account_state.index, 0);

    let trx = join_server_transaction(
        &blockchain.payer,
        &server.pubkey(),
        &server_members[1],
        &server_member_statuses[0],
        &dweller_1,
        &dweller_servers[4],
        blockchain.last_blockhash,
    );

    blockchain
        .banks_client
        .process_transaction(trx)
        .await
        .unwrap();

    let account_state: ServerMemberStatus =
        get_account_data(&mut blockchain, &server_members[1]).await;
    assert_eq!(account_state.container, server.pubkey());
    assert_eq!(account_state.index, 1);
    assert_eq!(account_state.dweller, dweller_1.pubkey());

    let account_state: DwellerServer = get_account_data(&mut blockchain, &dweller_servers[4]).await;
    assert_eq!(account_state.container, dweller_1.pubkey());
    assert_eq!(account_state.server, server.pubkey());
    assert_eq!(account_state.index, 0);

    let account_state: Server = get_account_data(&mut blockchain, &server.pubkey()).await;
    assert_eq!(account_state.members, 2);

    // groups and channels

    let mut server_groups = Vec::new();
    for index in 0u64..3 {
        let address_type = instruction::AddressTypeInput::ServerGroup(index);
        let seed = ServerGroup::SEED;

        let address_to_create = create_derived_account_index(
            &mut blockchain,
            &server.pubkey(),
            seed,
            index,
            address_type,
        )
        .await;
        server_groups.push(address_to_create);
        let account_state: ServerGroup =
            get_account_data(&mut blockchain, &address_to_create).await;

        assert_eq!(account_state.container, Pubkey::default(),);
    }

    let mut group_channels = Vec::new();
    for index in 0u64..3 {
        let address_type = instruction::AddressTypeInput::GroupChannel(index);
        let seed = GroupChannel::SEED;
        let address_to_create = create_derived_account_index(
            &mut blockchain,
            &server_groups[0],
            seed,
            index,
            address_type,
        )
        .await;
        group_channels.push(address_to_create);
        let account_state: GroupChannel =
            get_account_data(&mut blockchain, &address_to_create).await;
        assert_eq!(account_state.container, Pubkey::default(),);
    }

    let mut server_channels = Vec::new();
    for index in 0u64..3 {
        let address_type = instruction::AddressTypeInput::ServerChannel(index);
        let seed = ServerChannel::SEED;
        let address_to_create = create_derived_account_index(
            &mut blockchain,
            &server.pubkey(),
            seed,
            index,
            address_type,
        )
        .await;
        server_channels.push(address_to_create);
        let account_state: ServerChannel =
            get_account_data(&mut blockchain, &address_to_create).await;
        assert_eq!(account_state.container, Pubkey::default(),);
    }

    let trx = create_group_transaction(
        &blockchain.payer,
        &dweller_admin_1,
        &server_administrators[0],
        &server.pubkey(),
        &server_groups[0],
        &CreateGroupInput { name: [66; 32] },
        blockchain.last_blockhash,
    );
    blockchain
        .banks_client
        .process_transaction(trx)
        .await
        .unwrap();

    let trx = add_channel_transaction(
        &blockchain.payer,
        &dweller_admin_1,
        &server_administrators[0],
        &server.pubkey(),
        &server_channels[0],
        &AddChannelInput {
            name: [66; 32],
            type_id: 17,
        },
        blockchain.last_blockhash,
    );
    blockchain
        .banks_client
        .process_transaction(trx)
        .await
        .unwrap();

    let trx = add_channel_to_group_transaction(
        &blockchain.payer,
        &server.pubkey(),
        &dweller_admin_1,
        &server_administrators[0],
        &server_channels[0],
        &server_groups[0],
        &group_channels[0],
        blockchain.last_blockhash,
    );
    blockchain
        .banks_client
        .process_transaction(trx)
        .await
        .unwrap();

    let account_state: ServerGroup = get_account_data(&mut blockchain, &server_groups[0]).await;
    assert_eq!(account_state.channels, 1);

    // removing/deleting

    // groups/channels

    let trx = remove_channel_from_group_transaction(
        &blockchain.payer,
        &server.pubkey(),
        &dweller_admin_1,
        &server_administrators[0],
        &server_groups[0],
        &group_channels[0],
        &group_channels[0],
        blockchain.last_blockhash,
    );
    blockchain
        .banks_client
        .process_transaction(trx)
        .await
        .unwrap();

    let trx = delete_group_transaction(
        &blockchain.payer,
        &dweller_admin_1,
        &server_administrators[0],
        &server.pubkey(),
        &server_groups[0],
        &server_groups[0],
        &group_channels[0],
        blockchain.last_blockhash,
    );

    blockchain
        .banks_client
        .process_transaction(trx)
        .await
        .unwrap();

    let account_state: Server = get_account_data(&mut blockchain, &server.pubkey()).await;
    assert_eq!(account_state.groups, 0);

    let trx = delete_channel_transaction(
        &blockchain.payer,
        &dweller_admin_1,
        &server_administrators[0],
        &server.pubkey(),
        &server_channels[0],
        &server_channels[0],
        blockchain.last_blockhash,
    );

    blockchain
        .banks_client
        .process_transaction(trx)
        .await
        .unwrap();

    let account_state: Server = get_account_data(&mut blockchain, &server.pubkey()).await;
    assert_eq!(account_state.channels, 0);

    // members/admin

    let trx = leave_server_transaction(
        &blockchain.payer,
        &server.pubkey(),
        &server_members[1],
        &server_members[1],
        &dweller_1,
        &dweller_servers[4],
        &dweller_servers[4],
        blockchain.last_blockhash,
    );

    blockchain
        .banks_client
        .process_transaction(trx)
        .await
        .unwrap();

    let account_state: Server = get_account_data(&mut blockchain, &server.pubkey()).await;
    assert_eq!(account_state.members, 1);

    let trx = revoke_invite_server_transaction(
        &blockchain.payer,
        &server.pubkey(),
        &dweller_admin_1,
        &server_administrators[0],
        &server_member_statuses[0],
        &server_member_statuses[0],
        blockchain.last_blockhash,
    );

    blockchain
        .banks_client
        .process_transaction(trx)
        .await
        .unwrap();

    let account_state: Server = get_account_data(&mut blockchain, &server.pubkey()).await;
    assert_eq!(account_state.member_statuses, 0);

    let trx = remove_admin_transaction(
        &blockchain.payer,
        &dweller_owner,
        &server.pubkey(),
        &server_administrators[0],
        &server_administrators[0],
        blockchain.last_blockhash,
    );

    blockchain
        .banks_client
        .process_transaction(trx)
        .await
        .unwrap();

    let account_state: Server = get_account_data(&mut blockchain, &server.pubkey()).await;
    assert_eq!(account_state.administrators, 0);
}

pub async fn create_derived_account_index(
    blockchain: &mut ProgramTestContext,
    owner: &Pubkey,
    seed: &str,
    index: u64,
    address_type: instruction::AddressTypeInput,
) -> Pubkey {
    let (address_to_create, base_program_address, ..) =
        satellite_servers::program::create_base_index_with_seed(&id(), seed, owner, index).unwrap();
    test_create_derived_account(
        blockchain,
        owner,
        &base_program_address,
        &address_to_create,
        address_type,
    )
    .await
    .unwrap();
    address_to_create
}

pub async fn test_initialize_dweller(
    payer: &Keypair,
    dweller_owner: &Keypair,
    rent: solana_program::rent::Rent,
    recent_blockhash: solana_program::hash::Hash,
    blockchain: &mut BanksClient,
) {
    let mut transaction = Transaction::new_with_payer(
        &[
            system_instruction::create_account(
                &payer.pubkey(),
                &dweller_owner.pubkey(),
                rent.minimum_balance(Dweller::LEN as usize),
                Dweller::LEN as u64,
                &satellite_servers::id(),
            ),
            instruction::initialize_dweller(
                &dweller_owner.pubkey(),
                InitializeDwellerInput { name: [42; 32] },
            )
            .unwrap(),
        ],
        Some(&payer.pubkey()),
    );
    transaction.sign(&[&payer, dweller_owner], recent_blockhash);
    blockchain.process_transaction(transaction).await.unwrap();
}

pub async fn test_add_administrator(
    payer: &Keypair,
    dweller_owner: &Keypair,
    dweller: &Pubkey,
    server: &Pubkey,
    server_administrator: &Pubkey,
    recent_blockhash: solana_program::hash::Hash,
    blockchain: &mut BanksClient,
) {
    let mut transaction = Transaction::new_with_payer(
        &[instruction::add_admin(
            &dweller_owner.pubkey(),
            dweller,
            server,
            server_administrator,
        )
        .unwrap()],
        Some(&payer.pubkey()),
    );
    transaction.sign(&[&payer, dweller_owner], recent_blockhash);
    blockchain.process_transaction(transaction).await.unwrap();
}

#[allow(clippy::too_many_arguments)]
pub async fn test_initialize_server(
    payer: &Keypair,
    dweller_owner: &Keypair,
    server: &Keypair,
    dweller_server: &Pubkey,
    server_member: &Pubkey,
    rent: solana_program::rent::Rent,
    recent_blockhash: solana_program::hash::Hash,
    blockchain: &mut BanksClient,
) {
    let mut transaction = Transaction::new_with_payer(
        &[
            system_instruction::create_account(
                &payer.pubkey(),
                &server.pubkey(),
                rent.minimum_balance(Server::LEN as usize),
                Server::LEN as u64,
                &satellite_servers::id(),
            ),
            instruction::initialize_server(
                &dweller_owner.pubkey(),
                &server.pubkey(),
                dweller_server,
                server_member,
                InitializeServerInput { name: [13; 32] },
            )
            .unwrap(),
        ],
        Some(&payer.pubkey()),
    );
    transaction.sign(&[&payer, server, dweller_owner], recent_blockhash);
    blockchain.process_transaction(transaction).await.unwrap();
}

pub async fn get_account(program_context: &mut ProgramTestContext, pubkey: &Pubkey) -> Account {
    program_context
        .banks_client
        .get_account(*pubkey)
        .await
        .expect("account not found")
        .expect("account empty")
}

pub async fn get_account_data<T: BorshDeserialize>(
    program_context: &mut ProgramTestContext,
    pubkey: &Pubkey,
) -> T {
    program_context
        .banks_client
        .get_account_data_with_borsh(*pubkey)
        .await
        .expect("account not found")
}
