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
    
    /// Photo identification of the dweller
    /// Multihash referencing IPFS hash of dwellers photo
    pub photo_hash: Option<[u8;64]>,    

    // here could be some u8 for status before String , or brought to other account
    pub status : String,
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


/// Has program derived address from Server
#[repr(C)]
#[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug, Clone, BorshSchema)]
pub struct ServerMember {
    pub version: StateVersion,
    pub server:Pubkey,
    pub dweller: Pubkey,
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
    /// must be dweller
    pub owner: Pubkey,

    // can join and leave Dwellers into server
    pub authority: Pubkey,

    pub name: [u8;32],
    
    /// Photo identification of the dweller
    /// Multihash referencing IPFS hash of dwellers photo
    pub photo_hash: Option<[u8;64]>,    

    /// additional access hash
    pub db_hash: Option<[u8;64]>,

    // we store roles as separate account types, not as enum flag in member state as in original Solidity contract

    /// Server members whom have joined, index used to derive addresses
    pub members: u16,

    pub administrators:u8,
}



#[repr(C)]
#[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug, Clone, BorshSchema)]
pub struct ServerChannel {
    pub version: StateVersion,
}		
		
#[repr(C)]
#[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug, Clone, BorshSchema)]
pub struct ServerGroup {
    pub version: StateVersion,
}

#[repr(C)]
#[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug, Clone, BorshSchema)]
pub struct ServerGroupChannel {
    pub version: StateVersion,
}

/// Server members whom have joined
/// many to many map of `DwellerID` to `Server` (inverse of `DwellerServer`)
#[repr(C)]
#[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug, Clone, BorshSchema)]
pub struct ServerMember {
    pub version: StateVersion,
}