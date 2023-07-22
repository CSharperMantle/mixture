//! MIX simulation framework.

use cfg_block::cfg_block;

mod alphabet;
mod instr;
mod mem;
mod mix_machine;
mod word;

pub use alphabet::*;
pub use instr::*;
pub use mem::*;
pub use mix_machine::*;
pub use word::*;

cfg_block! {
    #[cfg(feature = "io")] {
        mod io;
        pub use io::*;
    }
}

#[cfg(test)]
mod tests;
