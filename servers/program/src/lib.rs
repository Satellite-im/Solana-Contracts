//! Satellite servers
#![deny(missing_docs)]

mod borsh;
pub mod error;

/// instruction
pub mod instruction;
mod math;
mod prelude;
pub mod processor;

pub mod program;
/// state
pub mod state;

#[cfg(not(feature = "no-entrypoint"))]
mod entrypoint;

// Export current sdk types for downstream users building with a different sdk version
pub use solana_program;

solana_program::declare_id!("BDhwBerjCPBbT6NpcwwQ4m923JCB56vC1fauSxfdhYHy");
