#![cfg(feature = "test-bpf")]

use solana_program::{pubkey::Pubkey, system_instruction};
use solana_program_test::*;
use solana_sdk::{account::Account, signature::{Keypair, Signer}, transaction::Transaction};

use crate::{id, instruction::{self, InitializeDwellerInput, InitializeServerInput}, processor, state::*};

pub fn program_test() -> ProgramTest {
    ProgramTest::new(
        "sattelite_server",
        id(),
        processor!(processor::Processor::process_instruction),
    )
}


#[tokio::test]
async fn test_create_address() {
    let mut program_context = program_test().start_with_context().await;
    let rent = program_context.banks_client.get_rent().await.unwrap();

    let dweller = Keypair::new();

    test_initialize_dweller(&program_context.payer, &dweller, rent, program_context.last_blockhash,&mut program_context.banks_client).await;

    let (base_program_address, bump_seed) = Pubkey::find_program_address(
        &[&dweller.pubkey().to_bytes()[..32]],
        &id(),
    );
    let address_to_create = Pubkey::create_with_seed(&base_program_address, "DwellerServer", &id()).unwrap();

    let mut transaction = Transaction::new_with_payer(
        &[instruction::create_derived_account(
            &id(),
            &program_context.payer.pubkey(),
            &dweller.pubkey(),
            &base_program_address,
            &address_to_create,
            instruction::AddressTypeInput::DwellerServer(1),
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
    
    let dweller_server_info_data = get_account(&mut program_context, &address_to_create).await;

    assert_eq!(dweller_server_info_data.data.len(), DwellerServer::LEN as usize);
}

//#[tokio::test]
async fn flow() {
    let (mut blockchain, payer, recent_blockhash) = program_test().start().await;
    let rent = blockchain.get_rent().await.unwrap();

    let dweller_owner = Keypair::new();
    let dweller_admin_1 = Keypair::new();
    let dweller_admin_2 = Keypair::new();
    let dweller_admin_3 = Keypair::new();
    let dweller_1 = Keypair::new();
    let dweller_2 = Keypair::new();
    let dweller_3 = Keypair::new();
    let server = Keypair::new();

    test_initialize_dweller(&payer, &dweller_owner, rent, recent_blockhash,&mut blockchain).await;

    // test_initialize_dweller(&payer, dweller_admin_1, rent, recent_blockhash,&mut blockchain).await;
    // test_initialize_dweller(&payer, dweller_admin_2, rent, recent_blockhash,&mut blockchain).await;   
    // test_initialize_dweller(&payer, dweller_admin_3, rent, recent_blockhash,&mut blockchain).await;   
    // test_initialize_dweller(&payer, dweller_1, rent, recent_blockhash,&mut blockchain).await;   
    // test_initialize_dweller(&payer, dweller_2, rent, recent_blockhash,&mut blockchain).await;   
    // test_initialize_dweller(&payer, dweller_3, rent, recent_blockhash,&mut blockchain).await;   
}

async fn test_initialize_dweller(payer: &Keypair, dweller_owner: &Keypair, rent: solana_program::rent::Rent, recent_blockhash: solana_program::hash::Hash, blockchain:&mut BanksClient) {
    let mut transaction = Transaction::new_with_payer(
        &[
            system_instruction::create_account(
                &payer.pubkey(),
                &dweller_owner.pubkey(),
                rent.minimum_balance(Dweller::LEN as usize),
                Dweller::LEN as u64,
                &crate::id(),
            ),
            instruction::initialize_dweller(                
                &dweller_owner.pubkey(),
                InitializeDwellerInput{ name : [42;32]},                
            )
            .unwrap(),
        ],
        Some(&payer.pubkey()),
    );
    transaction.sign(&[&payer, dweller_owner], recent_blockhash);
    blockchain.process_transaction(transaction).await.unwrap();
}

async fn test_initialize_server(payer: &Keypair, dweller_owner: &Keypair, server: &Keypair,  dweller_server:&Pubkey, server_member:&Pubkey, rent: solana_program::rent::Rent, recent_blockhash: solana_program::hash::Hash, blockchain:&mut BanksClient) {
    let mut transaction = Transaction::new_with_payer(
        &[
            system_instruction::create_account(
                &payer.pubkey(),
                &dweller_owner.pubkey(),
                rent.minimum_balance(Dweller::LEN as usize),
                Dweller::LEN as u64,
                &crate::id(),
            ),
            instruction::initialize_server(                
                &dweller_owner.pubkey(),
                &server.pubkey(),
                dweller_server,
                server_member,
                InitializeServerInput{ name : [13;32] },                
            )
            .unwrap(),
        ],
        Some(&payer.pubkey()),
    );
    transaction.sign(&[&payer, dweller_owner], recent_blockhash);
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