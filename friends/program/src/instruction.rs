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
    /// FriendInfo
    FriendInfo,
    /// Outgoing request with index
    RequestOutgoing(u64),
    /// Incoming request with index
    RequestIncoming(u64),
    /// Friend
    Friend(Pubkey),
}

/// Instruction definition
#[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug, Clone)]
pub enum FriendsInstruction {
    /// InitFriendInfo
    ///
    ///   0. `[w]` Uninitialized FriendInfo account
    ///   1. `[rs]` User's account
    ///   2. `[r]` Rent sysvar
    InitFriendInfo,

    /// MakeRequest
    ///
    ///   0. `[w]` Friendship request for "from" account
    ///   1. `[w]` Friendship request for "to" account
    ///   2. `[w]` Friend info of account which request friendship
    ///   3. `[w]` Friend info of account with which friendship requested
    ///   4. `[rs]` friend_info_from's "user" key. To verify friendship request
    MakeRequest,

    /// AcceptRequest
    ///
    ///   0. `[w]` Friendship request for "from" account
    ///   1. `[w]` Friendship request for "to" account
    ///   2. `[w]` Last friendship request for "from" account
    ///   3. `[w]` Last friendship request for "to" account
    ///   4. `[w]` Friend info of account which request friendship
    ///   5. `[w]` Friend info of account with which friendship requested
    ///   6. `[w]` Uninitialized Friend account for "to" account
    ///   7. `[w]` Uninitialized Friend account for "from" account
    ///   8. `[rs]` friend_info_to's "user" key. To verify acception side
    AcceptRequest([u8; 32], [u8; 32]),

    /// DenyRequest
    ///
    ///   0. `[w]` Friendship request for "from" account
    ///   1. `[w]` Friendship request for "to" account
    ///   2. `[w]` Last friendship request for "from" account
    ///   3. `[w]` Last friendship request for "to" account
    ///   4. `[w]` Friend info of account which request friendship
    ///   5. `[w]` Friend info of account with which friendship requested
    ///   6. `[rs]` friend_info_to's "user" key. To verify acception side
    DenyRequest,

    /// RemoveRequest
    ///
    ///   0. `[w]` Friendship request for "from" account
    ///   1. `[w]` Friendship request for "to" account
    ///   2. `[w]` Last friendship request for "from" account
    ///   3. `[w]` Last friendship request for "to" account
    ///   4. `[w]` Friend info of account which request friendship
    ///   5. `[w]` Friend info of account with which friendship requested
    ///   6. `[rs]` friend_info_from's "user" key. To verify requesting side
    RemoveRequest,

    /// RemoveFriend
    ///
    ///   0. `[w]` Friend info of account which wants to break friendship
    ///   1. `[w]` Friend info of account with which wants to break friendship
    ///   2. `[w]` Friend account which wants to break friendship
    ///   3. `[w]` Friend account with which wants to break friendship
    ///   4. `[w]` Last friend account of account which initiate friendship break
    ///   5. `[w]` Last friend account of account with which wants break friendship
    ///   6. `[rs]` User account which initiate break friendship
    RemoveFriend,

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

/// Create `InitFriendInfo` instruction
pub fn init(
    program_id: &Pubkey,
    friend_info: &Pubkey,
    user: &Pubkey,
) -> Result<Instruction, ProgramError> {
    let init_data = FriendsInstruction::InitFriendInfo;
    let data = init_data
        .try_to_vec()
        .or(Err(ProgramError::InvalidArgument))?;
    let accounts = vec![
        AccountMeta::new(*friend_info, false),
        AccountMeta::new_readonly(*user, true),
        AccountMeta::new_readonly(sysvar::rent::id(), false),
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
    request_from_to: &Pubkey,
    request_to_from: &Pubkey,
    friend_info_from: &Pubkey,
    friend_info_to: &Pubkey,
    user_from: &Pubkey,
) -> Result<Instruction, ProgramError> {
    let init_data = FriendsInstruction::MakeRequest;
    let data = init_data
        .try_to_vec()
        .or(Err(ProgramError::InvalidArgument))?;
    let accounts = vec![
        AccountMeta::new(*request_from_to, false),
        AccountMeta::new(*request_to_from, false),
        AccountMeta::new(*friend_info_from, false),
        AccountMeta::new(*friend_info_to, false),
        AccountMeta::new_readonly(*user_from, true),
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
    request_from_to: &Pubkey,
    request_to_from: &Pubkey,
    last_request_from_to: &Pubkey,
    last_request_to_from: &Pubkey,
    friend_info_from: &Pubkey,
    friend_info_to: &Pubkey,
    friend_to: &Pubkey,
    friend_from: &Pubkey,
    user_to: &Pubkey,
    thread_id1: [u8; 32],
    thread_id2: [u8; 32],
) -> Result<Instruction, ProgramError> {
    let init_data = FriendsInstruction::AcceptRequest(thread_id1, thread_id2);
    let data = init_data
        .try_to_vec()
        .or(Err(ProgramError::InvalidArgument))?;
    let accounts = vec![
        AccountMeta::new(*request_from_to, false),
        AccountMeta::new(*request_to_from, false),
        AccountMeta::new(*last_request_from_to, false),
        AccountMeta::new(*last_request_to_from, false),
        AccountMeta::new(*friend_info_from, false),
        AccountMeta::new(*friend_info_to, false),
        AccountMeta::new(*friend_to, false),
        AccountMeta::new(*friend_from, false),
        AccountMeta::new_readonly(*user_to, true),
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
    request_from_to: &Pubkey,
    request_to_from: &Pubkey,
    last_request_from: &Pubkey, // last outgoing request for requested account
    last_request_to: &Pubkey,   // last incoming request for denied account
    friend_info_from: &Pubkey,
    friend_info_to: &Pubkey,
    user_to: &Pubkey,
) -> Result<Instruction, ProgramError> {
    let init_data = FriendsInstruction::DenyRequest;
    let data = init_data
        .try_to_vec()
        .or(Err(ProgramError::InvalidArgument))?;
    let accounts = vec![
        AccountMeta::new(*request_from_to, false),
        AccountMeta::new(*request_to_from, false),
        AccountMeta::new(*last_request_from, false),
        AccountMeta::new(*last_request_to, false),
        AccountMeta::new(*friend_info_from, false),
        AccountMeta::new(*friend_info_to, false),
        AccountMeta::new_readonly(*user_to, true),
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
    request_from_to: &Pubkey,
    request_to_from: &Pubkey,
    last_request_from: &Pubkey, // last outgoing request for requested account
    last_request_to: &Pubkey,   // last incoming request for denied account
    friend_info_from: &Pubkey,
    friend_info_to: &Pubkey,
    user_from: &Pubkey,
) -> Result<Instruction, ProgramError> {
    let init_data = FriendsInstruction::RemoveRequest;
    let data = init_data
        .try_to_vec()
        .or(Err(ProgramError::InvalidArgument))?;
    let accounts = vec![
        AccountMeta::new(*request_from_to, false),
        AccountMeta::new(*request_to_from, false),
        AccountMeta::new(*last_request_from, false),
        AccountMeta::new(*last_request_to, false),
        AccountMeta::new(*friend_info_from, false),
        AccountMeta::new(*friend_info_to, false),
        AccountMeta::new_readonly(*user_from, true),
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
    friend_info_first: &Pubkey,
    friend_info_second: &Pubkey,
    friend_first: &Pubkey,
    friend_second: &Pubkey,
    user: &Pubkey,
) -> Result<Instruction, ProgramError> {
    let init_data = FriendsInstruction::RemoveFriend;
    let data = init_data
        .try_to_vec()
        .or(Err(ProgramError::InvalidArgument))?;
    let accounts = vec![
        AccountMeta::new(*friend_info_first, false),
        AccountMeta::new(*friend_info_second, false),
        AccountMeta::new(*friend_first, false),
        AccountMeta::new(*friend_second, false),
        AccountMeta::new_readonly(*user, true),
    ];
    Ok(Instruction {
        program_id: *program_id,
        accounts,
        data,
    })
}
