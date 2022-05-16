use crate::sim::mix::*;

/// Values of the comparison indicator.
#[derive(PartialEq, Eq, Debug)]
pub enum ComparisonIndicatorValue {
    Equal,
    Lesser,
    Greater,
}

/// A generic register of a MIX machine.
pub type GenericRegister = mem::Word<6, false>;

/// An index register of a MIX machine.
pub type IndexRegister = mem::Word<3, false>;

/// A jump register of a MIX machine.
pub type JumpRegister = mem::Word<3, true>;
