//! Error types

use num_derive::FromPrimitive;
use num_traits::FromPrimitive;
use solana_program::{
    decode_error::DecodeError,
    msg,
    program_error::{PrintProgramError, ProgramError},
};
use thiserror::Error;

/// Errors that may be returned by the Friends program.
#[derive(Clone, Debug, Eq, Error, FromPrimitive, PartialEq)]
pub enum FriendsProgramError {
    /// Calculation error
    #[error("Calculation error")]
    CalculationError,
    /// Addresses in request don't match addresses in FriendInfo accounts
    #[error("Addresses in request don't match addresses in FriendInfo accounts")]
    WrongRequestData,
    /// Accounts are already friends
    #[error("Accounts are already friends")]
    AlreadyFriends,
}
impl From<FriendsProgramError> for ProgramError {
    fn from(e: FriendsProgramError) -> Self {
        ProgramError::Custom(e as u32)
    }
}
impl<T> DecodeError<T> for FriendsProgramError {
    fn type_of() -> &'static str {
        "FriendsProgramError"
    }
}

impl PrintProgramError for FriendsProgramError {
    fn print<E>(&self)
    where
        E: 'static + std::error::Error + DecodeError<E> + PrintProgramError + FromPrimitive,
    {
        match self {
            FriendsProgramError::CalculationError => msg!("Calculation error"),
            FriendsProgramError::WrongRequestData => {
                msg!("Addresses in request don't match addresses in FriendInfo accounts")
            }
            FriendsProgramError::AlreadyFriends => msg!("Accounts are already friends"),
        }
    }
}
