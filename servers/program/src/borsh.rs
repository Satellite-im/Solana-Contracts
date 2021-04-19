use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{account_info::AccountInfo, program_error::ProgramError};

/// Forces rust auto cast, it does not work with write in default serialize
/// In Rust 1.51 can make it const N generic
pub trait BorshSerializeConst {
    fn serialize_const(&self, writer: &mut [u8]) -> std::io::Result<()>;
}

impl<T> BorshSerializeConst for T
where
    T: BorshSerialize,
{
    #[allow(clippy::redundant_slicing)] // for some reason rust clippy this but fails to compile clippied
    fn serialize_const(&self, writer: &mut [u8]) -> std::io::Result<()> {
        let mut writer = &mut writer[..];
        self.serialize(&mut writer)
    }
}

pub trait BorshDeserialiseConst<T: BorshDeserialize> {
    fn deserialize_const(reader: &[u8]) -> std::io::Result<T>;
}

impl<T> BorshDeserialiseConst<T> for T
where
    T: BorshDeserialize,
{
    #[allow(clippy::redundant_slicing)] // for some reason rust clippy this but fails to compile clippied
    fn deserialize_const(writer: &[u8]) -> std::io::Result<T> {
        let mut writer = &writer[..];
        Self::deserialize(&mut writer)
    }
}

pub trait AccountWithBorsh {
    fn read_data_with_borsh<T: BorshDeserialize>(&self) -> Result<T, ProgramError>;
    fn read_data_with_borsh_mut<T: BorshDeserialize>(
        &self,
    ) -> Result<(std::cell::RefMut<&mut [u8]>, T), ProgramError>;
}

impl<'a> AccountWithBorsh for AccountInfo<'a> {
    fn read_data_with_borsh<T: BorshDeserialize>(&self) -> Result<T, ProgramError> {
        let data = self.try_borrow_data()?;
        Ok(T::deserialize_const(&data)?)
    }

    fn read_data_with_borsh_mut<T: BorshDeserialize>(
        &self,
    ) -> Result<(std::cell::RefMut<'a, &mut [u8]>, T), ProgramError> {
        let server_data = self.try_borrow_mut_data()?;
        let server_state = T::deserialize_const(&server_data)?;
        Ok((server_data, server_state))
    }
}
