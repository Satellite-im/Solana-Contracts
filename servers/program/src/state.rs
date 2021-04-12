///! Registry types. Registry it self is off chain API.
/// So there should be Registry authorities which can do some operation on Dwellers stuff

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
    pub server_index_seed: u8,

    /// This is the display name of a dweller
    pub name: [u8;32],
    
    /// Photo identification of the dweller
    /// Multihash referencing IPFS hash of dwellers photo
    pub photo_hash: Option<[u8;64]>,    
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


#[repr(C)]
#[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug, Clone, BorshSchema)]
pub struct ServerMemberStatus {
    pub version: StateVersion,
    pub dweller: Pubkey,
    // here could be some u8 for status before String 
    pub status : String,
}

#[repr(C)]
#[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug, Clone, BorshSchema)]
pub struct Server {
    pub version: StateVersion,
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

/// many to many map of `DwellerID` to `Server` (inverse of `DwellerServer`)
#[repr(C)]
#[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug, Clone, BorshSchema)]
pub struct ServerMember {
    pub version: StateVersion,
}


#[repr(C)]
#[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug, Clone, BorshSchema)]
pub struct ServerAdministrator {
    pub version: StateVersion,
}