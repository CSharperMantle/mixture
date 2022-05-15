use mixture::sim::mix::instr::*;
use mixture::sim::mix::mix_machine::*;

#[test]
fn test_reset() {
    let mut mix = MixMachine::new();

    mix.halted = true;
    mix.pc = 123;
    mix.toggle_overflow = true;

    mix.reset();

    assert_eq!(mix.halted, false);
    assert_eq!(mix.pc, 0);
    assert_eq!(mix.toggle_overflow, false);
}

#[test]
fn test_simple_load() {
    let mut mix = MixMachine::new();

    // For test instruction sequence, see D. E. Knuth,
    // 'The Art of Computer Programming', Volume 1, pp. 129.
    mix.mem[0] = Instruction::new(2000, 5, 0, Opcode::LdA).try_into().unwrap();
    mix.mem[1] = Instruction::new(2000, 13, 0, Opcode::LdA).try_into().unwrap();
    mix.mem[2] = Instruction::new(2000, 29, 0, Opcode::LdA).try_into().unwrap();
    mix.mem[3] = Instruction::new(2000, 3, 0, Opcode::LdA).try_into().unwrap();
    mix.mem[4] = Instruction::new(2000, 36, 0, Opcode::LdA).try_into().unwrap();
    mix.mem[5] = Instruction::new(2000, 0, 0, Opcode::LdA).try_into().unwrap();
    mix.mem[2000].set(0..=5, &[0, 0, 80, 3, 5, 4]).unwrap();

    mix.reset();
    mix.step();
    assert_eq!(mix.halted, false);
    assert_eq!(mix.toggle_overflow, false);
    assert_eq!(mix.r_a[0..=5], [0, 0, 80, 3, 5, 4]);

    mix.step();
    assert_eq!(mix.halted, false);
    assert_eq!(mix.toggle_overflow, false);
    assert_eq!(mix.r_a[0..=5], [1, 0, 80, 3, 5, 4]);

    mix.step();
    assert_eq!(mix.halted, false);
    assert_eq!(mix.toggle_overflow, false);
    assert_eq!(mix.r_a[0..=5], [1, 0, 0, 3, 5, 4]);

    mix.step();
    assert_eq!(mix.halted, false);
    assert_eq!(mix.toggle_overflow, false);
    assert_eq!(mix.r_a[0..=5], [0, 0, 0, 0, 80, 3]);

    mix.step();
    assert_eq!(mix.halted, false);
    assert_eq!(mix.toggle_overflow, false);
    assert_eq!(mix.r_a[0..=5], [1, 0, 0, 0, 0, 5]);

    mix.step();
    assert_eq!(mix.halted, false);
    assert_eq!(mix.toggle_overflow, false);
    assert_eq!(mix.r_a[0..=5], [0, 0, 0, 0, 0, 0]);
}

#[test]
fn test_simple_load_neg() {
    let mut mix = MixMachine::new();

    // For test instruction sequence, see D. E. Knuth,
    // 'The Art of Computer Programming', Volume 1, pp. 129.
    mix.mem[0] = Instruction::new(2000, 5, 0, Opcode::LdAN).try_into().unwrap();
    mix.mem[1] = Instruction::new(2000, 13, 0, Opcode::LdAN).try_into().unwrap();
    mix.mem[2] = Instruction::new(2000, 29, 0, Opcode::LdAN).try_into().unwrap();
    mix.mem[3] = Instruction::new(2000, 3, 0, Opcode::LdAN).try_into().unwrap();
    mix.mem[4] = Instruction::new(2000, 36, 0, Opcode::LdAN).try_into().unwrap();
    mix.mem[5] = Instruction::new(2000, 0, 0, Opcode::LdAN).try_into().unwrap();
    mix.mem[2000].set(0..=5, &[0, 0, 80, 3, 5, 4]).unwrap();

    mix.reset();
    mix.step();
    assert_eq!(mix.halted, false);
    assert_eq!(mix.toggle_overflow, false);
    assert_eq!(mix.r_a[0..=5], [1, 0, 80, 3, 5, 4]);

    mix.step();
    assert_eq!(mix.halted, false);
    assert_eq!(mix.toggle_overflow, false);
    assert_eq!(mix.r_a[0..=5], [0, 0, 80, 3, 5, 4]);

    mix.step();
    assert_eq!(mix.halted, false);
    assert_eq!(mix.toggle_overflow, false);
    assert_eq!(mix.r_a[0..=5], [0, 0, 0, 3, 5, 4]);

    mix.step();
    assert_eq!(mix.halted, false);
    assert_eq!(mix.toggle_overflow, false);
    assert_eq!(mix.r_a[0..=5], [1, 0, 0, 0, 80, 3]);

    mix.step();
    assert_eq!(mix.halted, false);
    assert_eq!(mix.toggle_overflow, false);
    assert_eq!(mix.r_a[0..=5], [0, 0, 0, 0, 0, 5]);

    mix.step();
    assert_eq!(mix.halted, false);
    assert_eq!(mix.toggle_overflow, false);
    assert_eq!(mix.r_a[0..=5], [1, 0, 0, 0, 0, 0]);
}

// #[test]
// fn test_simple_arith_add() {
//     let mut mix = MixMachine::new();

//     mix.r_a.set(0..=5, &[1, 0, 1, 2, 3, 4]).unwrap();

//     mix.mem[0] = Instruction::new(1, 5, 0, Opcode::Add).try_into().unwrap();
//     mix.mem[1].set(0..=5, &[1, 1, 1, 1, 1, 1]).unwrap();

//     mix.reset();
//     mix.step();

//     assert_eq!(mix.halted, false);
//     assert_eq!(mix.toggle_overflow, false);
//     assert_eq!(mix.r_a[0..=5], [1, 1, 2, 3, 4, 5]);
// }

// #[test]
// fn test_simple_arith_sub() {
//     let mut mix = MixMachine::new();

//     mix.r_a.set(0..=5, &[1, 0, 1, 2, 3, 4]).unwrap();

//     mix.mem[0] = Instruction::new(1, 5, 0, Opcode::Sub).try_into().unwrap();
//     mix.mem[1].set(0..=5, &[1, 0, 1, 1, 1, 1]).unwrap();

//     mix.reset();
//     mix.step();

//     assert_eq!(mix.halted, false);
//     assert_eq!(mix.toggle_overflow, false);
//     assert_eq!(mix.r_a[0..=5], [1, 0, 0, 1, 2, 3]);
// }
