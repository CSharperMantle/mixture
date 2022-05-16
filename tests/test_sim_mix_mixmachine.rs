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
fn test_simple_load_6b() {
    let mut mix = MixMachine::new();

    // For test instruction sequence, see D. E. Knuth,
    // 'The Art of Computer Programming', Volume 1, pp. 129.
    mix.mem[0] = Instruction::new(2000, 5, 0, Opcode::LdA)
        .try_into()
        .unwrap();
    mix.mem[1] = Instruction::new(2000, 13, 0, Opcode::LdA)
        .try_into()
        .unwrap();
    mix.mem[2] = Instruction::new(2000, 29, 0, Opcode::LdA)
        .try_into()
        .unwrap();
    mix.mem[3] = Instruction::new(2000, 3, 0, Opcode::LdA)
        .try_into()
        .unwrap();
    mix.mem[4] = Instruction::new(2000, 36, 0, Opcode::LdA)
        .try_into()
        .unwrap();
    mix.mem[5] = Instruction::new(2000, 0, 0, Opcode::LdA)
        .try_into()
        .unwrap();
    mix.mem[2000].set(0..=5, &[0, 0, 80, 3, 5, 4]).unwrap();

    mix.reset();
    mix.step().unwrap();
    assert_eq!(mix.halted, false);
    assert_eq!(mix.toggle_overflow, false);
    assert_eq!(mix.r_a[0..=5], [0, 0, 80, 3, 5, 4]);

    mix.step().unwrap();
    assert_eq!(mix.halted, false);
    assert_eq!(mix.toggle_overflow, false);
    assert_eq!(mix.r_a[0..=5], [1, 0, 80, 3, 5, 4]);

    mix.step().unwrap();
    assert_eq!(mix.halted, false);
    assert_eq!(mix.toggle_overflow, false);
    assert_eq!(mix.r_a[0..=5], [1, 0, 0, 3, 5, 4]);

    mix.step().unwrap();
    assert_eq!(mix.halted, false);
    assert_eq!(mix.toggle_overflow, false);
    assert_eq!(mix.r_a[0..=5], [0, 0, 0, 0, 80, 3]);

    mix.step().unwrap();
    assert_eq!(mix.halted, false);
    assert_eq!(mix.toggle_overflow, false);
    assert_eq!(mix.r_a[0..=5], [1, 0, 0, 0, 0, 5]);

    mix.step().unwrap();
    assert_eq!(mix.halted, false);
    assert_eq!(mix.toggle_overflow, false);
    assert_eq!(mix.r_a[0..=5], [0, 0, 0, 0, 0, 0]);
}

#[test]
fn test_simple_load_neg_6b() {
    let mut mix = MixMachine::new();

    mix.mem[0] = Instruction::new(2000, 5, 0, Opcode::LdAN)
        .try_into()
        .unwrap();
    mix.mem[1] = Instruction::new(2000, 13, 0, Opcode::LdAN)
        .try_into()
        .unwrap();
    mix.mem[2] = Instruction::new(2000, 29, 0, Opcode::LdAN)
        .try_into()
        .unwrap();
    mix.mem[3] = Instruction::new(2000, 3, 0, Opcode::LdAN)
        .try_into()
        .unwrap();
    mix.mem[4] = Instruction::new(2000, 36, 0, Opcode::LdAN)
        .try_into()
        .unwrap();
    mix.mem[5] = Instruction::new(2000, 0, 0, Opcode::LdAN)
        .try_into()
        .unwrap();
    mix.mem[2000].set(0..=5, &[0, 0, 80, 3, 5, 4]).unwrap();

    mix.reset();
    mix.step().unwrap();
    assert_eq!(mix.halted, false);
    assert_eq!(mix.toggle_overflow, false);
    assert_eq!(mix.r_a[0..=5], [1, 0, 80, 3, 5, 4]);

    mix.step().unwrap();
    assert_eq!(mix.halted, false);
    assert_eq!(mix.toggle_overflow, false);
    assert_eq!(mix.r_a[0..=5], [0, 0, 80, 3, 5, 4]);

    mix.step().unwrap();
    assert_eq!(mix.halted, false);
    assert_eq!(mix.toggle_overflow, false);
    assert_eq!(mix.r_a[0..=5], [0, 0, 0, 3, 5, 4]);

    mix.step().unwrap();
    assert_eq!(mix.halted, false);
    assert_eq!(mix.toggle_overflow, false);
    assert_eq!(mix.r_a[0..=5], [1, 0, 0, 0, 80, 3]);

    mix.step().unwrap();
    assert_eq!(mix.halted, false);
    assert_eq!(mix.toggle_overflow, false);
    assert_eq!(mix.r_a[0..=5], [0, 0, 0, 0, 0, 5]);

    mix.step().unwrap();
    assert_eq!(mix.halted, false);
    assert_eq!(mix.toggle_overflow, false);
    assert_eq!(mix.r_a[0..=5], [1, 0, 0, 0, 0, 0]);
}

#[test]
fn test_indexed_load_6b() {
    let mut mix = MixMachine::new();

    mix.mem[0] = Instruction::new(1000, 5, 1, Opcode::LdA)
        .try_into()
        .unwrap();
    mix.mem[1] = Instruction::new(1000, 13, 1, Opcode::LdA)
        .try_into()
        .unwrap();
    mix.mem[2] = Instruction::new(1000, 29, 1, Opcode::LdA)
        .try_into()
        .unwrap();
    mix.mem[3] = Instruction::new(3000, 3, 2, Opcode::LdA)
        .try_into()
        .unwrap();
    mix.mem[4] = Instruction::new(3000, 36, 2, Opcode::LdA)
        .try_into()
        .unwrap();
    mix.mem[5] = Instruction::new(3000, 0, 2, Opcode::LdA)
        .try_into()
        .unwrap();
    mix.mem[2000].set(0..=5, &[0, 0, 80, 3, 5, 4]).unwrap();
    mix.r_in[0].set(0..=2, &[1, 0x03, 0xE8]).unwrap();
    mix.r_in[1].set(0..=2, &[0, 0x03, 0xE8]).unwrap();

    mix.reset();
    mix.step().unwrap();
    assert_eq!(mix.halted, false);
    assert_eq!(mix.toggle_overflow, false);
    assert_eq!(mix.r_a[0..=5], [0, 0, 80, 3, 5, 4]);

    mix.step().unwrap();
    assert_eq!(mix.halted, false);
    assert_eq!(mix.toggle_overflow, false);
    assert_eq!(mix.r_a[0..=5], [1, 0, 80, 3, 5, 4]);

    mix.step().unwrap();
    assert_eq!(mix.halted, false);
    assert_eq!(mix.toggle_overflow, false);
    assert_eq!(mix.r_a[0..=5], [1, 0, 0, 3, 5, 4]);

    mix.step().unwrap();
    assert_eq!(mix.halted, false);
    assert_eq!(mix.toggle_overflow, false);
    assert_eq!(mix.r_a[0..=5], [0, 0, 0, 0, 80, 3]);

    mix.step().unwrap();
    assert_eq!(mix.halted, false);
    assert_eq!(mix.toggle_overflow, false);
    assert_eq!(mix.r_a[0..=5], [1, 0, 0, 0, 0, 5]);

    mix.step().unwrap();
    assert_eq!(mix.halted, false);
    assert_eq!(mix.toggle_overflow, false);
    assert_eq!(mix.r_a[0..=5], [0, 0, 0, 0, 0, 0]);
}

#[test]
fn test_simple_load_3b() {
    let mut mix = MixMachine::new();

    mix.mem[0] = Instruction::new(2000, 5, 0, Opcode::Ld1)
        .try_into()
        .unwrap();
    mix.mem[1] = Instruction::new(2000, 13, 0, Opcode::Ld1)
        .try_into()
        .unwrap();
    mix.mem[2] = Instruction::new(2000, 29, 0, Opcode::Ld1)
        .try_into()
        .unwrap();
    mix.mem[3] = Instruction::new(2000, 3, 0, Opcode::Ld1)
        .try_into()
        .unwrap();
    mix.mem[4] = Instruction::new(2000, 36, 0, Opcode::Ld1)
        .try_into()
        .unwrap();
    mix.mem[5] = Instruction::new(2000, 0, 0, Opcode::Ld1)
        .try_into()
        .unwrap();
    mix.mem[2000].set(0..=5, &[0, 0, 80, 3, 5, 4]).unwrap();

    mix.reset();
    mix.step().unwrap();
    assert_eq!(mix.halted, false);
    assert_eq!(mix.toggle_overflow, false);
    assert_eq!(mix.r_in[0][0..=2], [0, 5, 4]);

    mix.step().unwrap();
    assert_eq!(mix.halted, false);
    assert_eq!(mix.toggle_overflow, false);
    assert_eq!(mix.r_in[0][0..=2], [1, 5, 4]);

    mix.step().unwrap();
    assert_eq!(mix.halted, false);
    assert_eq!(mix.toggle_overflow, false);
    assert_eq!(mix.r_in[0][0..=2], [1, 5, 4]);

    mix.step().unwrap();
    assert_eq!(mix.halted, false);
    assert_eq!(mix.toggle_overflow, false);
    assert_eq!(mix.r_in[0][0..=2], [0, 80, 3]);

    mix.step().unwrap();
    assert_eq!(mix.halted, false);
    assert_eq!(mix.toggle_overflow, false);
    assert_eq!(mix.r_in[0][0..=2], [1, 0, 5]);

    mix.step().unwrap();
    assert_eq!(mix.halted, false);
    assert_eq!(mix.toggle_overflow, false);
    assert_eq!(mix.r_in[0][0..=2], [0, 0, 0]);
}

#[test]
fn test_simple_load_neg_3b() {
    let mut mix = MixMachine::new();

    mix.mem[0] = Instruction::new(2000, 5, 0, Opcode::Ld1N)
        .try_into()
        .unwrap();
    mix.mem[1] = Instruction::new(2000, 13, 0, Opcode::Ld1N)
        .try_into()
        .unwrap();
    mix.mem[2] = Instruction::new(2000, 29, 0, Opcode::Ld1N)
        .try_into()
        .unwrap();
    mix.mem[3] = Instruction::new(2000, 3, 0, Opcode::Ld1N)
        .try_into()
        .unwrap();
    mix.mem[4] = Instruction::new(2000, 36, 0, Opcode::Ld1N)
        .try_into()
        .unwrap();
    mix.mem[5] = Instruction::new(2000, 0, 0, Opcode::Ld1N)
        .try_into()
        .unwrap();
    mix.mem[2000].set(0..=5, &[0, 0, 80, 3, 5, 4]).unwrap();

    mix.reset();
    mix.step().unwrap();
    assert_eq!(mix.halted, false);
    assert_eq!(mix.toggle_overflow, false);
    assert_eq!(mix.r_in[0][0..=2], [1, 5, 4]);

    mix.step().unwrap();
    assert_eq!(mix.halted, false);
    assert_eq!(mix.toggle_overflow, false);
    assert_eq!(mix.r_in[0][0..=2], [0, 5, 4]);

    mix.step().unwrap();
    assert_eq!(mix.halted, false);
    assert_eq!(mix.toggle_overflow, false);
    assert_eq!(mix.r_in[0][0..=2], [0, 5, 4]);

    mix.step().unwrap();
    assert_eq!(mix.halted, false);
    assert_eq!(mix.toggle_overflow, false);
    assert_eq!(mix.r_in[0][0..=2], [1, 80, 3]);

    mix.step().unwrap();
    assert_eq!(mix.halted, false);
    assert_eq!(mix.toggle_overflow, false);
    assert_eq!(mix.r_in[0][0..=2], [0, 0, 5]);

    mix.step().unwrap();
    assert_eq!(mix.halted, false);
    assert_eq!(mix.toggle_overflow, false);
    assert_eq!(mix.r_in[0][0..=2], [1, 0, 0]);
}

#[test]
fn test_simple_jmp() {
    let mut mix = MixMachine::new();

    mix.mem[0] = Instruction::new(1000, 0, 0, Opcode::Jmp)
        .try_into()
        .unwrap();
    mix.mem[1000] = Instruction::new(2000, 2, 0, Opcode::Jmp)
        .try_into()
        .unwrap();
    mix.mem[1001] = Instruction::new(0, 1, 0, Opcode::Jmp)
        .try_into()
        .unwrap();

    mix.reset();

    mix.step().unwrap();
    assert_eq!(mix.halted, false);
    assert_eq!(mix.pc, 1000);
    assert_eq!(mix.r_j[0..=2], [1, 0, 1]);

    mix.step().unwrap();
    assert_eq!(mix.halted, false);
    assert_eq!(mix.pc, 1001);
    assert_eq!(mix.r_j[0..=2], [1, 0, 1]);

    mix.step().unwrap();
    assert_eq!(mix.halted, false);
    assert_eq!(mix.pc, 0);
    assert_eq!(mix.r_j[0..=2], [1, 0, 1]);
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
