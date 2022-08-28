//! Simulation library for MIX computers described by Donald E. Knuth.
//! 
//! Mixture provides a robust simulation environment and binary interface
//! for MIX computers used extensively in *The Art of Computer Programming*
//! series written by D. E. Knuth. The binary format of MIX programs can be
//! easily created from textual code using the provided compiler driver.

pub mod parse;
pub mod sim;
