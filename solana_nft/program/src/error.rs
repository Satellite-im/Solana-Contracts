//! Error types

use num_derive::FromPrimitive;
use num_traits::FromPrimitive;
use solana_program::{
    decode_error::DecodeError,
    msg,
    program_error::{PrintProgramError, ProgramError},
};
use thiserror::Error;

/// Errors that may be returned by the NFT program.
#[derive(Clone, Debug, Eq, Error, FromPrimitive, PartialEq)]
pub enum NftError {
    #[error("Signer neither owner or approval of token")]
    SignerNotOwnerOrApproval,
    #[error("Signer has to many sols to get burned account sols")]
    Overflow,
}
impl From<NftError> for ProgramError {
    fn from(e: NftError) -> Self {
        ProgramError::Custom(e as u32)
    }
}
impl<T> DecodeError<T> for NftError {
    fn type_of() -> &'static str {
        "NftError"
    }
}

impl PrintProgramError for NftError {
    fn print<E>(&self)
    where
        E: 'static + std::error::Error + DecodeError<E> + PrintProgramError + FromPrimitive,
    {
        msg!(&self.to_string());
    }
}
