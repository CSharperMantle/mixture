
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
    /// use mixture::common::Alphabet;
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
    /// use mixture::common::Alphabet;
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
