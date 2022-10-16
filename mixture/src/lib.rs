//! Simulation library for MIX computers described by Donald E. Knuth.
//!
//! Mixture provides a robust simulation environment and
//! binary interface for MIX computers used extensively in *The Art of Computer
//! Programming* series written by D. E. Knuth, currently featuring:
//!
//! * Easy-to-inspect single-step MIX simulation via [`sim::MixMachine`]
//! * I/O device simulation via [`sim::IODevice`] (enabled by `io` feature)
//! * `#[no_std]` compatibility
//!
//! ## Crate Features
//!
//! All features are enabled by default.
//!
//! * `std` enables functionalities built on `std` lib.
//! * `io` enables I/O features. **Depend on `std`.**
//!

#![no_std]

#![deny(clippy::all)]
#![warn(clippy::cargo)]
#![allow(unused_imports)]
#![allow(clippy::result_unit_err)]

#[cfg(any(feature = "std", test))]
extern crate std;

#[cfg(all(not(feature = "std"), not(test)))]
extern crate core;

pub mod common;
pub mod parse;
pub mod sim;
