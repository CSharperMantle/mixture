use crate::sim::mix::register::*;

/// The state of a MIX machine.
pub struct MixMachine {
    /// The register `rA`.
    pub r_a: GeneralRegister,
    /// The register `rX`.
    pub r_x: GeneralRegister,

    /// The register `rI1`.
    pub r_i1: IndexRegister,
    /// The register `rI2`.
    pub r_i2: IndexRegister,
    /// The register `rI3`.
    pub r_i3: IndexRegister,
    /// The register `rI4`.
    pub r_i4: IndexRegister,
    /// The register `rI5`.
    pub r_i5: IndexRegister,
    /// The register `rI6`.
    pub r_i6: IndexRegister,

    /// The register `rJ`.
    pub r_j: JumpRegister,

    pub toggle_overflow: bool,
    pub indicator_comp: ComparisonIndicatorValue,
}
