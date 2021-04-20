//! Error types

use num_derive::FromPrimitive;
use num_traits::FromPrimitive;
use solana_program::{
    decode_error::DecodeError, msg, program_error::PrintProgramError, program_error::ProgramError,
};

/// Errors that may be returned by the program.
#[derive(Clone, Debug, Eq, thiserror::Error, FromPrimitive, PartialEq)]
pub enum Error {
    ///Cannot execute instruction
    #[error("Cannot execute instruction")]
    Failed,
    ///Only owner can add remove admins
    #[error("Only owner can add remove admins")]
    OnlyOwnerCanAddRemoveAdmins,

    ///Overflow
    #[error("Overflow")]
    Overflow,

    ///Underflow
    #[error("Underflow")]
    Underflow,

    ///Invalid derived address
    #[error("Invalid derived address")]
    InvalidDerivedAddress,

    ///Invalid derived dweller server address
    #[error("Invalid derived dweller server address")]
    InvalidDerivedDwellerServerAddress,

    ///Invalid derived server member status address
    #[error("Invalid derived server member status address")]
    InvalidDerivedServerMemberStatusAddress,

    ///Invalid derived server member address
    #[error("Invalid derived server member address")]
    InvalidDerivedServerMemberAddress,

    ///Invalid derived server group address
    #[error("Invalid derived server group address")]
    InvalidDerivedServerGroupAddress,

    ///Invalid derived server channel address
    #[error("Invalid derived server channel address")]
    InvalidDerivedServerChannelAddress,

    ///Invalid derived group channel address
    #[error("Invalid derived group channel address")]
    InvalidDerivedGroupChannelAddress,

    ///Invalid derived server administrator address
    #[error("Invalid derived server administrator address")]
    InvalidDerivedServerAdministratorAddress,

    ///Provided dweller is not the owner of the server
    #[error("Provided dweller is not the owner of the server")]
    ProvidedDwellerIsNotTheOwnerOfTheServer,

    ///Invalid derived address wrong server
    #[error("Invalid derived address wrong server")]
    InvalidDerivedAddressWrongServer,

    ///Invalid derived server member laast address
    #[error("Invalid derived server member laast address")]
    InvalidDerivedServerMemberLaastAddress,
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
        msg!(&self.to_string())
    }
}
