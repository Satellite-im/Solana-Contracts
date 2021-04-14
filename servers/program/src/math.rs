use solana_program::program_error::ProgramError;

use crate::error::Error;

/// checked add into error
pub trait ErrorAdd {
    fn error_increment(self) -> Result<u64, ProgramError>;
}

impl ErrorAdd for u64 {
    fn error_increment(self) -> Result<u64, ProgramError> {
        self.checked_add(1)
            .ok_or::<ProgramError>(Error::Overflow.into())
    }
}
