///! Registry types.
use borsh::{BorshDeserialize, BorshSchema, BorshSerialize};
use num_derive::{FromPrimitive, ToPrimitive};
use solana_program::{
    entrypoint::ProgramResult, program_error::ProgramError, program_pack::IsInitialized,
    pubkey::Pubkey,
};

#[repr(C)]
#[derive(
    BorshSerialize,
    BorshDeserialize,
    PartialEq,
    Debug,
    Clone,
    BorshSchema,
    ToPrimitive,
    FromPrimitive,
)]
pub enum StateVersion {
    Uninitialized,
    V1,
}

impl Default for StateVersion {
    fn default() -> Self {
        StateVersion::Uninitialized
    }
}

/// address of signer + separate program deployed
/// https://github.com/Satellite-im/Satellite-Contracts/blob/main/contracts/DwellerID.sol
#[repr(C)]
#[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug, Clone, BorshSchema)]
pub struct Dweller {
    pub version: StateVersion,

    /// used to derive DwellerServer
    pub servers: u64,

    /// This is the display name of a dweller
    pub name: [u8; 32],

    /// Optional Photo identification of the dweller
    /// Multihash referencing IPFS hash of dwellers photo
    pub photo_hash: [u8; 64],

    /// string
    pub status: [u8; 32],
}

impl Dweller {
    pub const LEN: u64 = 137;
}

/// Mapping of `Dweller` to `Server`.
/// Account address is be derived from `Dweller`
#[repr(C)]
#[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug, Clone, BorshSchema, Default)]
pub struct DwellerServer {
    pub version: StateVersion,
    /// dweller
    pub container: Pubkey,
    /// [Dweller::servers] index used to derive address
    pub index: u64,
    pub server: Pubkey,
}

impl DwellerServer {
    pub const LEN: u64 = 73;
    pub const SEED: &'static str = "DwellerServer";
}

/// Server members whom have joined
/// Has program derived address from Server
/// many to many map of `Server` to `DwellerID` (inverse of `DwellerServer`)
/// Payed by dweller.
#[repr(C)]
#[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug, Clone, BorshSchema, Default)]
pub struct ServerMember {
    pub version: StateVersion,
    /// server
    pub container: Pubkey,
    /// [Server::members] index used to derive address
    pub index: u64,
    pub dweller: Pubkey,
}

impl ServerMember {
    pub const LEN: u64 = 73;
    pub const SEED: &'static str = "ServerMember";
}

/// Dwellers who were invited.
#[repr(C)]
#[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug, Clone, BorshSchema, Default)]
pub struct ServerMemberStatus {
    pub version: StateVersion,
    /// server
    pub container: Pubkey,
    pub index: u64,
    pub dweller: Pubkey,
}

impl ServerMemberStatus {
    pub const LEN: u64 = 73;
    pub const SEED: &'static str = "ServerMemberStatus";
}

#[repr(C)]
#[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug, Clone, BorshSchema, Default)]
pub struct ServerAdministrator {
    pub version: StateVersion,
    /// server
    pub container: Pubkey,
    /// [Server::administrators] index used to derive address
    pub index: u64,
    pub dweller: Pubkey,
}

impl ServerAdministrator {
    pub const LEN: u64 = 73;
    pub const SEED: &'static str = "ServerAdministrator";
    pub fn is_initialized(&self) -> ProgramResult {
        if self.version == StateVersion::Uninitialized {
            Err(ProgramError::UninitializedAccount)
        } else {
            Ok(())
        }
    }
}

#[repr(C)]
#[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug, Clone, BorshSchema)]
pub struct Server {
    pub version: StateVersion,
    /// must be dweller, can add and remove admins
    pub owner: Pubkey,

    pub name: [u8; 32],

    /// empty hash is optional
    /// Photo identification of the dweller
    /// Multihash referencing IPFS hash of dwellers photo
    pub photo_hash: [u8; 64],

    /// optional additional access hash
    pub db_hash: [u8; 64],

    /// Server members whom have joined, index used to derive addresses
    pub members: u64,
    pub member_statuses: u64,
    pub administrators: u64,
    pub channels: u64,
    pub groups: u64,
}

impl Server {
    pub const LEN: u64 = 233;
}

#[repr(C)]
#[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug, Clone, BorshSchema, Default)]
pub struct ServerChannel {
    pub version: StateVersion,

    /// server
    pub container: Pubkey,
    /// [Server::channels] index used to derive address
    pub index: u64,
    pub type_id: u8,
    pub name: [u8; 32],
}

impl ServerChannel {
    pub const LEN: u64 = 74;
    pub const SEED: &'static str = "ServerChannel";
}

#[repr(C)]
#[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug, Clone, BorshSchema, Default)]
pub struct ServerGroup {
    pub version: StateVersion,
    /// server
    pub container: Pubkey,
    /// [Server::groups] index used to derive address
    pub index: u64,
    pub name: [u8; 32],

    pub channels: u64,
}

impl ServerGroup {
    pub const LEN: u64 = 81;
    pub const SEED: &'static str = "ServerGroup";
}

#[repr(C)]
#[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug, Clone, BorshSchema, Default)]
pub struct GroupChannel {
    pub version: StateVersion,
    /// group
    pub container: Pubkey,
    /// [Group::channels] index used to derive address
    pub index: u64,
    pub channel: Pubkey,
}

impl GroupChannel {
    pub const LEN: u64 = 73;
    pub const SEED: &'static str = "GroupChannel";
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn len() {
        assert_eq!(
            DwellerServer::LEN,
            solana_program::borsh::get_packed_len::<DwellerServer>() as u64
        );
        assert_eq!(
            Dweller::LEN,
            solana_program::borsh::get_packed_len::<Dweller>() as u64
        );
        assert_eq!(
            Server::LEN,
            solana_program::borsh::get_packed_len::<Server>() as u64
        );
        assert_eq!(
            ServerAdministrator::LEN,
            solana_program::borsh::get_packed_len::<ServerAdministrator>() as u64
        );
        assert_eq!(
            ServerChannel::LEN,
            solana_program::borsh::get_packed_len::<ServerChannel>() as u64
        );
        assert_eq!(
            ServerGroup::LEN,
            solana_program::borsh::get_packed_len::<ServerGroup>() as u64
        );
        assert_eq!(
            ServerMember::LEN,
            solana_program::borsh::get_packed_len::<ServerMember>() as u64
        );
        assert_eq!(
            ServerMemberStatus::LEN,
            solana_program::borsh::get_packed_len::<ServerMemberStatus>() as u64
        );
        assert_eq!(
            GroupChannel::LEN,
            solana_program::borsh::get_packed_len::<GroupChannel>() as u64
        );
    }
}
