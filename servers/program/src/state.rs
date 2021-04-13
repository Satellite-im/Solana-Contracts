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
    pub servers: u8,

    /// This is the display name of a dweller
    pub name: [u8;32],
    
    /// Optional Photo identification of the dweller
    /// Multihash referencing IPFS hash of dwellers photo
    pub photo_hash: [u8;64],    

    /// string
    pub status : [u8;32],
}

/// Mapping of `Dweller` to `Server`.
/// Account address is be derived from `Dweller`
#[repr(C)]
#[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug, Clone, BorshSchema)]
pub struct DwellerServer {
    pub version: StateVersion,
    pub dweller: Pubkey,    
    pub server: Pubkey,
}

/// Server members whom have joined
/// Has program derived address from Server
/// many to many map of `DwellerID` to `Server` (inverse of `DwellerServer`)
/// Payed by dweller.
#[repr(C)]
#[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug, Clone, BorshSchema)]
pub struct ServerMember {
    pub version: StateVersion,
    pub server:Pubkey,
    pub dweller: Pubkey,
}

/// Dwellers who were invited. Payed by admin or registry pool(server)?
#[repr(C)]
#[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug, Clone, BorshSchema)]		
pub struct ServerMemberStatus {
    pub version: StateVersion,
    pub server:Pubkey,
    pub invited: bool,
}	

#[repr(C)]
#[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug, Clone, BorshSchema)]
pub struct ServerAdministrator {
    pub version: StateVersion,
    pub server:Pubkey,
    pub dweller: Pubkey,
}


#[repr(C)]
#[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug, Clone, BorshSchema)]
pub struct Server {
    pub version: StateVersion,
    /// must be dweller, can add and remove admins
    pub owner: Pubkey,

    pub name: [u8;32],
    
    /// empty hash is optional
    /// Photo identification of the dweller
    /// Multihash referencing IPFS hash of dwellers photo
    pub photo_hash: [u8;64],     

    /// optional additional access hash
    pub db_hash: [u8;64], 

    /// Server members whom have joined, index used to derive addresses
    pub members: u64,
    pub administrators:u32,  
    pub channels: u32,
    pub groups:u32,
    pub groups_channels:u32,

}

#[repr(C)]
#[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug, Clone, BorshSchema)]
pub struct ServerChannel {
    pub version: StateVersion,
    pub server:Pubkey,
    pub type_id : u8,
    pub name: [u8;32],
}		
		
#[repr(C)]
#[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug, Clone, BorshSchema)]
pub struct ServerGroup {
    pub version: StateVersion,
    pub server:Pubkey,
    pub name: [u8;32],
}

#[repr(C)]
#[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug, Clone, BorshSchema)]
pub struct ServerGroupChannel {
    pub version: StateVersion,
    pub server:Pubkey,
    pub group: Pubkey,
    pub channel: Pubkey,
}