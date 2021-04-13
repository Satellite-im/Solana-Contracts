//! sattelite servers
//TODO: fix it #![deny(missing_docs)]

mod borsh;
pub mod error;
pub mod instruction;
pub mod processor;
#[cfg(test)]
mod processor_tests;
pub mod state;
#[cfg(test)]
mod test;

mod prelude;

#[cfg(not(feature = "no-entrypoint"))]
mod entrypoint;

// Export current sdk types for downstream users building with a different sdk version
pub use solana_program;

solana_program::declare_id!("BDhwBerjCPBbT6NpcwwQ4m923JCB56vC1fauSxfdhYHy");
