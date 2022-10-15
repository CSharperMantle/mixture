use core::ops::Index;
use core::ops::IndexMut;
use core::ops::Range;

use crate::common::*;

/// The memory area of a [`crate::sim::mix_machine::MixMachine`]
/// with [`Mem::SIZE`] cells.
#[derive(Debug, Clone)]
pub struct Mem {
    /// The memory area.
    data: [FullWord; Self::SIZE],
}

impl Mem {
    /// Create a new memory area with all-zero words.
    ///
    /// # Returns
    /// * [`Mem`] - The new memory area.
    ///
    /// # Example
    /// ```rust
    /// use mixture::sim::*;
    ///
    /// let mem = Mem::new();
    /// assert_eq!(mem[0][0..=5], [0, 0, 0, 0, 0, 0]);
    /// ```
    pub const fn new() -> Self {
        Mem {
            data: [Word::<6, false>::new(); 4000],
        }
    }

    /// Number of words in the memory area.
    pub const SIZE: usize = 4000;
}

impl Index<u16> for Mem {
    type Output = FullWord;

    /// Access the word at a memory location.
    fn index(&self, index: u16) -> &Self::Output {
        &self.data[index as usize]
    }
}

impl IndexMut<u16> for Mem {
    /// Access the mutable word at a memory location.
    fn index_mut(&mut self, index: u16) -> &mut Self::Output {
        &mut self.data[index as usize]
    }
}

impl Index<Range<usize>> for Mem {
    type Output = [FullWord];

    /// Access the word at a range.
    fn index(&self, index: Range<usize>) -> &Self::Output {
        &self.data[index]
    }
}

impl IndexMut<Range<usize>> for Mem {
    /// Access the mutable word at a range.
    fn index_mut(&mut self, index: Range<usize>) -> &mut Self::Output {
        &mut self.data[index]
    }
}

impl Default for Mem {
    /// Create a clean memory area.
    ///
    /// Equivalent to [`Mem::new`].
    fn default() -> Self {
        Self::new()
    }
}