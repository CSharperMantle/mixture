use crate::sim::instr::*;
use crate::sim::io::*;
use crate::sim::mix_machine::*;

#[test]
fn test_illegal_instruction() {
    let mut mix = MixMachine::new();
    mix.reset();

    mix.mem[0]
        .set(0..=5, &[0, 255, 255, 255, 255, 255])
        .unwrap();

    mix.restart();

    let err = mix.step().expect_err("Expect error");
    assert_eq!(err, ErrorCode::IllegalInstruction);
    assert_eq!(mix.halted, true);
}

#[test]
fn test_halted() {
    let mut mix = MixMachine::new();
    mix.reset();

    let err = mix.step().expect_err("Expect error");
    assert_eq!(err, ErrorCode::Halted);
    assert_eq!(mix.halted, true);
}

#[test]
fn test_invalid_addr() {
    let mut mix = MixMachine::new();
    mix.reset();

    mix.mem[0] = Instruction::new(-1, 0, 0, Opcode::Shift)
        .try_into()
        .unwrap();

    mix.restart();

    let err = mix.step().expect_err("Expect error");
    assert_eq!(err, ErrorCode::InvalidAddress);
    assert_eq!(mix.halted, true);
}

#[test]
fn test_invalid_field() {
    let mut mix = MixMachine::new();
    mix.reset();

    mix.mem[0] = Instruction::new(0, 255, 0, Opcode::Shift)
        .try_into()
        .unwrap();

    mix.restart();

    let err = mix.step().expect_err("Expect error");
    assert_eq!(err, ErrorCode::InvalidField);
    assert_eq!(mix.halted, true);
}

#[test]
fn test_invalid_index() {
    let mut mix = MixMachine::new();
    mix.reset();

    mix.mem[0] = Instruction::new(0, 5, 255, Opcode::LdA).try_into().unwrap();

    mix.restart();

    let err = mix.step().expect_err("Expect error");
    assert_eq!(err, ErrorCode::InvalidIndex);
    assert_eq!(mix.halted, true);
}

#[test]
fn test_unknown_device() {
    let mut mix = MixMachine::new();
    mix.reset();

    mix.mem[0] = Instruction::new(1000, 0, 0, Opcode::In).try_into().unwrap();

    mix.restart();

    let err = mix.step().expect_err("Expect error");
    assert_eq!(err, ErrorCode::UnknownDevice);
    assert_eq!(mix.halted, true);
}

#[test]
fn test_io_error() {
    let dev_err = IODevice {
        in_handler: |_, _| Err(()),
        out_handler: |_, _| Err(()),
        control_handler: |_| Err(()),
        is_ready_handler: || Err(()),
        is_busy_handler: || Err(()),
    };

    let mut mix = MixMachine::new();
    mix.reset();

    mix.io_devices[0] = Some(dev_err);

    mix.mem[0] = Instruction::new(1000, 0, 0, Opcode::In).try_into().unwrap();

    mix.restart();

    let err = mix.step().expect_err("Expect error");
    assert_eq!(err, ErrorCode::IOError);
    assert_eq!(mix.halted, true);
}
