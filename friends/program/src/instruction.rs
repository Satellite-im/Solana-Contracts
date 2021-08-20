//! Instruction types

use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    instruction::{AccountMeta, Instruction},
    program_error::ProgramError,
    pubkey::Pubkey,
    system_program, sysvar,
};

/// Address type
#[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug, Clone)]
pub enum AddressType {
    // /// FriendInfo
    // FriendInfo,
    // /// Outgoing request with index
    // RequestOutgoing(u64),
    // /// Incoming request with index
    // RequestIncoming(u64),
    /// Friend
    Friend(Pubkey),
}

/// Instruction definition
#[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug, Clone)]
pub enum FriendsInstruction {
    /// MakeRequest
    ///
    ///   0. `[w]` Friend account
    ///   1. `[w]` "from" account
    ///   2. `[w]` "to" account
    MakeRequest([u8; 32], [u8; 32], [u8; 32], [u8; 32]),

    /// AcceptRequest
    ///
    ///   0. `[w]` Friend account
    ///   1. `[w]` "from" account
    ///   2. `[w]` "to" account
    AcceptRequest([u8; 32], [u8; 32], [u8; 32], [u8; 32]),

    /// DenyRequest
    ///
    ///   0. `[w]` Friend account
    ///   1. `[w]` "from" account
    ///   2. `[w]` "to" account
    DenyRequest([u8; 32], [u8; 32], [u8; 32], [u8; 32]),

    /// RemoveRequest
    ///
    ///   0. `[w]` Friend account
    ///   1. `[w]` "from" account
    ///   2. `[w]` "to" account
    RemoveRequest([u8; 32], [u8; 32], [u8; 32], [u8; 32]),

    /// RemoveFriend
    ///
    ///   0. `[w]` Friend account
    ///   1. `[w]` "from" account
    ///   2. `[w]` "to" account
    RemoveFriend([u8; 32], [u8; 32], [u8; 32], [u8; 32]),

    /// Create derived account
    CreateAccount(AddressType),
}

/// Create `CreateAccount` instruction
pub fn create_account(
    program_id: &Pubkey,
    payer: &Pubkey,
    user_address: &Pubkey,
    base_address: &Pubkey,
    account_to_create: &Pubkey,
    address_type: AddressType,
) -> Result<Instruction, ProgramError> {
    let init_data = FriendsInstruction::CreateAccount(address_type);
    let data = init_data
        .try_to_vec()
        .or(Err(ProgramError::InvalidArgument))?;
    let accounts = vec![
        AccountMeta::new(*payer, true),
        AccountMeta::new_readonly(*user_address, false),
        AccountMeta::new_readonly(*base_address, false),
        AccountMeta::new(*account_to_create, false),
        AccountMeta::new_readonly(sysvar::rent::id(), false),
        AccountMeta::new_readonly(system_program::id(), false),
    ];
    Ok(Instruction {
        program_id: *program_id,
        accounts,
        data,
    })
}

/// Create `MakeRequest` instruction
pub fn make_request(
    program_id: &Pubkey,
    friend_account: &Pubkey,
    from_account: &Pubkey,
    to_account: &Pubkey,
    t_f1: [u8; 32],
    t_f2: [u8; 32],
    t_t1: [u8; 32],
    t_t2: [u8; 32],
) -> Result<Instruction, ProgramError> {
    let init_data = FriendsInstruction::MakeRequest(t_f1, t_f2, t_t1, t_t2);
    let data = init_data
        .try_to_vec()
        .or(Err(ProgramError::InvalidArgument))?;
    let accounts = vec![
        AccountMeta::new(*friend_account, false),
        AccountMeta::new_readonly(*from_account, true),
        AccountMeta::new_readonly(*to_account, false),
        AccountMeta::new_readonly(sysvar::rent::id(), false),
    ];
    Ok(Instruction {
        program_id: *program_id,
        accounts,
        data,
    })
}

/// Create `AcceptRequest` instruction
pub fn accept_request(
    program_id: &Pubkey,
    friend_account: &Pubkey,
    from_account: &Pubkey,
    to_account: &Pubkey,
    t_f1: [u8; 32],
    t_f2: [u8; 32],
    t_t1: [u8; 32],
    t_t2: [u8; 32],
) -> Result<Instruction, ProgramError> {
    let init_data = FriendsInstruction::AcceptRequest(t_f1, t_f2, t_t1, t_t2);
    let data = init_data
        .try_to_vec()
        .or(Err(ProgramError::InvalidArgument))?;
    let accounts = vec![
        AccountMeta::new(*friend_account, false),
        AccountMeta::new_readonly(*from_account, false),
        AccountMeta::new_readonly(*to_account, true),
        AccountMeta::new_readonly(sysvar::rent::id(), false),
    ];
    Ok(Instruction {
        program_id: *program_id,
        accounts,
        data,
    })
}

/// Create `DenyRequest` instruction
pub fn deny_request(
    program_id: &Pubkey,
    friend_account: &Pubkey,
    from_account: &Pubkey,
    to_account: &Pubkey,
    t_f1: [u8; 32],
    t_f2: [u8; 32],
    t_t1: [u8; 32],
    t_t2: [u8; 32],
) -> Result<Instruction, ProgramError> {
    let init_data = FriendsInstruction::DenyRequest(t_f1, t_f2, t_t1, t_t2);
    let data = init_data
        .try_to_vec()
        .or(Err(ProgramError::InvalidArgument))?;
    let accounts = vec![
        AccountMeta::new(*friend_account, false),
        AccountMeta::new_readonly(*from_account, false),
        AccountMeta::new_readonly(*to_account, true),
        AccountMeta::new_readonly(sysvar::rent::id(), false),
    ];
    Ok(Instruction {
        program_id: *program_id,
        accounts,
        data,
    })
}

/// Create `RemoveRequest` instruction
pub fn remove_request(
    program_id: &Pubkey,
    friend_account: &Pubkey,
    from_account: &Pubkey,
    to_account: &Pubkey,
    t_f1: [u8; 32],
    t_f2: [u8; 32],
    t_t1: [u8; 32],
    t_t2: [u8; 32],
) -> Result<Instruction, ProgramError> {
    let init_data = FriendsInstruction::RemoveRequest(t_f1, t_f2, t_t1, t_t2);
    let data = init_data
        .try_to_vec()
        .or(Err(ProgramError::InvalidArgument))?;
    let accounts = vec![
        AccountMeta::new(*friend_account, false),
        AccountMeta::new_readonly(*from_account, true),
        AccountMeta::new_readonly(*to_account, false),
        AccountMeta::new_readonly(sysvar::rent::id(), false),
    ];
    Ok(Instruction {
        program_id: *program_id,
        accounts,
        data,
    })
}

/// Create `RemoveFriend` instruction
pub fn remove_friend(
    program_id: &Pubkey,
    friend_account: &Pubkey,
    from_account: &Pubkey,
    to_account: &Pubkey,
    t_f1: [u8; 32],
    t_f2: [u8; 32],
    t_t1: [u8; 32],
    t_t2: [u8; 32],
) -> Result<Instruction, ProgramError> {
    let init_data = FriendsInstruction::RemoveFriend(t_f1, t_f2, t_t1, t_t2);
    let data = init_data
        .try_to_vec()
        .or(Err(ProgramError::InvalidArgument))?;
    let accounts = vec![
        AccountMeta::new(*friend_account, false),
        AccountMeta::new_readonly(*from_account, true),
        AccountMeta::new_readonly(*to_account, false),
        AccountMeta::new_readonly(sysvar::rent::id(), false),
    ];
    Ok(Instruction {
        program_id: *program_id,
        accounts,
        data,
    })
}
