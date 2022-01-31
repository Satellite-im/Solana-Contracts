//! Instruction types

use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    instruction::{AccountMeta, Instruction},
    program_error::ProgramError,
    pubkey::Pubkey,
    system_program, sysvar,
};

/// Instruction definition
#[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug, Clone)]
pub enum FriendsInstruction {
    /// MakeRequest
    ///
    ///   0. `[w]` Friend account
    ///   1. `[w]` Friend account with switched "from" and "to"
    ///   2. `[w]` "from" account
    ///   3. `[w]` "to" account
    ///   4. `[ ]` rent sysvar
    MakeRequest([u8; 32], [u8; 32], [u8; 32], [u8; 32]),

    /// AcceptRequest
    ///
    ///   0. `[w]` Friend account
    ///   1. `[w]` "from" account
    ///   2. `[w]` "to" account
    ///   3. `[ ]` rent sysvar
    AcceptRequest([u8; 32], [u8; 32], [u8; 32], [u8; 32]),

    /// DenyRequest
    ///
    ///   0. `[w]` Friend account
    ///   1. `[w]` "from" account
    ///   2. `[w]` "to" account
    ///   3. `[ ]` rent sysvar
    DenyRequest,

    /// RemoveRequest
    ///
    ///   0. `[w]` Friend account
    ///   1. `[w]` "from" account
    ///   2. `[w]` "to" account
    ///   3. `[ ]` rent sysvar
    RemoveRequest,

    /// RemoveFriend
    ///
    ///   0. `[w]` Friend account
    ///   1. `[w]` "from" account
    ///   2. `[w]` "to" account
    ///   3. `[ ]` rent sysvar
    RemoveFriend,

    /// Create derived account
    CreateAccount(Pubkey),
}

/// Create `CreateAccount` instruction
pub fn create_account(
    program_id: &Pubkey,
    payer: &Pubkey,
    user_address: &Pubkey,
    base_address: &Pubkey,
    account_to_create: &Pubkey,
    friend_key: Pubkey,
) -> Result<Instruction, ProgramError> {
    let init_data = FriendsInstruction::CreateAccount(friend_key);
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
    mirrored_friend_account: &Pubkey,
    from_account: &Pubkey,
    to_account: &Pubkey,
    t_f1: [u8; 32],
    t_f2: [u8; 32],
    t_f3: [u8; 32],
    t_f4: [u8; 32],
) -> Result<Instruction, ProgramError> {
    let init_data = FriendsInstruction::MakeRequest(t_f1, t_f2, t_f3, t_f4);
    let data = init_data
        .try_to_vec()
        .or(Err(ProgramError::InvalidArgument))?;
    let accounts = vec![
        AccountMeta::new(*friend_account, false),
        AccountMeta::new(*mirrored_friend_account, false),
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
    t_t1: [u8; 32],
    t_t2: [u8; 32],
    t_t3: [u8; 32],
    t_t4: [u8; 32],
) -> Result<Instruction, ProgramError> {
    let init_data = FriendsInstruction::AcceptRequest(t_t1, t_t2, t_t3, t_t4);
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
) -> Result<Instruction, ProgramError> {
    let init_data = FriendsInstruction::DenyRequest;
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
) -> Result<Instruction, ProgramError> {
    let init_data = FriendsInstruction::RemoveRequest;
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
) -> Result<Instruction, ProgramError> {
    let init_data = FriendsInstruction::RemoveFriend;
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
