use crate::sim::mix::*;

/// A word in MIX machine, with variable number of bytes.
///
/// Word are the basic unit of memory in MIX. A normal word
/// contains 5 bytes and a sign byte. Note, however, that a
/// byte may contain *arbitrary* amount of bits. A proper MIX
/// program should run regardless of the number of bytes in a
/// word. It is thus impossible to tell the content of individual
/// bytes if several bytes are joined to represent a single
/// scalar.
///
/// A byte should be able to represent a scalar no less than
/// decimal `60`.
///
/// # Generic Parameters
/// * `N` - The number of bytes in the word, including sign.
/// * `P` - Whether the sign byte is always `1`, positive.
///
/// # Example
/// ```rust
/// use mixture::sim::mix::mem::*;
///
/// let mut word = Word::<6, false>::new();
///
/// word.set(0..=5, &[0, 1, 2, 3, 4, 5]).unwrap();
/// word.set(1..=2, &[6, 7]).unwrap();
/// assert_eq!(word[0..=5], [0, 6, 7, 3, 4, 5]);
///
/// let mut word_copy = word;
/// word_copy.set(0..=0, &[1]);
/// assert_eq!(word[0..=5], [0, 6, 7, 3, 4, 5]);
/// assert_eq!(word_copy[0..=5], [1, 6, 7, 3, 4, 5]);
/// ```
#[derive(Clone, Copy, Debug)]
pub struct Word<const N: usize, const P: bool> {
    data: [u8; N],
}

impl<const N: usize, const P: bool> Word<N, P> {
    /// Create a new word with default values.
    pub fn new() -> Self {
        let mut w: Word<N, P> = Word { data: [0; N] };
        if P {
            w.data[0] = 1;
        }
        w
    }

    /// Create a new word from an `i64`.
    ///
    /// The function stores big-endian representation of the
    /// given `i64` shifted to right. It means that if we have
    /// a `Word<6, false>` only 5 bytes starting from right will
    /// be stored. The sign byte is always `1` if `P` is `true`.
    ///
    /// # Arguments
    /// * `value` - The value to initialize the word with.
    ///
    /// # Returns
    /// * `Self` - The initialized word.
    /// * `bool` - `true` if the given `i64` is too large, `false` otherwise.
    ///
    /// # Example
    /// ```rust
    /// use mixture::sim::mix::mem::*;
    ///
    /// let v = -0x0102030405060708;
    ///
    /// let (word, overflow) = Word::<3, false>::from_i64(v);
    /// assert_eq!(overflow, true);
    /// assert_eq!(word[0..=2], [1, 0x07, 0x08]);
    /// ```
    pub fn from_i64(value: i64) -> (Self, bool) {
        let mut word = Self::new();
        let bytes = value.abs().to_be_bytes();
        // See if we have something not copied.
        // Bytes marked 'dirty' have not been copied yet.
        let mut bytes_dirty = bytes.map(|byte| byte != 0);
        word[0] = if !P && value < 0 { 1 } else { 0 };
        for (word_i, bytes_i) in (1..N).rev().zip((0..8).rev()) {
            word[word_i] = bytes[bytes_i];
            // We have copied the byte; make it clean.
            bytes_dirty[bytes_i] = false;
        }
        // If we have left some data behind, we have overflowed.
        (word, bytes_dirty.iter().any(|&dirty| dirty))
    }

    /// Set the content of the word.
    ///
    /// # Arguments
    /// * `range` - The range of bytes to set.
    /// * `value` - The value to set the register to.
    ///
    /// # Returns
    /// * `Ok(())` - If `start` is less than 0.
    /// * `Err(())` - If `range` is empty or given `value` is
    /// not the same length as `range`.
    ///
    /// # Example
    /// ```rust
    /// use mixture::sim::mix::mem::*;
    ///
    /// let mut word = Word::<6, false>::new();
    ///
    /// assert_eq!(word.set(0..=5, &[0, 1, 2, 3, 4, 5]), Ok(()));
    /// ```
    pub fn set(&mut self, range: std::ops::RangeInclusive<usize>, value: &[u8]) -> Result<(), ()> {
        if range.is_empty() {
            return Err(());
        }
        let start = *range.start();
        let end = *range.end();
        if end >= N || value.len() != end - start + 1 {
            return Err(());
        }

        for i in start..=end {
            // If we are always positive and we are setting sign,
            // then make it 1.
            self.data[i] = if P && i == 0 { 1 } else { value[i - start] };
        }

        Ok(())
    }

    /// Check if the word is positive.
    ///
    /// # Returns
    /// * `true` - If the word is positive, `word[0] == 0`.
    /// * `false` - If the word is negative, `word[0] == 1`.
    ///
    /// # Example
    /// ```rust
    /// use mixture::sim::mix::mem::*;
    ///
    /// let mut word = Word::<6, false>::new();
    /// word.set(0..=5, &[0, 1, 2, 3, 4, 5]).unwrap();
    ///
    /// assert_eq!(word.is_positive(), true);
    /// ```
    pub fn is_positive(&self) -> bool {
        self.data[0] == 0
    }

    /// Toggle the sign of the word.
    ///
    /// This method has no effect if the word is always positive,
    /// i.e. `P == true`.
    ///
    /// # Example
    /// ```rust
    /// use mixture::sim::mix::mem::*;
    ///
    /// let mut word = Word::<6, false>::new();
    /// word[0] = 0;
    ///
    /// word.toggle_sign();
    /// assert_eq!(word[0], 1);
    /// word.toggle_sign();
    /// assert_eq!(word[0], 0);
    /// ```
    pub fn toggle_sign(&mut self) {
        self.data[0] = if !P && self.is_positive() { 1 } else { 0 };
    }

    /// Convert the word to an `i64`.
    ///
    /// This method squashes big-endian representation of the bytes
    /// into a single quantity, ignoring too significant bytes.
    ///
    /// # Returns
    /// * `i64` - The converted value.
    /// * `bool` - `true` if the word is too large, `false` otherwise.
    ///
    /// # Example
    /// ```rust
    /// use mixture::sim::mix::mem::*;
    ///
    /// let mut word = Word::<6, false>::new();
    /// word.set(0..=5, &[0, 1, 2, 3, 4, 5]).unwrap();
    ///
    /// let (value, overflow) = word.to_i64();
    /// assert_eq!(overflow, false);
    /// assert_eq!(value, 0x0102030405);
    /// ```
    pub fn to_i64(&self) -> (i64, bool) {
        let sign = if self.is_positive() { 1 } else { -1 };
        let mut bytes: [u8; 8] = [0; 8];
        // Bytes marked 'dirty' have not been copied yet.
        let mut data_bytes_dirty = self.data.map(|byte| byte != 0);
        // Sign byte is always dealt properly.
        data_bytes_dirty[0] = false;
        for (bytes_i, data_i) in (0..8).rev().zip((1..N).rev()) {
            bytes[bytes_i] = self.data[data_i];
            // We have copied the byte; make it clean.
            data_bytes_dirty[data_i] = false;
        }
        let value = i64::from_be_bytes(bytes);
        (value * sign, data_bytes_dirty.iter().any(|&dirty| dirty))
    }
}

impl<const N: usize, const P: bool> std::ops::Index<std::ops::Range<usize>> for Word<N, P> {
    type Output = [u8];

    /// Access the content of the word with
    /// the given range.
    fn index(&self, index: std::ops::Range<usize>) -> &Self::Output {
        &self.data[index]
    }
}

impl<const N: usize, const P: bool> std::ops::Index<std::ops::RangeInclusive<usize>>
    for Word<N, P>
{
    type Output = [u8];

    /// Access the content of the word with
    /// the given range.
    fn index(&self, index: std::ops::RangeInclusive<usize>) -> &Self::Output {
        &self.data[index]
    }
}

impl<const N: usize, const P: bool> std::ops::Index<usize> for Word<N, P> {
    type Output = u8;

    /// Access the content of the word with
    /// the given index.
    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl<const N: usize, const P: bool> std::ops::IndexMut<usize> for Word<N, P> {
    /// Access the mutable content of the word with
    /// the given index.
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index]
    }
}

impl std::convert::TryFrom<instr::Instruction> for Word<6, false> {
    type Error = ();

    /// Convert an `Instruction` to a `Word<6, false>`.
    ///
    /// # Arguments
    /// * `source` - The instruction to convert.
    ///
    /// # Returns
    /// * `Ok(Word<6, false>)` - If the instruction is successful.
    /// * `Err(&'static str)` - If the instruction is invalid.
    ///
    /// # Example
    /// ```rust
    /// use mixture::sim::mix::mem::*;
    /// use mixture::sim::mix::instr::*;
    ///
    /// let instr = Instruction::new(2000, 0x03, 0x02, Opcode::LdA);
    ///
    /// let word: Word<6, false> = instr.try_into().unwrap();
    /// assert_eq!(word[0..=5], [0, 0x07, 0xD0, 0x02, 0x03, 0x08]);
    /// ```
    fn try_from(source: instr::Instruction) -> Result<Self, Self::Error> {
        let mut word: Word<6, false> = Word::new();
        word[0] = if source.addr < 0 { 1u8 } else { 0u8 };
        word.set(1..=2, &(source.addr.abs() as u16).to_be_bytes())?;
        word[3] = source.index;
        word[4] = source.field;
        word[5] = source.opcode as u8;
        Ok(word)
    }
}

/// The memory area of a MIX machine.
pub struct Mem {
    /// The memory area.
    data: [Word<6, false>; 4000],
}

impl Mem {
    /// Create a new memory area with all-zero words.
    ///
    /// # Returns
    /// * `Mem` - The new memory area.
    ///
    /// # Example
    /// ```rust
    /// use mixture::sim::mix::mem::*;
    ///
    /// let mem = Mem::new();
    /// assert_eq!(mem[0][0..=5], [0, 0, 0, 0, 0, 0]);
    /// ```
    pub fn new() -> Self {
        Mem {
            data: [Word::<6, false>::new(); 4000],
        }
    }
}

impl std::ops::Index<u16> for Mem {
    type Output = Word<6, false>;

    /// Access the word at a memory location.
    fn index(&self, index: u16) -> &Self::Output {
        &self.data[index as usize]
    }
}

impl std::ops::IndexMut<u16> for Mem {
    /// Access the mutable word at a memory location.
    fn index_mut(&mut self, index: u16) -> &mut Self::Output {
        &mut self.data[index as usize]
    }
}
