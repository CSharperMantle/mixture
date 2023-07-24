/// The common alphabet used in [`MixVM`].
///
/// See D. E. Knuth, *The Art of Computer Programming*, Volume 1, pp 140
/// for more information.
///
/// [`MixVM`]: crate::sim::MixVM
#[derive(Clone, Copy, PartialEq, Eq, Debug, num_enum::TryFromPrimitive)]
#[repr(u8)]
pub enum Alphabet {
    /// The character '` `'.
    Space = 0,

    /// The character '`A`'.
    A = 1,

    /// The character '`B`'.
    B = 2,

    /// The character '`C`'.
    C = 3,

    /// The character '`D`'.
    D = 4,

    /// The character '`E`'.
    E = 5,

    /// The character '`F`'.
    F = 6,

    /// The character '`G`'.
    G = 7,

    /// The character '`H`'.
    H = 8,

    /// The character '`I`'.
    I = 9,

    /// The character '`'`'.
    SQuote = 10,

    /// The character '`J`'.
    J = 11,

    /// The character '`K`'.
    K = 12,

    /// The character '`L`'.
    L = 13,

    /// The character '`M`'.
    M = 14,

    /// The character '`N`'.
    N = 15,

    /// The character '`O`'.
    O = 16,

    /// The character '`P`'.
    P = 17,

    /// The character '`Q`'.
    Q = 18,

    /// The character '`R`'.
    R = 19,

    /// The character '`°`'.
    Degree = 20,

    /// The character '`"`'.
    DQuote = 21,

    /// The character '`S`'.
    S = 22,

    /// The character '`T`'.
    T = 23,

    /// The character '`U`'.
    U = 24,

    /// The character '`V`'.
    V = 25,

    /// The character '`W`'.
    W = 26,

    /// The character '`X`'.
    X = 27,

    /// The character '`Y`'.
    Y = 28,

    /// The character '`Z`'.
    Z = 29,

    /// The character '`0`'.
    Zero = 30,

    /// The character '`1`'.
    One = 31,

    /// The character '`2`'.
    Two = 32,

    /// The character '`3`'.
    Three = 33,

    /// The character '`4`'.
    Four = 34,

    /// The character '`5`'.
    Five = 35,

    /// The character '`6`'.
    Six = 36,

    /// The character '`7`'.
    Seven = 37,

    /// The character '`8`'.
    Eight = 38,

    /// The character '`9`'.
    Nine = 39,

    /// The character '`.`'.
    Dot = 40,

    /// The character '`,`'.
    Comma = 41,

    /// The character '`(`'.
    LParen = 42,

    /// The character '`)`'.
    RParen = 43,

    /// The character '`+`'.
    Plus = 44,

    /// The character '`-`'.
    Minus = 45,

    /// The character '`*`'.
    Star = 46,

    /// The character '`/`'.
    Slash = 47,

    /// The character '`=`'.
    Equal = 48,

    /// The character '`$`'.
    Dollar = 49,

    /// The character '`<`'.
    LAngle = 50,

    /// The character '`>`'.
    RAngle = 51,

    /// The character '`@`'.
    At = 52,

    /// The character '`;`'.
    SemiColon = 53,

    /// The character '`:`'.
    Colon = 54,

    /// The character '`‚`'.
    LowSQuote = 55,

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
        Ok(match value {
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
            Alphabet::SQuote => '\'',
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
            Alphabet::DQuote => '"',
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
            Alphabet::LParen => '(',
            Alphabet::RParen => ')',
            Alphabet::Plus => '+',
            Alphabet::Minus => '-',
            Alphabet::Star => '*',
            Alphabet::Slash => '/',
            Alphabet::Equal => '=',
            Alphabet::Dollar => '$',
            Alphabet::LAngle => '<',
            Alphabet::RAngle => '>',
            Alphabet::At => '@',
            Alphabet::SemiColon => ';',
            Alphabet::Colon => ':',
            Alphabet::LowSQuote => '‚',
        })
    }
}
