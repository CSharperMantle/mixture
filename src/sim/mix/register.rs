use crate::sim::mix::*;

/// Values of the comparison indicator.
pub enum ComparisonIndicatorValue {
    EQUAL,
    LESSER,
    GREATER,
}

/// A generic register of a MIX machine.
pub type GenericRegister = mem::Word<6, false>;

/// An index register of a MIX machine.
pub type IndexRegister = mem::Word<3, false>;

/// A jump register of a MIX machine.
pub type JumpRegister = mem::Word<3, true>;
