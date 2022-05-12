use mixture::sim::mix::instr::*;
use mixture::sim::mix::mem::*;

#[test]
fn test_instruction_from() {
    let mut word = Word::<6, false>::new();
    word.set(0..=5, &[0, 0xD0, 0x07, 0x02, 0x03, 0x08]).unwrap();

    let instr = Instruction::try_from(word).unwrap();
    assert_eq!(instr.opcode, Opcode::Lda);
    assert_eq!(instr.field, 0x03);
    assert_eq!(instr.index, 0x02);
    assert_eq!(instr.addr, 2000);
}

#[test]
fn test_instruction_into() {
    let instr = Instruction::new(2000, 0x03, 0x02, Opcode::Lda);

    let word: Word<6, false> = instr.try_into().unwrap();
    assert_eq!(word[0..=5], [0, 0xD0, 0x07, 0x02, 0x03, 0x08]);
}
