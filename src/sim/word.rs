use core::convert::From;
use core::ops::Index;
use core::ops::IndexMut;
use core::ops::RangeFull;
use core::ops::RangeInclusive;

use super::instr::Instruction;

/// A generic word in [`MixVM`] with `N` bytes in it.
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
/// * `P` - Whether the sign byte is always positive.
///
/// # Example
/// ```rust
/// use mixture::sim::*;
///
/// let mut word = Word::<6, false>::new();
///
/// word.set_all([0, 1, 2, 3, 4, 5]);
/// word[1..=2].clone_from_slice(&[6, 7]);
/// assert_eq!(word[..], [0, 6, 7, 3, 4, 5]);
///
/// let mut word_copy = word;
/// word_copy[0..=0].clone_from_slice(&[1]);
/// assert_eq!(word[..], [0, 6, 7, 3, 4, 5]);
/// assert_eq!(word_copy[..], [1, 6, 7, 3, 4, 5]);
/// ```
///
/// [`MixVM`]: crate::sim::MixVM
#[derive(Clone, Copy, Debug)]
pub struct Word<const N: usize, const P: bool> {
    data: [u8; N],
}

impl<const N: usize, const P: bool> Word<N, P> {
    /// Negative sign byte content.
    pub const NEG: u8 = 1;

    /// Positive sign byte content.
    pub const POS: u8 = 0;

    /// Create a new word with default values.
    ///
    /// Equivalent to [`Word<N, P>::default()`].
    pub const fn new() -> Self {
        let mut w: Word<N, P> = Word { data: [0; N] };
        w.data[0] = if P { Self::POS } else { w.data[0] };
        w
    }

    /// Create a new word from the given content.
    ///
    /// Sign byte settings of `P` will be honored.
    ///
    /// # Arguments
    /// * `bytes` - The content of the word.
    ///
    /// # Example
    /// ```rust
    /// use mixture::sim::*;
    ///
    /// let word = Word::<6, false>::from_bytes([0, 1, 2, 3, 4, 5]);
    /// assert_eq!(word[..], [0, 1, 2, 3, 4, 5]);
    /// ```
    pub const fn from_bytes(bytes: [u8; N]) -> Self {
        let mut w = Word { data: bytes };
        w.data[0] = if P { Self::POS } else { w.data[0] };
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
    /// * [`Word`] - The initialized word.
    /// * [`bool`] - `true` if the given `i64` is too large, `false` otherwise.
    ///
    /// # Example
    /// ```rust
    /// use mixture::sim::*;
    ///
    /// let v = -0x0102030405060708;
    ///
    /// let (word, overflow) = Word::<3, false>::from_i64(v);
    /// assert_eq!(overflow, true);
    /// assert_eq!(word[..], [1, 0x07, 0x08]);
    /// ```
    pub fn from_i64(value: i64) -> (Self, bool) {
        let mut word = Self::new();
        let bytes = value.abs().to_be_bytes();
        // See if we have something not copied.
        // Bytes marked 'dirty' have not been copied yet.
        let mut bytes_dirty = bytes.map(|byte| byte != 0);
        word[0] = if !P && value < 0 {
            Self::NEG
        } else {
            Self::POS
        };
        for (word_i, bytes_i) in (1..N).rev().zip((0..8).rev()) {
            word[word_i] = bytes[bytes_i];
            // We have copied the byte; make it clean.
            bytes_dirty[bytes_i] = false;
        }
        // If we have left some data behind, we have overflowed.
        (word, bytes_dirty.iter().any(|&dirty| dirty))
    }

    /// Set the content of the whole word to given array.
    ///
    /// # Arguments
    /// * `value` - The value to set.
    ///
    /// # Example
    /// ```rust
    /// use mixture::sim::*;
    ///
    /// let mut word = Word::<6, false>::new();
    ///
    /// word.set_all([0, 1, 2, 3, 4, 5]);
    ///
    /// assert_eq!(word[..], [0, 1, 2, 3, 4, 5]);
    /// ```
    pub fn set_all(&mut self, value: [u8; N]) {
        for i in 0..N {
            self.data[i] = value[i];
        }
        if P {
            self.data[0] = Self::POS;
        }
    }

    /// Check if the word is positive.
    ///
    /// # Returns
    /// * `true` - If the word is positive, `word[0] == Self::POS`.
    /// * `false` - If the word is negative, `word[0] != Self::POS`.
    ///
    /// # Example
    /// ```rust
    /// use mixture::sim::*;
    ///
    /// let mut word = Word::<6, false>::new();
    /// word.set_all([0, 1, 2, 3, 4, 5]);
    ///
    /// assert_eq!(word.is_positive(), true);
    /// ```
    pub const fn is_positive(&self) -> bool {
        self.data[0] == Self::POS
    }

    /// Get sign adjustment coefficient for the word.
    ///
    /// # Returns
    /// * `1` - If the word is positive.
    /// * `-1` - If the word is negative.
    ///
    /// # Example
    /// ```rust
    /// use mixture::sim::*;
    ///
    /// let mut word = Word::<6, false>::new();
    /// word.set_all([0, 1, 2, 3, 4, 5]);
    ///
    /// assert_eq!(word.get_sign(), 1);
    /// ```
    pub const fn get_sign(&self) -> i8 {
        if self.is_positive() {
            1
        } else {
            -1
        }
    }

    /// Flip the sign of the word.
    ///
    /// This method has no effect if the word is always positive,
    /// i.e. `P == true`.
    ///
    /// # Example
    /// ```rust
    /// use mixture::sim::*;
    ///
    /// let mut word = Word::<6, false>::new();
    /// word[0] = 0;
    ///
    /// word.flip_sign();
    /// assert_eq!(word[0], 1);
    /// word.flip_sign();
    /// assert_eq!(word[0], 0);
    /// ```
    pub fn flip_sign(&mut self) {
        self.data[0] = if !P && self.is_positive() {
            Self::NEG
        } else {
            Self::POS
        };
    }

    /// Convert the word to an `i64`.
    ///
    /// This method squashes big-endian representation of the bytes
    /// into a single quantity, ignoring too significant bytes.
    ///
    /// # Returns
    /// * [`i64`] - The converted value.
    /// * [`bool`] - `true` if the word overflows.
    ///
    /// # Example
    /// ```rust
    /// use mixture::sim::*;
    ///
    /// let mut word = Word::<6, false>::new();
    /// word.set_all([0, 1, 2, 3, 4, 5]);
    ///
    /// let (value, overflow) = word.to_i64();
    /// assert_eq!(overflow, false);
    /// assert_eq!(value, 0x0102030405);
    /// ```
    pub fn to_i64(&self) -> (i64, bool) {
        let sign = self.get_sign() as i64;
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

    /// Convert the corresponding range of an word to an `i64`.
    ///
    /// # Arguments
    /// * `field` - The field to convert. Value: `F <- L * 8 + R`.
    ///
    /// # Returns
    /// * [`i64`] - The converted value.
    /// * [`bool`] - `true` if the word overflows.
    ///
    /// # Example
    /// ```rust
    /// use mixture::sim::*;
    ///
    /// let mut word = Word::<6, false>::new();
    /// word.set_all([0, 1, 2, 3, 4, 5]);
    ///
    /// let (value, overflow) = word.to_i64_ranged(1..=1);
    /// assert_eq!(overflow, false);
    /// assert_eq!(value, 0x01);
    /// ```
    pub fn to_i64_ranged(&self, field: RangeInclusive<usize>) -> (i64, bool) {
        // Move sign byte out.
        let sign_included = *field.start() == 0;
        let new_start = if sign_included {
            *field.start() + 1
        } else {
            *field.start()
        };
        let field = new_start..=*field.end();
        // Get sliced data.
        let data = &self.data[field];
        // If the range is empty, fast-fail.
        if data.is_empty() {
            return (0, false);
        }
        // Find sign.
        let sign = if !sign_included { 1 } else { self.get_sign() } as i64;
        let mut result_bytes: [u8; 8] = [0; 8];
        // Get count of bytes that is needed to copy.
        let data_bytes_nonzero_count = data.iter().filter(|&&b| b != 0).count();
        // Copy bytes from the slice.
        // Ranges are chained by zip, and the shorter range is
        // iterated over in order to prevent out-of-bound indices.
        // Filling starts from the LSB.
        for (bytes_i, data_i) in (0..8).rev().zip((0..data.len()).rev()) {
            result_bytes[bytes_i] = data[data_i];
        }
        let value = i64::from_be_bytes(result_bytes);

        (value * sign, data_bytes_nonzero_count > 8)
    }
}

impl<const N: usize, const P: bool> Default for Word<N, P> {
    /// Create a new word with default value.
    ///
    /// Equivalent to [`Word<N, P>::new()`].
    fn default() -> Self {
        Self::new()
    }
}

impl<const N: usize, const P: bool> Index<RangeInclusive<usize>> for Word<N, P> {
    type Output = [u8];

    /// Access the content of the word with the given range.
    fn index(&self, index: RangeInclusive<usize>) -> &Self::Output {
        &self.data[index]
    }
}

impl<const N: usize, const P: bool> IndexMut<RangeInclusive<usize>> for Word<N, P> {
    /// Mutably access the content of the word with the given range.
    fn index_mut(&mut self, index: RangeInclusive<usize>) -> &mut Self::Output {
        &mut self.data[index]
    }
}

impl<const N: usize, const P: bool> Index<RangeFull> for Word<N, P> {
    type Output = [u8];

    /// Access the whole word.
    fn index(&self, index: RangeFull) -> &Self::Output {
        &self.data[index]
    }
}

impl<const N: usize, const P: bool> IndexMut<RangeFull> for Word<N, P> {
    /// Mutably access the whole word.
    fn index_mut(&mut self, index: RangeFull) -> &mut Self::Output {
        &mut self.data[index]
    }
}

impl<const N: usize, const P: bool> Index<usize> for Word<N, P> {
    type Output = u8;

    /// Access the content of the word with the given index.
    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl<const N: usize, const P: bool> IndexMut<usize> for Word<N, P> {
    /// Mutably Access the content of the word with the given index.
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index]
    }
}

impl From<Instruction> for Word<6, false> {
    /// Convert an [`Instruction`] to a [`Word<6, false>`].
    ///
    /// # Arguments
    /// * `source` - The instruction to convert.
    ///
    /// # Returns
    /// * [`Word<6, false>`] - Converted [`Word`].
    ///
    /// # Example
    /// ```rust
    /// use mixture::sim::*;
    /// use mixture::sim::*;
    ///
    /// let instr = Instruction::new(2000, 0x03, 0x02, Opcode::LdA);
    ///
    /// let word: Word<6, false> = instr.into();
    /// assert_eq!(word[..], [0, 0x07, 0xD0, 0x02, 0x03, 0x08]);
    /// ```
    fn from(source: Instruction) -> Self {
        let source_addr = source.addr.unsigned_abs().to_be_bytes();
        Self::from_bytes([
            u8::from(source.addr < 0),
            source_addr[0],
            source_addr[1],
            source.index,
            source.field,
            source.opcode as u8,
        ])
    }
}

/// Alias for a 6-byte [`Word`] including a sign byte.
pub type FullWord = Word<6, false>;

/// Alias for a 3-byte [`Word`] including a sign byte.
pub type HalfWord = Word<3, false>;

/// Alias for a 3-byte [`Word`] including a sign byte,
/// which is always equal to [`Word::POS`].
pub type PosHalfWord = Word<3, true>;
