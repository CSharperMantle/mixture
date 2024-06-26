#![doc = include_str!("../README.md")]
#![no_std]
#![deny(clippy::all)]
#![allow(clippy::result_unit_err)]

#[cfg(any(feature = "std", test))]
extern crate std;

#[cfg(all(not(feature = "std"), not(test)))]
extern crate core;

use cfg_block::cfg_block;

mod alphabet;
pub use alphabet::*;

mod instr;
pub use instr::*;

mod mem;
pub use mem::*;

mod mix_vm;
pub use mix_vm::*;

cfg_block! {
    #[cfg(feature = "io")] {
        mod io;
        pub use io::*;
    }
}

#[cfg(test)]
mod tests;
