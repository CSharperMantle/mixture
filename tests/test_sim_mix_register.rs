use mixture::sim::mix::register::*;

#[test]
fn test_general_register_mutation() {
    let mut reg = GeneralRegister::new();

    assert_eq!(reg[0..6], [0; 6]);
    reg.set(0, 5, &[1, 2, 3, 4, 5, 6]);
    assert_eq!(reg[0..6], [1, 2, 3, 4, 5, 6]);
    reg.set(0, 2, &[1, 8, 9]);
    assert_eq!(reg[0..6], [1, 8, 9, 4, 5, 6]);
    reg.set(2, 5, &[1, 2, 3, 4]);
    assert_eq!(reg[0..6], [1, 8, 1, 2, 3, 4]);
}

#[test]
fn test_index_register_mutation() {
    let mut reg = IndexRegister::new();

    assert_eq!(reg[0..3], [0; 3]);
    reg.set(0, 2, &[1, 2, 3]);
    assert_eq!(reg[0..3], [1, 2, 3]);
    reg.set(0, 1, &[0, 8]);
    assert_eq!(reg[0..3], [0, 8, 3]);
}

#[test]
fn test_jump_register_mutation() {
    let mut reg = JumpRegister::new();

    assert_eq!(reg[0..3], [1, 0, 0]);
    reg.set(0, 2, &[1, 2, 3]);
    assert_eq!(reg[0..3], [1, 2, 3]);
    reg.set(0, 2, &[0, 0, 0]);
    assert_eq!(reg[0..3], [1, 0, 0]);
}

#[test]
fn test_register_interchange() {
    let mut reg1 = GeneralRegister::new();
    let mut reg2 = GeneralRegister::new();
    reg1.set(0, 5, &[1, 2, 3, 4, 5, 6]);
    reg2.set(0, 5, &[1, 8, 9, 10, 11, 12]);

    reg1.set(3, 5, &reg2[3..6]);
    assert_eq!(reg1[0..6], [1, 2, 3, 10, 11, 12]);
    assert_eq!(reg2[0..6], [1, 8, 9, 10, 11, 12]);
    reg2.set(0, 2, &reg1[0..3]);
    assert_eq!(reg1[0..6], [1, 2, 3, 10, 11, 12]);
    assert_eq!(reg2[0..6], [1, 2, 3, 10, 11, 12]);
}
