use mixture::sim::mix::mem::*;

#[test]
fn test_word_mutation() {
    let mut reg = Word::<6, false>::new();

    assert_eq!(reg[0..6], [0; 6]);
    reg.set(0..=5, &[1, 2, 3, 4, 5, 6]).unwrap();
    assert_eq!(reg[0..6], [1, 2, 3, 4, 5, 6]);
    reg.set(0..=2, &[1, 8, 9]).unwrap();
    assert_eq!(reg[0..6], [1, 8, 9, 4, 5, 6]);
    reg.set(2..=5, &[1, 2, 3, 4]).unwrap();
    assert_eq!(reg[0..6], [1, 8, 1, 2, 3, 4]);
}

#[test]
fn test_word_pos_sign_mutation() {
    let mut reg = Word::<3, true>::new();

    assert_eq!(reg[0..3], [1, 0, 0]);
    reg.set(0..=2, &[1, 2, 3]).unwrap();
    assert_eq!(reg[0..3], [1, 2, 3]);
    reg.set(0..=2, &[0, 0, 0]).unwrap();
    assert_eq!(reg[0..3], [1, 0, 0]);
}

#[test]
fn test_word_interchange() {
    let mut reg1 = Word::<6, false>::new();
    let mut reg2 = Word::<6, false>::new();
    reg1.set(0..=5, &[1, 2, 3, 4, 5, 6]).unwrap();
    reg2.set(0..=5, &[1, 8, 9, 10, 11, 12]).unwrap();

    reg1.set(3..=5, &reg2[3..6]).unwrap();
    assert_eq!(reg1[0..6], [1, 2, 3, 10, 11, 12]);
    assert_eq!(reg2[0..6], [1, 8, 9, 10, 11, 12]);
    reg2.set(0..=2, &reg1[0..3]).unwrap();
    assert_eq!(reg1[0..6], [1, 2, 3, 10, 11, 12]);
    assert_eq!(reg2[0..6], [1, 2, 3, 10, 11, 12]);
}

#[test]
fn test_word_error_handling() {
    let mut reg = Word::<6, false>::new();

    assert_eq!(reg.set(8..=0, &[]), Err(()));
    assert_eq!(reg.set(0..=100, &[]), Err(()));
    assert_eq!(reg.set(0..=2, &[]), Err(()));
    assert_eq!(reg.set(0..=2, &[1, 2, 3, 4, 5, 6, 7]), Err(()));
}
