use crate::sim::mix::*;

/// The state of a MIX machine.
pub struct MixMachine {
    /// The register `rA`.
    pub r_a: register::GenericRegister,
    /// The register `rX`.
    pub r_x: register::GenericRegister,

    /// The register `rI1`.e
    pub r_i1: register::IndexRegister,
    /// The register `rI2`.
    pub r_i2: register::IndexRegister,
    /// The register `rI3`.
    pub r_i3: register::IndexRegister,
    /// The register `rI4`.
    pub r_i4: register::IndexRegister,
    /// The register `rI5`.
    pub r_i5: register::IndexRegister,
    /// The register `rI6`.
    pub r_i6: register::IndexRegister,

    /// The register `rJ`.
    pub r_j: register::JumpRegister,

    /// The overflow toggle.
    pub toggle_overflow: bool,

    /// The comparison indicator.
    pub indicator_comp: register::ComparisonIndicatorValue,

    /// The memory.
    pub mem: mem::Mem,

    /// The instruction pointer.
    pub pc: u32,
}

impl MixMachine {
    /// Create a new MIX machine.
    pub fn new() -> Self {
        MixMachine {
            r_a: register::GenericRegister::new(),
            r_x: register::GenericRegister::new(),
            r_i1: register::IndexRegister::new(),
            r_i2: register::IndexRegister::new(),
            r_i3: register::IndexRegister::new(),
            r_i4: register::IndexRegister::new(),
            r_i5: register::IndexRegister::new(),
            r_i6: register::IndexRegister::new(),
            r_j: register::JumpRegister::new(),
            toggle_overflow: false,
            indicator_comp: register::ComparisonIndicatorValue::EQUAL,
            mem: mem::Mem::new(),
            pc: 0,
        }
    }
}
