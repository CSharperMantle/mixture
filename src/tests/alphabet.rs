use crate::alphabet::*;

#[test]
fn test_alphabet_to_u8() {
    let a = Alphabet::A;

    let a_byte: u8 = a.try_into().unwrap();
    assert_eq!(a_byte, 1);
}

#[test]
fn test_alphabet_to_char() {
    let deg = Alphabet::Degree;

    let deg_char: char = deg.try_into().unwrap();
    assert_eq!(deg_char, '°');
}

#[test]
fn test_alphabet_u8_to_char() {
    let lp_byte = 42;

    let lp_char: char = Alphabet::try_from(lp_byte).unwrap().try_into().unwrap();
    assert_eq!(lp_char, '(');
}

#[test]
fn test_char_to_alphabet() {
    let lp_char = '(';

    let lp_byte: Alphabet = lp_char.try_into().unwrap();
    assert_eq!(lp_byte, Alphabet::LParen);
}
