use crate::*;

#[test]
fn test_binarith_logics() {
    let mut mix = MixVM::new();
    mix.reset();

    mix.mem[0] = Instruction::new(0, 9, 0, Opcode::Special).into();
    mix.mem[1] = Instruction::new(0, 9, 0, Opcode::Special).into();
    mix.mem[2] = Instruction::new(1000, 10, 0, Opcode::Special).into();
    mix.mem[3] = Instruction::new(1000, 11, 0, Opcode::Special).into();
    mix.mem[4] = Instruction::new(1000, 12, 0, Opcode::Special).into();
    mix.mem[1000].set_all([
        1, 0b10101010, 0b11001100, 0b11110000, 0b00001111, 0b00000000,
    ]);
    mix.r_a.set_all([0, 0b11111111, 0, 0, 0, 0b00000110]);

    mix.restart();
    mix.step().unwrap();
    assert_eq!(mix.halted, false);
    assert_eq!(
        mix.r_a[..],
        [1, 0, 0b11111111, 0b11111111, 0b11111111, 0b11111001]
    );

    mix.step().unwrap();
    assert_eq!(mix.halted, false);
    assert_eq!(mix.r_a[..], [0, 0b11111111, 0, 0, 0, 0b00000110]);

    mix.step().unwrap();
    assert_eq!(mix.halted, false);
    assert_eq!(mix.r_a[..], [0, 0b10101010, 0, 0, 0, 0b00000000]);

    mix.step().unwrap();
    assert_eq!(mix.halted, false);
    assert_eq!(
        mix.r_a[..],
        [1, 0b10101010, 0b11001100, 0b11110000, 0b00001111, 0b00000000]
    );

    mix.step().unwrap();
    assert_eq!(mix.halted, false);
    assert_eq!(mix.r_a[..], [0, 0, 0, 0, 0, 0]);
}
