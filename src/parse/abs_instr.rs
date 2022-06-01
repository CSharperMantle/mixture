use crate::parse::maybe::Maybe;
use crate::*;

pub struct AbstractInstruction {
    /// The abstract address.
    pub addr: Maybe<i16, i32>,

    /// The abstract field.
    pub field: Maybe<u8, i32>,

    /// The abstract index.
    pub index: Maybe<u8, i32>,

    /// The operation code.
    pub opcode: sim::Opcode,
}

impl AbstractInstruction {
    /// Create a new abstract instruction.
    /// 
    /// # Arguments
    /// * `addr` - The abstract address.
    /// * `field` - The abstract field.
    /// * `index` - The abstract index.
    /// * `opcode` - The operation code.
    /// 
    /// # Returns
    /// * `AbstractInstruction` - The abstract instruction.
    pub const fn new(
        addr: Maybe<i16, i32>,
        field: Maybe<u8, i32>,
        index: Maybe<u8, i32>,
        opcode: sim::Opcode,
    ) -> Self {
        Self {
            addr,
            field,
            index,
            opcode,
        }
    }
}
