#![doc = include_str!("../README.md")]

#![no_std]
#![deny(clippy::all)]
#![deny(clippy::unwrap_used)]
#![allow(clippy::result_unit_err)]

#[cfg(any(feature = "std", test))]
extern crate std;

#[cfg(all(not(feature = "std"), not(test)))]
extern crate core;

use cfg_block::cfg_block;

mod alphabet;
mod instr;
mod mem;
mod mix_vm;

pub use alphabet::*;
pub use instr::*;
pub use mem::*;
pub use mix_vm::*;

cfg_block! {
    #[cfg(feature = "io")] {
        mod io;
        pub use io::*;
    }
}

#[cfg(test)]
mod tests;