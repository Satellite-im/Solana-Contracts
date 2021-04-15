///! Registry types.
use borsh::{BorshDeserialize, BorshSchema, BorshSerialize};
use num_derive::{FromPrimitive, ToPrimitive};
use solana_program::pubkey::Pubkey;

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
    pub const LEN:  u64 = 300;
}

/// Mapping of `Dweller` to `Server`.
/// Account address is be derived from `Dweller`
#[repr(C)]
#[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug, Clone, BorshSchema)]
pub struct DwellerServer {
    pub version: StateVersion,
    /// dweller
    pub container: Pubkey,
    /// [Dweller::servers] index used to derive address
    pub index: u64,
    pub server: Pubkey,
}

impl DwellerServer {
    pub const LEN:  u64 = 300;
}

/// Server members whom have joined
/// Has program derived address from Server
/// many to many map of `Server` to `DwellerID` (inverse of `DwellerServer`)
/// Payed by dweller.
#[repr(C)]
#[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug, Clone, BorshSchema)]
pub struct ServerMember {
    pub version: StateVersion,
    /// server
    pub container: Pubkey,
    /// [Server::members] index used to derive address
    pub index: u64,
    pub dweller: Pubkey,
}

/// Dwellers who were invited.
#[repr(C)]
#[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug, Clone, BorshSchema)]
pub struct ServerMemberStatus {
    pub version: StateVersion,
    /// server
    pub container: Pubkey,
    pub index: u64,
    pub dweller: Pubkey,
    pub invited: bool,
}

#[repr(C)]
#[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug, Clone, BorshSchema)]
pub struct ServerAdministrator {
    pub version: StateVersion,
    /// server
    pub container: Pubkey,
    /// [Server::administrators] index used to derive address
    pub index: u64,
    pub dweller: Pubkey,
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
    pub groups_channels: u64,
}

#[repr(C)]
#[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug, Clone, BorshSchema)]
pub struct ServerChannel {
    pub version: StateVersion,

    /// server
    pub container: Pubkey,
    /// [Server::channels] index used to derive address
    pub index: u64,
    pub type_id: u8,
    pub name: [u8; 32],
}

#[repr(C)]
#[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug, Clone, BorshSchema)]
pub struct ServerGroup {
    pub version: StateVersion,
    /// server
    pub container: Pubkey,
    /// [Server::groups] index used to derive address
    pub index: u64,
    pub name: [u8; 32],
}

#[repr(C)]
#[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug, Clone, BorshSchema)]
pub struct ServerGroupChannel {
    pub version: StateVersion,
    pub container: Pubkey,
    /// [Server::group_channels] index used to derive address
    pub index: u64,
    pub group: Pubkey,
    pub channel: Pubkey,
}
