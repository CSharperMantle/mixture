use crate::sim::instr::*;
use crate::sim::word::*;

#[test]
fn test_clone() {
    let instr = Instruction::new(0, 1, 2, Opcode::Div);
    let instr2 = instr.clone();

    assert_eq!(instr.addr, instr2.addr);
    assert_eq!(instr.field, instr2.field);
    assert_eq!(instr.index, instr2.index);
    assert_eq!(instr.opcode, instr2.opcode);
}

#[test]
fn test_from_word() {
    let mut word = Word::<6, false>::new();
    word.set_all([0, 0x07, 0xD0, 0x02, 0x03, 0x08]);

    let instr = Instruction::try_from(word).unwrap();
    assert_eq!(instr.opcode, Opcode::LdA);
    assert_eq!(instr.field, 3);
    assert_eq!(instr.index, 2);
    assert_eq!(instr.addr, 2000);
}

#[test]
fn test_into_word() {
    let instr = Instruction::new(2000, 0x03, 0x02, Opcode::LdA);

    let word: FullWord = instr.try_into().unwrap();
    assert_eq!(word[..], [0, 0x07, 0xD0, 0x02, 0x03, 0x08]);
}

#[test]
fn test_field_into_range_inclusive() {
    assert_eq!(1.to_range_inclusive(), 0..=1);
    assert_eq!(13.to_range_inclusive(), 1..=5);
}

#[test]
fn test_field_into_range_inclusive_signless() {
    assert_eq!(1.to_range_inclusive_signless(), (1..=1, true));
    assert_eq!(13.to_range_inclusive_signless(), (1..=5, false));
}
