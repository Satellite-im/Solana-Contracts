use solana_program::program_error::ProgramError;

use crate::error::Error;

/// checked add into error
pub trait ErrorAdd {
    fn error_increment(self) -> Result<u64, ProgramError>;
    fn error_decrement(self) -> Result<u64, ProgramError>;
}

impl ErrorAdd for u64 {
    fn error_increment(self) -> Result<u64, ProgramError> {
        self.checked_add(1).ok_or_else(|| Error::Overflow.into())
    }

    fn error_decrement(self) -> Result<u64, ProgramError> {
        self.checked_sub(1).ok_or_else(|| Error::Underflow.into())
    }
}
