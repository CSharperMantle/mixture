use crate::sim::mem::*;

#[test]
fn test_clone() {
    let mut word = Word::<6, false>::new();
    word.set_all([0, 1, 2, 3, 4, 5]);

    let mut word2 = word.clone();
    assert_eq!(word[..], word2[..]);

    word2.set_all([0, 6, 7, 8, 9, 0]);
    assert_ne!(word[..], word2[..]);
}

#[test]
fn test_from_bytes() {
    let word = Word::<6, false>::from_bytes([1, 2, 3, 4, 5, 6]);
    assert_eq!(word[..], [1, 2, 3, 4, 5, 6]);

    let word_pos = Word::<6, true>::from_bytes([1, 2, 3, 4, 5, 6]);
    assert_eq!(word_pos[..], [0, 2, 3, 4, 5, 6]);
}

#[test]
fn test_mutation() {
    let mut word = Word::<6, false>::new();

    assert_eq!(word[..], [0; 6]);
    word.set_all([1, 2, 3, 4, 5, 6]);
    assert_eq!(word[..], [1, 2, 3, 4, 5, 6]);
    word[0..=2].copy_from_slice(&[1, 8, 9]);
    assert_eq!(word[..], [1, 8, 9, 4, 5, 6]);
    word[2..=5].copy_from_slice(&[1, 2, 3, 4]);
    assert_eq!(word[..], [1, 8, 1, 2, 3, 4]);
}

#[test]
fn test_pos_sign_mutation() {
    let mut word = Word::<3, true>::new();

    assert_eq!(word[..], [0, 0, 0]);
    word.set_all([1, 2, 3]);
    assert_eq!(word[..], [0, 2, 3]);
    word.set_all([0, 0, 0]);
    assert_eq!(word[..], [0, 0, 0]);
}

#[test]
fn test_interchange() {
    let mut word_1 = Word::<6, false>::new();
    let mut word_2 = Word::<6, false>::new();
    word_1.set_all([1, 2, 3, 4, 5, 6]);
    word_2.set_all([1, 8, 9, 10, 11, 12]);

    word_1[3..=5].copy_from_slice(&word_2[3..=5]);
    assert_eq!(word_1[..], [1, 2, 3, 10, 11, 12]);
    assert_eq!(word_2[..], [1, 8, 9, 10, 11, 12]);
    word_2[0..=2].copy_from_slice(&word_1[0..=2]);
    assert_eq!(word_1[..], [1, 2, 3, 10, 11, 12]);
    assert_eq!(word_2[..], [1, 2, 3, 10, 11, 12]);
}

#[test]
fn test_from_i64() {
    let v = -0x0102030405060708;
    let v_small = 0x01i64;

    let (word, overflow) = Word::<9, false>::from_i64(v);
    assert_eq!(overflow, false);
    assert_eq!(
        word[0..=8],
        [1, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08]
    );

    let (word, overflow) = Word::<3, false>::from_i64(v);
    assert_eq!(overflow, true);
    assert_eq!(word[..], [1, 0x07, 0x08]);

    let (word, overflow) = Word::<3, true>::from_i64(v);
    assert_eq!(overflow, true);
    assert_eq!(word[..], [0, 0x07, 0x08]);

    let (word, overflow) = Word::<3, false>::from_i64(v_small);
    assert_eq!(overflow, false);
    assert_eq!(word[..], [0, 0x00, 0x01]);
}

#[test]
fn test_to_i64() {
    let mut word = Word::<6, false>::new();
    word.set_all([0, 1, 2, 3, 4, 5]);
    let (value, overflow) = word.to_i64();
    assert_eq!(overflow, false);
    assert_eq!(value, 0x0102030405);

    let mut word_big = Word::<10, false>::new();
    word_big.set_all([0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
    let (value, overflow) = word_big.to_i64();
    assert_eq!(overflow, true);
    assert_eq!(value, 0x0203040506070809);

    let mut word_neg = Word::<6, false>::new();
    word_neg.set_all([1, 1, 2, 3, 4, 5]);
    let (value, overflow) = word_neg.to_i64();
    assert_eq!(overflow, false);
    assert_eq!(value, -0x0102030405);

    let mut word_big_neg = Word::<10, false>::new();
    word_big_neg.set_all([1, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF]);
    let (value, overflow) = word_big_neg.to_i64();
    assert_eq!(overflow, true);
    assert_eq!(value, 1);
}

#[test]
fn test_to_i64_ranged() {
    let mut word = Word::<6, false>::new();
    word.set_all([0, 1, 2, 3, 4, 5]);
    let (value, overflow) = word.to_i64_ranged(0..=5);
    assert_eq!(overflow, false);
    assert_eq!(value, 0x0102030405);

    let mut word_big = Word::<10, false>::new();
    word_big.set_all([0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
    let (value, overflow) = word_big.to_i64_ranged(0..=9);
    assert_eq!(overflow, true);
    assert_eq!(value, 0x0203040506070809);

    let mut word_neg = Word::<6, false>::new();
    word_neg.set_all([1, 1, 2, 3, 4, 5]);
    let (value, overflow) = word_neg.to_i64_ranged(1..=5);
    assert_eq!(overflow, false);
    assert_eq!(value, 0x0102030405);

    let mut word_big_neg = Word::<10, false>::new();
    word_big_neg.set_all([1, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF]);
    let (value, overflow) = word_big_neg.to_i64_ranged(1..=1);
    assert_eq!(overflow, false);
    assert_eq!(value, 0xFF);
}

#[test]
fn test_toggle_sign() {
    let mut word = Word::<6, false>::new();
    word[0] = 0;

    word.flip_sign();
    assert_eq!(word[0], 1);
    word.flip_sign();
    assert_eq!(word[0], 0);

    let mut word_positive = Word::<6, true>::new();
    word_positive.flip_sign();
    assert_eq!(word_positive[0], 0);
    word_positive.flip_sign();
    assert_eq!(word_positive[0], 0);
}
