//! Helpers for tests

use solana_program::pubkey::Pubkey;
use solana_sdk::{
    signature::{Keypair, Signer},
    transaction::Transaction,
};

use crate::instruction::{
    self, AddChannelInput, CreateGroupInput, SetDwellerStatusInput, SetHashInput, SetNameInput,
};

/// assumes not program dweller
pub fn add_invite_transaction(
    payer: &Keypair,
    server: &Pubkey,
    dweller_administrator: &Keypair,
    server_administrator: &Pubkey,
    dweller: &Pubkey,
    member_status: &Pubkey,
    recent_blockhash: solana_program::hash::Hash,
) -> Transaction {
    let mut transaction = Transaction::new_with_payer(
        &[instruction::invite_to_server(
            &server,
            &dweller_administrator.pubkey(),
            server_administrator,
            dweller,
            member_status,
        )
        .unwrap()],
        Some(&payer.pubkey()),
    );
    transaction.sign(&[payer, dweller_administrator], recent_blockhash);
    transaction
}

/// assumes not program dweller
pub fn join_server_transaction(
    payer: &Keypair,
    server: &Pubkey,
    server_member: &Pubkey,
    server_member_status: &Pubkey,
    dweller: &Keypair,
    dweller_server: &Pubkey,
    recent_blockhash: solana_program::hash::Hash,
) -> Transaction {
    let mut transaction = Transaction::new_with_payer(
        &[instruction::join_server(
            server,
            server_member,
            server_member_status,
            &dweller.pubkey(),
            dweller_server,
        )
        .unwrap()],
        Some(&payer.pubkey()),
    );
    transaction.sign(&[payer, dweller], recent_blockhash);
    transaction
}

/// assumes not program dweller
pub fn create_group_transaction(
    payer: &Keypair,
    dweller_administrator: &Keypair,
    server_administrator: &Pubkey,
    server: &Pubkey,
    server_group: &Pubkey,
    input: &CreateGroupInput,
    recent_blockhash: solana_program::hash::Hash,
) -> Transaction {
    let mut transaction = Transaction::new_with_payer(
        &[instruction::create_group(
            &dweller_administrator.pubkey(),
            server_administrator,
            server,
            &server_group,
            input,
        )
        .unwrap()],
        Some(&payer.pubkey()),
    );
    transaction.sign(&[payer, dweller_administrator], recent_blockhash);
    transaction
}

/// assumes not program dweller
pub fn add_channel_transaction(
    payer: &Keypair,
    dweller_administrator: &Keypair,
    server_administrator: &Pubkey,
    server: &Pubkey,
    server_channel: &Pubkey,
    input: &AddChannelInput,
    recent_blockhash: solana_program::hash::Hash,
) -> Transaction {
    let mut transaction = Transaction::new_with_payer(
        &[instruction::add_channel(
            &dweller_administrator.pubkey(),
            server_administrator,
            server,
            &server_channel,
            input,
        )
        .unwrap()],
        Some(&payer.pubkey()),
    );
    transaction.sign(&[payer, dweller_administrator], recent_blockhash);
    transaction
}

/// assumes not program dweller
#[allow(clippy::too_many_arguments)]
pub fn add_channel_to_group_transaction(
    payer: &Keypair,
    server: &Pubkey,
    dweller_administrator: &Keypair,
    server_administrator: &Pubkey,
    server_channel: &Pubkey,
    server_group: &Pubkey,
    group_channel: &Pubkey,
    recent_blockhash: solana_program::hash::Hash,
) -> Transaction {
    let mut transaction = Transaction::new_with_payer(
        &[instruction::add_channel_to_group(
            server,
            &dweller_administrator.pubkey(),
            server_administrator,
            server_channel,
            server_group,
            group_channel,
        )
        .unwrap()],
        Some(&payer.pubkey()),
    );
    transaction.sign(&[payer, dweller_administrator], recent_blockhash);
    transaction
}

/// assumes not program dweller
#[allow(clippy::too_many_arguments)]
pub fn remove_channel_from_group_transaction(
    payer: &Keypair,
    server: &Pubkey,
    dweller_administrator: &Keypair,
    server_administrator: &Pubkey,
    server_group: &Pubkey,
    group_channel: &Pubkey,
    group_channel_last: &Pubkey,
    recent_blockhash: solana_program::hash::Hash,
) -> Transaction {
    let mut transaction = Transaction::new_with_payer(
        &[instruction::remove_channel_from_group(
            server,
            &dweller_administrator.pubkey(),
            server_administrator,
            server_group,
            group_channel,
            group_channel_last,
        )
        .unwrap()],
        Some(&payer.pubkey()),
    );
    transaction.sign(&[payer, dweller_administrator], recent_blockhash);
    transaction
}

/// assumes not program dweller
#[allow(clippy::too_many_arguments)]
pub fn delete_group_transaction(
    payer: &Keypair,
    dweller_administrator: &Keypair,
    server_administrator: &Pubkey,
    server: &Pubkey,
    server_group: &Pubkey,
    server_group_last: &Pubkey,
    group_channels: &Pubkey,
    recent_blockhash: solana_program::hash::Hash,
) -> Transaction {
    let mut transaction = Transaction::new_with_payer(
        &[instruction::delete_group(
            &dweller_administrator.pubkey(),
            server_administrator,
            server,
            server_group,
            server_group_last,
            &[group_channels],
        )
        .unwrap()],
        Some(&payer.pubkey()),
    );
    transaction.sign(&[payer, dweller_administrator], recent_blockhash);
    transaction
}

/// assumes not program dweller
#[allow(clippy::too_many_arguments)]
pub fn delete_channel_transaction(
    payer: &Keypair,
    dweller_administrator: &Keypair,
    server_administrator: &Pubkey,
    server: &Pubkey,
    server_channel: &Pubkey,
    server_channel_last: &Pubkey,
    recent_blockhash: solana_program::hash::Hash,
) -> Transaction {
    let mut transaction = Transaction::new_with_payer(
        &[instruction::delete_channel(
            &dweller_administrator.pubkey(),
            server_administrator,
            server,
            server_channel,
            server_channel_last,
        )
        .unwrap()],
        Some(&payer.pubkey()),
    );
    transaction.sign(&[payer, dweller_administrator], recent_blockhash);
    transaction
}

/// assumes not program dweller
#[allow(clippy::too_many_arguments)]
pub fn leave_server_transaction(
    payer: &Keypair,
    server: &Pubkey,
    server_member: &Pubkey,
    server_member_last: &Pubkey,
    dweller: &Keypair,
    dweller_server: &Pubkey,
    dweller_server_last: &Pubkey,
    recent_blockhash: solana_program::hash::Hash,
) -> Transaction {
    let mut transaction = Transaction::new_with_payer(
        &[instruction::leave_server(
            server,
            server_member,
            server_member_last,
            &dweller.pubkey(),
            dweller_server,
            dweller_server_last,
        )
        .unwrap()],
        Some(&payer.pubkey()),
    );
    transaction.sign(&[payer, dweller], recent_blockhash);
    transaction
}

/// assumes not program dweller
pub fn revoke_invite_server_transaction(
    payer: &Keypair,
    server: &Pubkey,
    dweller_administrator: &Keypair,
    server_administrator: &Pubkey,
    server_member_status: &Pubkey,
    server_member_status_last: &Pubkey,
    recent_blockhash: solana_program::hash::Hash,
) -> Transaction {
    let mut transaction = Transaction::new_with_payer(
        &[instruction::revoke_invite_server(
            server,
            &dweller_administrator.pubkey(),
            server_administrator,
            server_member_status,
            server_member_status_last,
        )
        .unwrap()],
        Some(&payer.pubkey()),
    );
    transaction.sign(&[payer, dweller_administrator], recent_blockhash);
    transaction
}

/// assumes not program dweller
pub fn remove_admin_transaction(
    payer: &Keypair,
    owner: &Keypair,
    server: &Pubkey,
    server_administrator: &Pubkey,
    server_administrator_last: &Pubkey,
    recent_blockhash: solana_program::hash::Hash,
) -> Transaction {
    let mut transaction = Transaction::new_with_payer(
        &[instruction::remove_admin(
            &owner.pubkey(),
            server,
            server_administrator,
            server_administrator_last,
        )
        .unwrap()],
        Some(&payer.pubkey()),
    );
    transaction.sign(&[payer, owner], recent_blockhash);
    transaction
}

/// assumes not program dweller
pub fn set_dweller_name_transaction(
    payer: &Keypair,
    dweller: &Keypair,
    input: &SetNameInput,
    recent_blockhash: solana_program::hash::Hash,
) -> Transaction {
    let mut transaction = Transaction::new_with_payer(
        &[instruction::set_dweller_name(&dweller.pubkey(), input).unwrap()],
        Some(&payer.pubkey()),
    );
    transaction.sign(&[payer, dweller], recent_blockhash);
    transaction
}

/// assumes not program dweller
pub fn set_dweller_photo_transaction(
    payer: &Keypair,
    dweller: &Keypair,
    input: &SetHashInput,
    recent_blockhash: solana_program::hash::Hash,
) -> Transaction {
    let mut transaction = Transaction::new_with_payer(
        &[instruction::set_dweller_photo(&dweller.pubkey(), input).unwrap()],
        Some(&payer.pubkey()),
    );
    transaction.sign(&[payer, dweller], recent_blockhash);
    transaction
}

/// assumes not program dweller
pub fn set_dweller_status_transaction(
    payer: &Keypair,
    dweller: &Keypair,
    input: &SetDwellerStatusInput,
    recent_blockhash: solana_program::hash::Hash,
) -> Transaction {
    let mut transaction = Transaction::new_with_payer(
        &[instruction::set_dweller_status(&dweller.pubkey(), input).unwrap()],
        Some(&payer.pubkey()),
    );
    transaction.sign(&[payer, dweller], recent_blockhash);
    transaction
}

/// assumes not program dweller
pub fn set_server_name_transaction(
    payer: &Keypair,
    server: &Pubkey,
    dweller_administrator: &Keypair,
    server_administrator: &Pubkey,
    input: &SetNameInput,
    recent_blockhash: solana_program::hash::Hash,
) -> Transaction {
    let mut transaction = Transaction::new_with_payer(
        &[instruction::set_server_name(
            server,
            &dweller_administrator.pubkey(),
            server_administrator,
            input,
        )
        .unwrap()],
        Some(&payer.pubkey()),
    );
    transaction.sign(&[payer, dweller_administrator], recent_blockhash);
    transaction
}

/// assumes not program dweller
pub fn set_server_db_transaction(
    payer: &Keypair,
    server: &Pubkey,
    dweller_administrator: &Keypair,
    server_administrator: &Pubkey,
    input: &SetHashInput,
    recent_blockhash: solana_program::hash::Hash,
) -> Transaction {
    let mut transaction = Transaction::new_with_payer(
        &[instruction::set_server_db(
            server,
            &dweller_administrator.pubkey(),
            server_administrator,
            input,
        )
        .unwrap()],
        Some(&payer.pubkey()),
    );
    transaction.sign(&[payer, dweller_administrator], recent_blockhash);
    transaction
}
