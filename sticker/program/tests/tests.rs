#![cfg(feature = "test-bpf")]

use borsh::de::BorshDeserialize;
use satellite_stickers::*;
use solana_program::pubkey::Pubkey;
use solana_program::{program_pack::Pack, system_instruction};
use solana_program_test::*;
use solana_sdk::{
    account::Account,
    signature::{Keypair, Signer},
    transaction::Transaction,
    transport::TransportError,
};

pub fn program_test() -> ProgramTest {
    let mut program = ProgramTest::new(
        "satellite_stickers",
        id(),
        processor!(processor::Processor::process_instruction),
    );
    program.add_program("spl_nft_erc_721", spl_nft_erc_721::id(), None);
    program
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
    account_to_create: &Keypair,
    lamports: u64,
    space: u64,
    owner: &Pubkey,
) -> Result<(), TransportError> {
    let mut transaction = Transaction::new_with_payer(
        &[system_instruction::create_account(
            &program_context.payer.pubkey(),
            &account_to_create.pubkey(),
            lamports,
            space,
            owner,
        )],
        Some(&program_context.payer.pubkey()),
    );
    transaction.sign(
        &[&program_context.payer, account_to_create],
        program_context.last_blockhash,
    );
    program_context
        .banks_client
        .process_transaction(transaction)
        .await?;
    Ok(())
}

pub async fn create_token_account(
    program_context: &mut ProgramTestContext,
    account: &Keypair,
    account_rent: u64,
    mint: &Pubkey,
    owner: &Pubkey,
) -> Result<(), TransportError> {
    let instructions = vec![
        system_instruction::create_account(
            &program_context.payer.pubkey(),
            &account.pubkey(),
            account_rent,
            spl_token::state::Account::LEN as u64,
            &spl_token::id(),
        ),
        spl_token::instruction::initialize_account(
            &spl_token::id(),
            &account.pubkey(),
            mint,
            owner,
        )
        .unwrap(),
    ];

    let mut transaction =
        Transaction::new_with_payer(&instructions, Some(&program_context.payer.pubkey()));

    transaction.sign(
        &[&program_context.payer, account],
        program_context.last_blockhash,
    );
    program_context
        .banks_client
        .process_transaction(transaction)
        .await?;
    Ok(())
}

pub async fn create_program_account(
    program_context: &mut ProgramTestContext,
    sticker_factory: &Pubkey,
    base_program_address: &Pubkey,
    address_to_create: &Pubkey,
    address_type: instruction::AddressType,
) -> Result<(), TransportError> {
    let mut transaction = Transaction::new_with_payer(
        &[instruction::create_account(
            &id(),
            &program_context.payer.pubkey(),
            sticker_factory,
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

pub async fn create_sticker_factory(
    program_context: &mut ProgramTestContext,
    sticker_factory: &Keypair,
    sticker_factory_owner: &Keypair,
) -> Result<(), TransportError> {
    let mut transaction = Transaction::new_with_payer(
        &[instruction::create_sticker_factory(
            &id(),
            &sticker_factory.pubkey(),
            &sticker_factory_owner.pubkey(),
        )
        .unwrap()],
        Some(&program_context.payer.pubkey()),
    );
    transaction.sign(
        &[&program_context.payer, sticker_factory_owner],
        program_context.last_blockhash,
    );
    program_context
        .banks_client
        .process_transaction(transaction)
        .await?;
    Ok(())
}

pub async fn register_new_artist(
    program_context: &mut ProgramTestContext,
    user: &Pubkey,
    user_token_acc: &Pubkey,
    artist_to_create: &Pubkey,
    sticker_factory_owner: &Keypair,
    sticker_factory: &Pubkey,
    args: instruction::RegisterArtist,
) -> Result<(), TransportError> {
    let mut transaction = Transaction::new_with_payer(
        &[instruction::register_artist(
            &id(),
            user,
            user_token_acc,
            artist_to_create,
            &sticker_factory_owner.pubkey(),
            sticker_factory,
            args,
        )
        .unwrap()],
        Some(&program_context.payer.pubkey()),
    );
    transaction.sign(
        &[&program_context.payer, sticker_factory_owner],
        program_context.last_blockhash,
    );
    program_context
        .banks_client
        .process_transaction(transaction)
        .await?;
    Ok(())
}

pub async fn create_new_sticker(
    program_context: &mut ProgramTestContext,
    sticker: &Pubkey,
    sticker_factory: &Pubkey,
    mint: &Pubkey,
    artist: &Pubkey,
    user: &Keypair,
    mint_authority: &Pubkey,
    args: instruction::CreateNewSticker,
) -> Result<(), TransportError> {
    let mut transaction = Transaction::new_with_payer(
        &[instruction::create_new_sticker(
            &id(),
            sticker,
            sticker_factory,
            mint,
            artist,
            &user.pubkey(),
            mint_authority,
            args,
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

pub async fn buy_sticker(
    program_context: &mut ProgramTestContext,
    sticker_to_buy: &Pubkey,
    artist_account: &Pubkey,
    artist_token_acc: &Pubkey,
    buyer_token_acc: &Pubkey,
    buyer_transfer_authority: &Keypair,
    mint_authority: &Pubkey,
    nft_token: &Keypair,
    nft_token_rent: u64,
    nft_token_data_rent: u64,
    nft_token_mint: &Pubkey,
    nft_token_owner: &Pubkey,
) -> Result<Pubkey, TransportError> {
    let seed = "token_data";
    let token_data =
        Pubkey::create_with_seed(&nft_token.pubkey(), seed, &spl_nft_erc_721::id()).unwrap();

    let mut transaction = Transaction::new_with_payer(
        &[
            system_instruction::create_account(
                &program_context.payer.pubkey(),
                &nft_token.pubkey(),
                nft_token_rent,
                spl_nft_erc_721::state::Token::LEN,
                &spl_nft_erc_721::id(),
            ),
            system_instruction::create_account_with_seed(
                &program_context.payer.pubkey(),
                &token_data,
                &nft_token.pubkey(),
                seed,
                nft_token_data_rent,
                spl_nft_erc_721::state::TokenData::LEN,
                &spl_nft_erc_721::id(),
            ),
            instruction::buy_sticker(
                &id(),
                sticker_to_buy,
                artist_account,
                artist_token_acc,
                buyer_token_acc,
                &buyer_transfer_authority.pubkey(),
                mint_authority,
                &nft_token.pubkey(),
                &token_data,
                nft_token_mint,
                nft_token_owner,
            )
            .unwrap(),
        ],
        Some(&program_context.payer.pubkey()),
    );
    transaction.sign(
        &[&program_context.payer, buyer_transfer_authority, nft_token],
        program_context.last_blockhash,
    );
    program_context
        .banks_client
        .process_transaction(transaction)
        .await?;
    Ok(token_data)
}

pub async fn change_sticker_price(
    program_context: &mut ProgramTestContext,
    sticker: &Pubkey,
    creator: &Keypair,
    new_price: u64,
) -> Result<(), TransportError> {
    let mut transaction = Transaction::new_with_payer(
        &[
            instruction::change_sticker_price(&id(), sticker, &creator.pubkey(), new_price)
                .unwrap(),
        ],
        Some(&program_context.payer.pubkey()),
    );
    transaction.sign(
        &[&program_context.payer, creator],
        program_context.last_blockhash,
    );
    program_context
        .banks_client
        .process_transaction(transaction)
        .await?;
    Ok(())
}

#[tokio::test]
async fn test_create_account_instruction() {
    let mut program_context = program_test().start_with_context().await;

    let sticker_factory = Keypair::new();
    let sticker_factory_owner = Keypair::new();

    let rent = program_context.banks_client.get_rent().await.unwrap();
    let sticker_factory_min_rent = rent.minimum_balance(state::StickerFactory::LEN);

    create_account(
        &mut program_context,
        &sticker_factory,
        sticker_factory_min_rent,
        state::StickerFactory::LEN as u64,
        &id(),
    )
    .await
    .unwrap();

    create_sticker_factory(
        &mut program_context,
        &sticker_factory,
        &sticker_factory_owner,
    )
    .await
    .unwrap();

    let sticker_factory_data = get_account(&mut program_context, &sticker_factory.pubkey()).await;
    let sticker_factory =
        state::StickerFactory::try_from_slice(&sticker_factory_data.data.as_slice()).unwrap();

    assert!(sticker_factory.is_initialized());
}

#[tokio::test]
async fn test_register_artist_instruction() {
    let mut program_context = program_test().start_with_context().await;

    let sticker_factory = Keypair::new();
    let sticker_factory_owner = Keypair::new();

    let rent = program_context.banks_client.get_rent().await.unwrap();
    let sticker_factory_min_rent = rent.minimum_balance(state::StickerFactory::LEN);
    let token_account_rent = rent.minimum_balance(spl_token::state::Account::LEN);

    create_account(
        &mut program_context,
        &sticker_factory,
        sticker_factory_min_rent,
        state::StickerFactory::LEN as u64,
        &id(),
    )
    .await
    .unwrap();

    create_sticker_factory(
        &mut program_context,
        &sticker_factory,
        &sticker_factory_owner,
    )
    .await
    .unwrap();

    let factory_info_data = get_account(&mut program_context, &sticker_factory.pubkey()).await;
    let factory_info =
        state::StickerFactory::try_from_slice(&factory_info_data.data.as_slice()).unwrap();

    let (base, _) =
        Pubkey::find_program_address(&[&sticker_factory.pubkey().to_bytes()[..32]], &id());
    let address_to_create = Pubkey::create_with_seed(
        &base,
        &format!(
            "{:?}{}",
            factory_info.artist_count,
            processor::Processor::ARTIST_SEED
        ),
        &id(),
    )
    .unwrap();
    create_program_account(
        &mut program_context,
        &sticker_factory.pubkey(),
        &base,
        &address_to_create,
        instruction::AddressType::Artist,
    )
    .await
    .unwrap();

    let artist_user = Keypair::new();
    let artist_token_acc = Keypair::new();
    let owner = Keypair::new();

    create_token_account(
        &mut program_context,
        &artist_token_acc,
        token_account_rent,
        &spl_token::native_mint::id(),
        &owner.pubkey(),
    )
    .await
    .unwrap();

    let artist_data = instruction::RegisterArtist {
        name: [1; 32],
        signature: [2; 256],
        description: [3; 256],
    };

    let sticker_factory_data = get_account(&mut program_context, &sticker_factory.pubkey()).await;
    let sticker_factory_acc =
        state::StickerFactory::try_from_slice(&sticker_factory_data.data.as_slice()).unwrap();
    let artist_index_before = sticker_factory_acc.artist_count;

    register_new_artist(
        &mut program_context,
        &artist_user.pubkey(),
        &artist_token_acc.pubkey(),
        &address_to_create,
        &sticker_factory_owner,
        &sticker_factory.pubkey(),
        artist_data,
    )
    .await
    .unwrap();

    let artist_acc_data = get_account(&mut program_context, &address_to_create).await;
    let artist_acc = state::Artist::try_from_slice(&artist_acc_data.data.as_slice()).unwrap();

    let sticker_factory_data = get_account(&mut program_context, &sticker_factory.pubkey()).await;
    let sticker_factory_acc =
        state::StickerFactory::try_from_slice(&sticker_factory_data.data.as_slice()).unwrap();

    assert!(artist_acc.is_initialized());
    assert_eq!(artist_acc.user, artist_user.pubkey());
    assert_eq!(artist_acc.user_token_acc, artist_token_acc.pubkey());
    assert_eq!(sticker_factory_acc.artist_count, artist_index_before + 1);
}

#[tokio::test]
async fn test_create_new_sticker_instruction() {
    let mut program_context = program_test().start_with_context().await;

    let sticker_factory = Keypair::new();
    let sticker_factory_owner = Keypair::new();

    let rent = program_context.banks_client.get_rent().await.unwrap();
    let sticker_factory_min_rent = rent.minimum_balance(state::StickerFactory::LEN);
    let token_account_rent = rent.minimum_balance(spl_token::state::Account::LEN);
    let nft_mint_rent = rent.minimum_balance(spl_nft_erc_721::state::Mint::LEN as usize);

    create_account(
        &mut program_context,
        &sticker_factory,
        sticker_factory_min_rent,
        state::StickerFactory::LEN as u64,
        &id(),
    )
    .await
    .unwrap();

    create_sticker_factory(
        &mut program_context,
        &sticker_factory,
        &sticker_factory_owner,
    )
    .await
    .unwrap();

    let factory_info_data = get_account(&mut program_context, &sticker_factory.pubkey()).await;
    let factory_info =
        state::StickerFactory::try_from_slice(&factory_info_data.data.as_slice()).unwrap();

    let (base, _) =
        Pubkey::find_program_address(&[&sticker_factory.pubkey().to_bytes()[..32]], &id());
    let artist_key = Pubkey::create_with_seed(
        &base,
        &format!(
            "{:?}{}",
            factory_info.artist_count,
            processor::Processor::ARTIST_SEED
        ),
        &id(),
    )
    .unwrap();
    create_program_account(
        &mut program_context,
        &sticker_factory.pubkey(),
        &base,
        &artist_key,
        instruction::AddressType::Artist,
    )
    .await
    .unwrap();

    let artist_user = Keypair::new();
    let artist_token_acc = Keypair::new();
    let owner = Keypair::new();

    create_token_account(
        &mut program_context,
        &artist_token_acc,
        token_account_rent,
        &spl_token::native_mint::id(),
        &owner.pubkey(),
    )
    .await
    .unwrap();

    let artist_data = instruction::RegisterArtist {
        name: [1; 32],
        signature: [2; 256],
        description: [3; 256],
    };
    register_new_artist(
        &mut program_context,
        &artist_user.pubkey(),
        &artist_token_acc.pubkey(),
        &artist_key,
        &sticker_factory_owner,
        &sticker_factory.pubkey(),
        artist_data,
    )
    .await
    .unwrap();

    let (base, _) =
        Pubkey::find_program_address(&[&sticker_factory.pubkey().to_bytes()[..32]], &id());
    let sticker_key = Pubkey::create_with_seed(
        &base,
        &format!(
            "{:?}{}",
            factory_info.sticker_count,
            processor::Processor::STICKER_SEED
        ),
        &id(),
    )
    .unwrap();
    create_program_account(
        &mut program_context,
        &sticker_factory.pubkey(),
        &base,
        &sticker_key,
        instruction::AddressType::Sticker,
    )
    .await
    .unwrap();

    let nft_mint = Keypair::new();

    // create nft mint account
    create_account(
        &mut program_context,
        &nft_mint,
        nft_mint_rent,
        spl_nft_erc_721::state::Mint::LEN,
        &spl_nft_erc_721::id(),
    )
    .await
    .unwrap();

    let (mint_auth, _) = Pubkey::find_program_address(&[&sticker_key.to_bytes()[..32]], &id());

    let sticker_data = instruction::CreateNewSticker {
        max_supply: 1000,
        price: 100,
        uri: [5; 256],
        symbol: [7; 8],
        name: [6; 32],
    };

    let sticker_factory_data = get_account(&mut program_context, &sticker_factory.pubkey()).await;
    let sticker_factory_acc =
        state::StickerFactory::try_from_slice(&sticker_factory_data.data.as_slice()).unwrap();
    let sticker_index_before = sticker_factory_acc.sticker_count;

    create_new_sticker(
        &mut program_context,
        &sticker_key,
        &sticker_factory.pubkey(),
        &nft_mint.pubkey(),
        &artist_key,
        &artist_user,
        &mint_auth,
        sticker_data.clone(),
    )
    .await
    .unwrap();

    let sticker_acc_data = get_account(&mut program_context, &sticker_key).await;
    let sticker_acc = state::Sticker::try_from_slice(&sticker_acc_data.data.as_slice()).unwrap();

    assert!(sticker_acc.is_initialized());
    assert_eq!(sticker_acc.creator, artist_user.pubkey());
    assert_eq!(sticker_acc.supply, 0);
    assert_eq!(sticker_acc.max_supply, sticker_data.max_supply);
    assert_eq!(sticker_acc.price, sticker_data.price);
    assert_eq!(sticker_acc.mint, nft_mint.pubkey());
    assert_eq!(sticker_acc.uri, sticker_data.uri);

    let sticker_factory_data = get_account(&mut program_context, &sticker_factory.pubkey()).await;
    let sticker_factory_acc =
        state::StickerFactory::try_from_slice(&sticker_factory_data.data.as_slice()).unwrap();

    assert_eq!(sticker_factory_acc.sticker_count, sticker_index_before + 1);
}

#[tokio::test]
async fn test_buy_sticker_instruction() {
    let mut program_context = program_test().start_with_context().await;

    let sticker_factory = Keypair::new();
    let sticker_factory_owner = Keypair::new();

    let rent = program_context.banks_client.get_rent().await.unwrap();
    let sticker_factory_min_rent = rent.minimum_balance(state::StickerFactory::LEN);
    let token_account_rent = rent.minimum_balance(spl_token::state::Account::LEN);
    let nft_mint_rent = rent.minimum_balance(spl_nft_erc_721::state::Mint::LEN as usize);
    let nft_token_acc_rent = rent.minimum_balance(spl_nft_erc_721::state::Token::LEN as usize);
    let nft_token_data_acc_rent =
        rent.minimum_balance(spl_nft_erc_721::state::TokenData::LEN as usize);

    create_account(
        &mut program_context,
        &sticker_factory,
        sticker_factory_min_rent,
        state::StickerFactory::LEN as u64,
        &id(),
    )
    .await
    .unwrap();

    create_sticker_factory(
        &mut program_context,
        &sticker_factory,
        &sticker_factory_owner,
    )
    .await
    .unwrap();

    let factory_info_data = get_account(&mut program_context, &sticker_factory.pubkey()).await;
    let factory_info =
        state::StickerFactory::try_from_slice(&factory_info_data.data.as_slice()).unwrap();

    let (base, _) =
        Pubkey::find_program_address(&[&sticker_factory.pubkey().to_bytes()[..32]], &id());
    let artist_key = Pubkey::create_with_seed(
        &base,
        &format!(
            "{:?}{}",
            factory_info.artist_count,
            processor::Processor::ARTIST_SEED
        ),
        &id(),
    )
    .unwrap();
    create_program_account(
        &mut program_context,
        &sticker_factory.pubkey(),
        &base,
        &artist_key,
        instruction::AddressType::Artist,
    )
    .await
    .unwrap();

    let artist_user = Keypair::new();
    let artist_token_acc = Keypair::new();
    let owner = Keypair::new();

    create_token_account(
        &mut program_context,
        &artist_token_acc,
        token_account_rent,
        &spl_token::native_mint::id(),
        &owner.pubkey(),
    )
    .await
    .unwrap();

    let artist_data = instruction::RegisterArtist {
        name: [1; 32],
        signature: [2; 256],
        description: [3; 256],
    };
    register_new_artist(
        &mut program_context,
        &artist_user.pubkey(),
        &artist_token_acc.pubkey(),
        &artist_key,
        &sticker_factory_owner,
        &sticker_factory.pubkey(),
        artist_data,
    )
    .await
    .unwrap();

    let (base, _) =
        Pubkey::find_program_address(&[&sticker_factory.pubkey().to_bytes()[..32]], &id());
    let sticker_key = Pubkey::create_with_seed(
        &base,
        &format!(
            "{:?}{}",
            factory_info.sticker_count,
            processor::Processor::STICKER_SEED
        ),
        &id(),
    )
    .unwrap();
    create_program_account(
        &mut program_context,
        &sticker_factory.pubkey(),
        &base,
        &sticker_key,
        instruction::AddressType::Sticker,
    )
    .await
    .unwrap();

    let nft_mint = Keypair::new();

    // create nft mint account
    create_account(
        &mut program_context,
        &nft_mint,
        nft_mint_rent,
        spl_nft_erc_721::state::Mint::LEN,
        &spl_nft_erc_721::id(),
    )
    .await
    .unwrap();

    let (mint_auth, _) = Pubkey::find_program_address(&[&sticker_key.to_bytes()[..32]], &id());

    let sticker_price = 100;
    let url = url::Url::parse("ipfs://bafybeiemxf5abjwjbikoz4mc3a3dla6ual3jsgpdr4cjr3oz3evfyavhwq/wiki/Vincent_van_Gogh.html").unwrap();
    let mut uri = [0; 256];
    let url_bytes = url.as_str().as_bytes();
    let (left, _) = uri.split_at_mut(url_bytes.len());
    left.copy_from_slice(url_bytes);

    let sticker_data = instruction::CreateNewSticker {
        max_supply: 1000,
        price: sticker_price,
        uri,
        symbol: [7; 8],
        name: [6; 32],
    };

    create_new_sticker(
        &mut program_context,
        &sticker_key,
        &sticker_factory.pubkey(),
        &nft_mint.pubkey(),
        &artist_key,
        &artist_user,
        &mint_auth,
        sticker_data,
    )
    .await
    .unwrap();

    let user_token_acc = Keypair::new();
    let user_owner = Keypair::new();

    create_token_account(
        &mut program_context,
        &user_token_acc,
        token_account_rent + sticker_price,
        &spl_token::native_mint::id(),
        &user_owner.pubkey(),
    )
    .await
    .unwrap();

    let user_nft_token_acc = Keypair::new();
    let user_nft_token_owner = Keypair::new();

    let user_token_account_data = get_account(&mut program_context, &user_token_acc.pubkey()).await;
    let user_token_acc_balance_before = user_token_account_data.lamports;

    let artist_token_account_data =
        get_account(&mut program_context, &artist_token_acc.pubkey()).await;
    let artist_token_acc_balance_before = artist_token_account_data.lamports;

    let sticker_account_data = get_account(&mut program_context, &sticker_key).await;
    let sticker_acc =
        state::Sticker::try_from_slice(&sticker_account_data.data.as_slice()).unwrap();
    let sticker_supply_before = sticker_acc.supply;

    let token_data = buy_sticker(
        &mut program_context,
        &sticker_key,
        &artist_key,
        &artist_token_acc.pubkey(),
        &user_token_acc.pubkey(),
        &user_owner,
        &mint_auth,
        &user_nft_token_acc,
        nft_token_acc_rent,
        nft_token_data_acc_rent,
        &nft_mint.pubkey(),
        &user_nft_token_owner.pubkey(),
    )
    .await
    .unwrap();

    let user_nft_token_account_data =
        get_account(&mut program_context, &user_nft_token_acc.pubkey()).await;
    let user_nft_token_account = spl_nft_erc_721::state::Token::deserialize(
        &mut &user_nft_token_account_data.data.as_slice()[..],
    )
    .unwrap();

    assert_eq!(user_nft_token_account.mint, nft_mint.pubkey());
    assert_eq!(user_nft_token_account.owner, user_nft_token_owner.pubkey());

    let user_nft_token_data_account = get_account(&mut program_context, &token_data).await;
    let user_nft_token_data_account = spl_nft_erc_721::state::TokenData::deserialize(
        &mut &user_nft_token_data_account.data.as_slice()[..],
    )
    .unwrap();

    assert_eq!(
        user_nft_token_data_account.token,
        user_nft_token_acc.pubkey()
    );
    assert_eq!(
        url_bytes,
        &user_nft_token_data_account.uri[0..url_bytes.len()]
    );

    let user_token_account_data = get_account(&mut program_context, &user_token_acc.pubkey()).await;
    let artist_token_account_data =
        get_account(&mut program_context, &artist_token_acc.pubkey()).await;
    let sticker_account_data = get_account(&mut program_context, &sticker_key).await;
    let sticker_acc =
        state::Sticker::try_from_slice(&sticker_account_data.data.as_slice()).unwrap();

    assert_eq!(
        user_token_account_data.lamports,
        user_token_acc_balance_before - sticker_price
    );
    assert_eq!(
        artist_token_account_data.lamports,
        artist_token_acc_balance_before + sticker_price
    );
    assert_eq!(sticker_acc.supply, sticker_supply_before + 1);
}

#[tokio::test]
async fn test_change_sticker_price_instruction() {
    let mut program_context = program_test().start_with_context().await;

    let sticker_factory = Keypair::new();
    let sticker_factory_owner = Keypair::new();

    let rent = program_context.banks_client.get_rent().await.unwrap();
    let sticker_factory_min_rent = rent.minimum_balance(state::StickerFactory::LEN);
    let token_account_rent = rent.minimum_balance(spl_token::state::Account::LEN);
    let nft_mint_rent = rent.minimum_balance(spl_nft_erc_721::state::Mint::LEN as usize);

    create_account(
        &mut program_context,
        &sticker_factory,
        sticker_factory_min_rent,
        state::StickerFactory::LEN as u64,
        &id(),
    )
    .await
    .unwrap();

    create_sticker_factory(
        &mut program_context,
        &sticker_factory,
        &sticker_factory_owner,
    )
    .await
    .unwrap();

    let factory_info_data = get_account(&mut program_context, &sticker_factory.pubkey()).await;
    let factory_info =
        state::StickerFactory::try_from_slice(&factory_info_data.data.as_slice()).unwrap();

    let (base, _) =
        Pubkey::find_program_address(&[&sticker_factory.pubkey().to_bytes()[..32]], &id());
    let artist_key = Pubkey::create_with_seed(
        &base,
        &format!(
            "{:?}{}",
            factory_info.artist_count,
            processor::Processor::ARTIST_SEED
        ),
        &id(),
    )
    .unwrap();
    create_program_account(
        &mut program_context,
        &sticker_factory.pubkey(),
        &base,
        &artist_key,
        instruction::AddressType::Artist,
    )
    .await
    .unwrap();

    let artist_user = Keypair::new();
    let artist_token_acc = Keypair::new();
    let owner = Keypair::new();

    create_token_account(
        &mut program_context,
        &artist_token_acc,
        token_account_rent,
        &spl_token::native_mint::id(),
        &owner.pubkey(),
    )
    .await
    .unwrap();

    let artist_data = instruction::RegisterArtist {
        name: [1; 32],
        signature: [2; 256],
        description: [3; 256],
    };
    register_new_artist(
        &mut program_context,
        &artist_user.pubkey(),
        &artist_token_acc.pubkey(),
        &artist_key,
        &sticker_factory_owner,
        &sticker_factory.pubkey(),
        artist_data,
    )
    .await
    .unwrap();

    let (base, _) =
        Pubkey::find_program_address(&[&sticker_factory.pubkey().to_bytes()[..32]], &id());
    let sticker_key = Pubkey::create_with_seed(
        &base,
        &format!(
            "{:?}{}",
            factory_info.sticker_count,
            processor::Processor::STICKER_SEED
        ),
        &id(),
    )
    .unwrap();
    create_program_account(
        &mut program_context,
        &sticker_factory.pubkey(),
        &base,
        &sticker_key,
        instruction::AddressType::Sticker,
    )
    .await
    .unwrap();

    let nft_mint = Keypair::new();

    // create nft mint account
    create_account(
        &mut program_context,
        &nft_mint,
        nft_mint_rent,
        spl_nft_erc_721::state::Mint::LEN,
        &spl_nft_erc_721::id(),
    )
    .await
    .unwrap();

    let (mint_auth, _) = Pubkey::find_program_address(&[&sticker_key.to_bytes()[..32]], &id());

    let sticker_price = 100;
    let url = url::Url::parse("ipfs://bafybeiemxf5abjwjbikoz4mc3a3dla6ual3jsgpdr4cjr3oz3evfyavhwq/wiki/Vincent_van_Gogh.html").unwrap();
    let mut uri = [0; 256];
    let url_bytes = url.as_str().as_bytes();
    let (left, _) = uri.split_at_mut(url_bytes.len());
    left.copy_from_slice(url_bytes);

    let sticker_data = instruction::CreateNewSticker {
        max_supply: 1000,
        price: sticker_price,
        uri,
        symbol: [7; 8],
        name: [6; 32],
    };

    create_new_sticker(
        &mut program_context,
        &sticker_key,
        &sticker_factory.pubkey(),
        &nft_mint.pubkey(),
        &artist_key,
        &artist_user,
        &mint_auth,
        sticker_data,
    )
    .await
    .unwrap();

    let new_price = 777;

    change_sticker_price(&mut program_context, &sticker_key, &artist_user, new_price)
        .await
        .unwrap();

    let sticker_account_data = get_account(&mut program_context, &sticker_key).await;
    let sticker_acc =
        state::Sticker::try_from_slice(&sticker_account_data.data.as_slice()).unwrap();

    assert_eq!(sticker_acc.price, new_price);
}
