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
    /// let mut reg = Word::<6, false>::new();
    ///
    /// assert_eq!(reg.set(0..=5, &[0, 1, 2, 3, 4, 5]), Ok(()));
    ///
    /// assert_eq!(reg.set(8..=0, &[]), Err(()));
    /// assert_eq!(reg.set(0..=100, &[]), Err(()));
    /// assert_eq!(reg.set(0..=2, &[]), Err(()));
    /// assert_eq!(reg.set(0..=2, &[1, 2, 3, 4, 5, 6, 7]), Err(()));
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

impl std::convert::TryFrom<instr::Instruction> for Word<6, false> {
    type Error = &'static str;

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
    /// assert_eq!(word[0..=5], [0, 0xD0, 0x07, 0x02, 0x03, 0x08]);
    /// ```
    fn try_from(source: instr::Instruction) -> Result<Self, Self::Error> {
        let mut word: Word<6, false> = Word::new();
        let sign = if source.addr < 0 { 1u8 } else { 0u8 };
        match word.set(0..=0, &[sign]) {
            Ok(_) => {}
            Err(_) => return Err("Failed to set sign byte"),
        };
        let addr = (source.addr.abs() as u16).to_le_bytes();
        match word.set(1..=2, &addr) {
            Ok(_) => {}
            Err(_) => return Err("Failed to set address bytes"),
        };
        match word.set(3..=3, &[source.index]) {
            Ok(_) => {}
            Err(_) => return Err("Failed to set index byte"),
        };
        match word.set(4..=4, &[source.field as u8]) {
            Ok(_) => {}
            Err(_) => return Err("Failed to set field byte"),
        };
        match word.set(5..=5, &[source.opcode as u8]) {
            Ok(_) => {}
            Err(_) => return Err("Failed to set opcode bytes"),
        };
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
    pub fn new() -> Self {
        Mem {
            data: [Word::<6, false>::new(); 4000],
        }
    }
}

impl std::ops::Index<usize> for Mem {
    type Output = Word<6, false>;

    /// Access the content of the word with
    /// the given range.
    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}
