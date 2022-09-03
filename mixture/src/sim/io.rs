use std::convert::TryFrom;

use crate::sim::*;

/// A device plugged into a [`MixMachine`] to perform IO
/// operations.
/// 
/// This trait is used to build IO operations that may have side
/// effects. Implement the trait and insert the device to a [`MixMachine`]
/// instance to apply it.
/// 
/// # Example
/// ```rust
/// use mixture::sim::IODevice;
/// use mixture::sim::MixMachine;
/// use mixture::sim::FullWord;
///
/// pub struct SomeDevice {}
/// 
/// impl IODevice for SomeDevice {
///     fn read(&mut self) -> Result<Vec<FullWord>, ()> {
///         /* ... */
///         unimplemented!()
///     }
///     fn write(&mut self, data: &[FullWord]) -> Result<(), usize> {
///         /* ... */
///         unimplemented!()
///     }
///     fn control(&mut self, command: i16) -> Result<(), ()> {
///         /* ... */
///         unimplemented!()
///     }
///     fn is_busy(&self) -> Result<bool, ()> {
///         /* ... */
///         unimplemented!()
///     }
///     fn is_ready(&self) -> Result<bool, ()> {
///         /* ... */
///         unimplemented!()
///     }
///     fn get_block_size(&self) -> usize {
///         /* ... */
///         unimplemented!()
///     }
/// }
/// 
/// let mut mix = MixMachine::new();
/// mix.reset();
/// mix.io_devices[0] = Some(Box::new(SomeDevice {}));
/// ```
pub trait IODevice {
    /// Read a block of [`FullWord`]s from the device into the buffer.
    ///
    /// The amount of words in a block is defined by the device
    /// via [`IODevice::get_block_size`]. This method must return
    /// exactly one block of words on success. Otherwise it will
    /// fail.
    fn read(&mut self) -> Result<Vec<mem::FullWord>, ()>;

    /// Write a block of [`FullWord`]s out through the device.
    ///
    /// This method will always try to write a whole block. It will fail
    /// if the given slice of data has a length that is not exactly equal
    /// to the block size. On the case of non-rolling-back write failures,
    /// the actual amount of words already written is returned.
    ///
    /// # Arguments
    /// * `data` - The words to write.
    fn write(&mut self, data: &[mem::FullWord]) -> Result<(), usize>;

    /// Issue a control command to the device.
    ///
    /// # Arguments
    /// * `command` - The command to issue.
    fn control(&mut self, command: i16) -> Result<(), ()>;

    /// Check if the device is busy.
    /// 
    /// Note that when a device detects any malfunctions, like
    /// paper jams, it will always appear busy.
    fn is_busy(&self) -> Result<bool, ()>;

    /// Check if the device is ready for next operations.
    fn is_ready(&self) -> Result<bool, ()>;

    /// Get the count of [`FullWord`]s in a device block,
    /// that is, read or written in a single operation.
    fn get_block_size(&self) -> usize;
}

/// The common alphabet used in [`MixMachine`].
///
/// See D. E. Knuth, *The Art of Computer Programming*, Volume 1, pp 140
/// for more information.
#[derive(Clone, Copy, PartialEq, Eq, Debug, num_enum::TryFromPrimitive)]
#[repr(u8)]
pub enum Alphabet {
    Space = 0,
    A = 1,
    B = 2,
    C = 3,
    D = 4,
    E = 5,
    F = 6,
    G = 7,
    H = 8,
    I = 9,
    SingleQuote = 10,
    J = 11,
    K = 12,
    L = 13,
    M = 14,
    N = 15,
    O = 16,
    P = 17,
    Q = 18,
    R = 19,
    Degree = 20,
    DoubleQuote = 21,
    S = 22,
    T = 23,
    U = 24,
    V = 25,
    W = 26,
    X = 27,
    Y = 28,
    Z = 29,
    Zero = 30,
    One = 31,
    Two = 32,
    Three = 33,
    Four = 34,
    Five = 35,
    Six = 36,
    Seven = 37,
    Eight = 38,
    Nine = 39,
    Dot = 40,
    Comma = 41,
    LeftParenthesis = 42,
    RightParenthesis = 43,
    Plus = 44,
    Minus = 45,
    Star = 46,
    Slash = 47,
    Equal = 48,
    Dollar = 49,
    LeftAngle = 50,
    RightAngle = 51,
    At = 52,
    SemiColon = 53,
    Colon = 54,
    LowSingleQuote = 55,
}

impl TryFrom<Alphabet> for u8 {
    type Error = ();

    /// Converts an [`Alphabet`] to its numerical representation.
    ///
    /// # Returns
    /// * [`Ok(u8)`] - The converted byte.
    /// * [`Err(())`] - The conversion fails.
    ///
    /// # Example
    /// ```rust
    /// use mixture::sim::Alphabet;
    ///
    /// let a = Alphabet::A;
    /// let a_byte: u8 = a.try_into().unwrap();
    /// assert_eq!(a_byte, 1);
    /// ```
    fn try_from(value: Alphabet) -> Result<Self, Self::Error> {
        Ok(value as u8)
    }
}

impl TryFrom<Alphabet> for char {
    type Error = ();

    /// Converts an [`Alphabet`] to a [`char`].
    ///
    /// # Returns
    /// * [`Ok(char)`] - The converted [`char`].
    /// * [`Err(())`] - The conversion fails.
    ///
    /// # Example
    /// ```rust
    /// use mixture::sim::Alphabet;
    ///
    /// let a = Alphabet::A;
    /// let a_chr: char = a.try_into().unwrap();
    /// assert_eq!(a_chr, 'A');
    /// ```
    fn try_from(value: Alphabet) -> Result<Self, Self::Error> {
        let c = match value {
            Alphabet::Space => ' ',
            Alphabet::A => 'A',
            Alphabet::B => 'B',
            Alphabet::C => 'C',
            Alphabet::D => 'D',
            Alphabet::E => 'E',
            Alphabet::F => 'F',
            Alphabet::G => 'G',
            Alphabet::H => 'H',
            Alphabet::I => 'I',
            Alphabet::SingleQuote => '\'',
            Alphabet::J => 'J',
            Alphabet::K => 'K',
            Alphabet::L => 'L',
            Alphabet::M => 'M',
            Alphabet::N => 'N',
            Alphabet::O => 'O',
            Alphabet::P => 'P',
            Alphabet::Q => 'Q',
            Alphabet::R => 'R',
            Alphabet::Degree => '°',
            Alphabet::DoubleQuote => '"',
            Alphabet::S => 'S',
            Alphabet::T => 'T',
            Alphabet::U => 'U',
            Alphabet::V => 'V',
            Alphabet::W => 'W',
            Alphabet::X => 'X',
            Alphabet::Y => 'Y',
            Alphabet::Z => 'Z',
            Alphabet::Zero => '0',
            Alphabet::One => '1',
            Alphabet::Two => '2',
            Alphabet::Three => '3',
            Alphabet::Four => '4',
            Alphabet::Five => '5',
            Alphabet::Six => '6',
            Alphabet::Seven => '7',
            Alphabet::Eight => '8',
            Alphabet::Nine => '9',
            Alphabet::Dot => '.',
            Alphabet::Comma => ',',
            Alphabet::LeftParenthesis => '(',
            Alphabet::RightParenthesis => ')',
            Alphabet::Plus => '+',
            Alphabet::Minus => '-',
            Alphabet::Star => '*',
            Alphabet::Slash => '/',
            Alphabet::Equal => '=',
            Alphabet::Dollar => '$',
            Alphabet::LeftAngle => '<',
            Alphabet::RightAngle => '>',
            Alphabet::At => '@',
            Alphabet::SemiColon => ';',
            Alphabet::Colon => ':',
            Alphabet::LowSingleQuote => '‚',
        };
        Ok(c)
    }
}
