use crate::*;

#[test]
fn test_illegal_instruction() {
    let mut mix = MixVM::new();
    mix.reset();

    mix.mem[0].set_all([0, 255, 255, 255, 255, 255]);

    mix.restart();

    let err = mix.step().expect_err("Expect error");
    assert_eq!(err, ErrorCode::IllegalInstruction);
    assert_eq!(mix.halted, true);
}

#[test]
fn test_halted() {
    let mut mix = MixVM::new();
    mix.reset();

    let err = mix.step().expect_err("Expect error");
    assert_eq!(err, ErrorCode::Halted);
    assert_eq!(mix.halted, true);
}

#[test]
fn test_invalid_addr() {
    let mut mix = MixVM::new();
    mix.reset();

    mix.mem[0] = Instruction::new(-1, 0, 0, Opcode::Shift).into();

    mix.restart();

    let err = mix.step().expect_err("Expect error");
    assert_eq!(err, ErrorCode::InvalidAddress);
    assert_eq!(mix.halted, true);
}

#[test]
fn test_invalid_field() {
    let mut mix = MixVM::new();
    mix.reset();

    mix.mem[0] = Instruction::new(0, 255, 0, Opcode::Shift).into();

    mix.restart();

    let err = mix.step().expect_err("Expect error");
    assert_eq!(err, ErrorCode::InvalidField);
    assert_eq!(mix.halted, true);
}

#[test]
fn test_invalid_index() {
    let mut mix = MixVM::new();
    mix.reset();

    mix.mem[0] = Instruction::new(0, 5, 255, Opcode::LdA).into();

    mix.restart();

    let err = mix.step().expect_err("Expect error");
    assert_eq!(err, ErrorCode::InvalidIndex);
    assert_eq!(mix.halted, true);
}
