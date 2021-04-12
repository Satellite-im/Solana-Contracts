// //! Error types

// use num_derive::FromPrimitive;
// use num_traits::FromPrimitive;
// use solana_program::{
//     decode_error::DecodeError, msg, program_error::PrintProgramError, program_error::ProgramError,
// };
// use thiserror::Error;

// /// Errors that may be returned by the program.
// #[derive(Clone, Debug, Eq, Error, FromPrimitive, PartialEq)]
// pub enum PoolError {
//     #[error("Token must be under asset authority")]
//     TokenMustBeUnderAssetAuthority,
// }

// impl From<PoolError> for ProgramError {
//     fn from(e: PoolError) -> Self {
//         ProgramError::Custom(e as u32)
//     }
// }

// impl<T> DecodeError<T> for PoolError {
//     fn type_of() -> &'static str {
//         "SPL Token Mega Swap"
//     }
// }

// impl PrintProgramError for PoolError {
//     fn print<E>(&self)
//     where
//         E: 'static + std::error::Error + DecodeError<E> + PrintProgramError + FromPrimitive,
//     {
//         msg!(&self.to_string())
//     }
// }
