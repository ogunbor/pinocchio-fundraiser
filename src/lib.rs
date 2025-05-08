#![no_std]
#![allow(unexpected_cfgs)]

#[cfg(feature = "std")]
extern crate std;
#[cfg(not(feature = "no-entrypoint"))]

mod entrypoint;
pub mod error;
pub mod instruction;
pub mod state;
pub mod utils;
pub mod constants;
pinocchio_pubkey::declare_id!("Tz5qRmYdUpJ8KA1WnEcXBv2GRZ3tuFHQy6NMk97LsogT");
