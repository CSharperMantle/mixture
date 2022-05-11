/// A word in MIX machine, with variable number of bytes.
///
/// # Generic Parameters
/// * `N` - The number of bytes in the word, including sign.
/// * `P` - Whether the sign byte is always `0`, positive.
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

/// The memory area of a MIX machine.
pub struct Mem {
    /// The memory area.
    data: [Word<6, false>; 4000],
}

impl Mem {
    /// Create a new memory area with all-zero words.
    pub fn new () -> Self {
        Mem { data: [Word::<6, false>::new(); 4000] }
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
