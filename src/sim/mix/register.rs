/// Used for set the content of a register.
pub trait RegisterSet {
    /// Set the content of the register.
    ///
    /// # Arguments
    /// * `start` - The index of the first byte to set, inclusive.
    /// * `end` - The index of the last byte to set, inclusive.
    /// * `value` - The value to set the register to.
    ///
    /// # Panics
    /// * If `start` is less than 0.
    /// * If `start` is greater than or equal to `end`.
    /// * If `end` is greater than the length of the register.
    /// * If `value` is not the same length as the `end - start + 1`.
    fn set(&mut self, start: usize, end: usize, value: &[u8]);
}

/// A general register of MIX machine.
///
/// Each general register has 5 bytes and a sign bit.
pub struct GeneralRegister {
    /// The sign and other 5 bytes. Each byte should contain
    /// no more than 8 bits, or no larger than 64.
    ///
    /// `data[0]` is the sign bit.
    pub data: [u8; 6],
}

impl GeneralRegister {
    /// Create a new general register with default values.
    pub fn new() -> Self {
        GeneralRegister { data: [0; 6] }
    }
}

impl RegisterSet for GeneralRegister {
    fn set(&mut self, start: usize, end: usize, value: &[u8]) {
        assert!(start <= end);
        assert!(end < self.data.len());
        assert!(value.len() == end - start + 1);

        for i in start..=end {
            self.data[i] = value[i - start];
        }
    }
}

impl std::ops::Index<std::ops::Range<usize>> for GeneralRegister {
    type Output = [u8];

    /// Access the content of the general register with
    /// the given range.
    fn index(&self, index: std::ops::Range<usize>) -> &Self::Output {
        &self.data[index]
    }
}

/// An index register of MIX machine.
///
/// Each index register has 2 bytes and a sign bit.
pub struct IndexRegister {
    /// The sign and other 2 bytes. Each byte should contain
    /// no more than 8 bits, or no larger than 64.
    ///
    /// `data[0]` is the sign bit.
    pub data: [u8; 3],
}

impl IndexRegister {
    /// Create a new index register with default values.
    pub fn new() -> Self {
        IndexRegister { data: [0; 3] }
    }
}

impl RegisterSet for IndexRegister {
    fn set(&mut self, start: usize, end: usize, value: &[u8]) {
        assert!(start <= end);
        assert!(end < self.data.len());
        assert!(value.len() == end - start + 1);

        for i in start..=end {
            self.data[i] = value[i - start];
        }
    }
}

impl std::ops::Index<std::ops::Range<usize>> for IndexRegister {
    type Output = [u8];

    /// Access the content of the index register with
    /// the given range.
    fn index(&self, index: std::ops::Range<usize>) -> &Self::Output {
        &self.data[index]
    }
}

/// A jump register of MIX machine.
///
/// Each jump register has 2 bytes and a always-positive sign bit.
pub struct JumpRegister {
    /// The 3 bytes. Each byte should contain
    /// no more than 8 bits, or no larger than 64.
    ///
    /// `data[0]`, which is always 1 , is ignore.
    pub data: [u8; 3],
}

impl JumpRegister {
    /// Create a new jump register with default values.
    pub fn new() -> Self {
        JumpRegister { data: [1, 0, 0] }
    }
}

impl RegisterSet for JumpRegister {
    fn set(&mut self, start: usize, end: usize, value: &[u8]) {
        assert!(start <= end);
        assert!(end < self.data.len());
        assert!(value.len() == end - start + 1);

        for i in start..=end {
            self.data[i] = if i == 0 { 1 } else { value[i - start] };
        }
    }
}

impl std::ops::Index<std::ops::Range<usize>> for JumpRegister {
    type Output = [u8];

    /// Access the content of the jump register with
    /// the given range.
    fn index(&self, index: std::ops::Range<usize>) -> &Self::Output {
        &self.data[index]
    }
}

/// Values of the comparison indicator.
pub enum ComparisonIndicatorValue {
    EQUAL,
    LESSER,
    GREATER,
}
