use crate::common::*;
use crate::parse::maybe::Maybe;

pub struct AbstractInstruction {
    /// The abstract address.
    pub addr: Maybe<i16, i32>,

    /// The abstract field.
    pub field: Maybe<u8, i32>,

    /// The abstract index.
    pub index: Maybe<u8, i32>,

    /// The operation code.
    pub opcode: Opcode,
}

impl AbstractInstruction {
    /// Create a new abstract instruction.
    ///
    /// # Arguments
    /// * `addr` - The abstract address.
    /// * `field` - The abstract field.
    /// * `index` - The abstract index.
    /// * `opcode` - The operation code.
    pub const fn new(
        addr: Maybe<i16, i32>,
        field: Maybe<u8, i32>,
        index: Maybe<u8, i32>,
        opcode: Opcode,
    ) -> Self {
        Self {
            addr,
            field,
            index,
            opcode,
        }
    }
    /// Consume the object and turn it to a [`Instruction`].
    ///
    /// # Returns
    /// * [`Ok(Instruction)`] - The converted [`Instruction`].
    /// * [`Err(())`] - Any of the fields is [`Maybe::Placeholder`].
    ///
    /// # Example
    /// ```rust
    /// use mixture::*;
    ///
    /// let instr = parse::AbstractInstruction {
    ///     addr: parse::Maybe::Concrete(2000),
    ///     field: parse::Maybe::Concrete(0x03),
    ///     index: parse::Maybe::Concrete(0x02),
    ///     opcode: common::Opcode::LdA,
    /// };
    ///
    /// let instr = instr.concretize().unwrap();
    /// assert_eq!(instr.opcode, common::Opcode::LdA);
    /// assert_eq!(instr.field, 0x03);
    /// assert_eq!(instr.index, 0x02);
    /// assert_eq!(instr.addr, 2000);
    /// ```
    pub fn concretize(self) -> Result<Instruction, ()> {
        Ok(Instruction {
            addr: self.addr.try_unwrap()?,
            field: self.field.try_unwrap()?,
            index: self.index.try_unwrap()?,
            opcode: self.opcode,
        })
    }
}
