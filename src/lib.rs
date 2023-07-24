#![doc = include_str!("../README.md")]

#![no_std]
#![deny(clippy::all)]
#![deny(clippy::unwrap_used)]
#![allow(clippy::result_unit_err)]

#[cfg(any(feature = "std", test))]
extern crate std;

#[cfg(all(not(feature = "std"), not(test)))]
extern crate core;

pub mod sim;
