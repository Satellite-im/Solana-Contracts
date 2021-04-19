//! Error types

use num_derive::FromPrimitive;
use num_traits::FromPrimitive;
use solana_program::{
    decode_error::DecodeError,
    msg,
    program_error::{PrintProgramError, ProgramError},
};
use thiserror::Error;

/// Errors that may be returned by the Template program.
#[derive(Clone, Debug, Eq, Error, FromPrimitive, PartialEq)]
pub enum StickerProgramError {
    /// Wrong mint for user token account
    #[error("Wrong mint for user token account")]
    WrongTokenMint,
    /// Wrong sticker factory owner
    #[error("Wrong sticker factory owner")]
    WrongStickerFactoryOwner,
    /// Calculation error
    #[error("Calculation error")]
    CalculationError,
    /// Wrong sticker creator
    #[error("Wrong sticker creator")]
    WrongStickerCreator,
    /// Wrong token mint authority
    #[error("Wrong token mint authority")]
    WrongTokenMintAuthority,
    /// No more tokens to mint
    #[error("No more tokens to mint")]
    NoTokensToMint,
}
impl From<StickerProgramError> for ProgramError {
    fn from(e: StickerProgramError) -> Self {
        ProgramError::Custom(e as u32)
    }
}
impl<T> DecodeError<T> for StickerProgramError {
    fn type_of() -> &'static str {
        "StickerProgramError"
    }
}

impl PrintProgramError for StickerProgramError {
    fn print<E>(&self)
    where
        E: 'static + std::error::Error + DecodeError<E> + PrintProgramError + FromPrimitive,
    {
        match self {
            StickerProgramError::WrongTokenMint => msg!("Wrong mint for user token account"),
            StickerProgramError::WrongStickerFactoryOwner => msg!("Wrong sticker factory owner"),
            StickerProgramError::CalculationError => msg!("Calculation error"),
            StickerProgramError::WrongStickerCreator => msg!("Wrong sticker creator"),
            StickerProgramError::WrongTokenMintAuthority => msg!("Wrong token mint authority"),
            StickerProgramError::NoTokensToMint => msg!("No more tokens to mint"),
        }
    }
}
