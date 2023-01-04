use crate::common::Opcode;

/// Pseudo-operations in MIXAL.
#[derive(PartialEq, Eq)]
pub enum PseudoOpcode {
    /// * `EQU` - Define a macro named as `LOC` and valued as `ADDR`.
    Equ,

    /// * `ORIG` - Set the memory location for next instructions.
    Orig,

    /// * `CON` - Assemble `ADDR` into current memory location.
    Con,

    /// * `ALF` - Insert encoded characters specified in `ADDR` into current 
    /// memory location.
    Alf,

    /// * `END` - Denote the end of a MIXAL program, before which memory cells
    /// implicitly generated by assembler can be inserted.
    End,
}

/// An operation code encountered in assembling process.
#[derive(PartialEq, Eq)]
pub enum Op {
    /// A pseudo-operation as defined in [`PseudoOpcode`].
    Pseudo(PseudoOpcode),

    /// A real MIX operation as defined in [`crate::common::Opcode`].
    Concrete(Opcode),
}