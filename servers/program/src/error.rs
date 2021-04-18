//! Error types

use num_derive::FromPrimitive;
use num_traits::FromPrimitive;
use solana_program::{
    decode_error::DecodeError, msg, program_error::PrintProgramError, program_error::ProgramError,
};

use strum_macros::AsRefStr;

/// Errors that may be returned by the program.
#[derive(Clone, Debug, Eq, FromPrimitive, PartialEq, AsRefStr)]
pub enum Error {
    Failed,
    OnlyOwnerCanAddRemoveAdmins,
    Overflow,
    InvalidDerivedAddress,
}

impl From<Error> for ProgramError {
    fn from(e: Error) -> Self {
        ProgramError::Custom(e as u32)
    }
}

impl<T> DecodeError<T> for Error {
    fn type_of() -> &'static str {
        "Satellite Server"
    }
}

impl PrintProgramError for Error {
    fn print<E>(&self)
    where
        E: 'static + std::error::Error + DecodeError<E> + PrintProgramError + FromPrimitive,
    {
        msg!(&self.as_ref())
    }
}
