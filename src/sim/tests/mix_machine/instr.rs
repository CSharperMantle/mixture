use crate::sim::instr::*;
use crate::sim::io::*;
use crate::sim::mem::*;
use crate::sim::mix_machine::*;

#[test]
fn test_nop() {
    let mut mix = MixMachine::new();
    mix.reset();

    mix.mem[0] = Instruction::new(0, 0, 0, Opcode::Nop).try_into().unwrap();

    mix.restart();

    mix.step().unwrap();
    assert_eq!(mix.halted, false);
    assert_eq!(mix.pc, 1);
}

#[test]
fn test_load_6b() {
    let mut mix = MixMachine::new();
    mix.reset();

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
    mix.mem[2000].set(0..=5, &[1, 0, 80, 3, 5, 4]).unwrap();

    mix.restart();

    mix.step().unwrap();
    assert_eq!(mix.halted, false);
    assert_eq!(mix.overflow, false);
    assert_eq!(mix.r_a[0..=5], [1, 0, 80, 3, 5, 4]);

    mix.step().unwrap();
    assert_eq!(mix.halted, false);
    assert_eq!(mix.overflow, false);
    assert_eq!(mix.r_a[0..=5], [0, 0, 80, 3, 5, 4]);

    mix.step().unwrap();
    assert_eq!(mix.halted, false);
    assert_eq!(mix.overflow, false);
    assert_eq!(mix.r_a[0..=5], [0, 0, 0, 3, 5, 4]);

    mix.step().unwrap();
    assert_eq!(mix.halted, false);
    assert_eq!(mix.overflow, false);
    assert_eq!(mix.r_a[0..=5], [1, 0, 0, 0, 80, 3]);

    mix.step().unwrap();
    assert_eq!(mix.halted, false);
    assert_eq!(mix.overflow, false);
    assert_eq!(mix.r_a[0..=5], [0, 0, 0, 0, 0, 5]);

    mix.step().unwrap();
    assert_eq!(mix.halted, false);
    assert_eq!(mix.overflow, false);
    assert_eq!(mix.r_a[0..=5], [1, 0, 0, 0, 0, 0]);
}

#[test]
fn test_load_neg_6b() {
    let mut mix = MixMachine::new();
    mix.reset();

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

    mix.restart();

    mix.step().unwrap();
    assert_eq!(mix.halted, false);
    assert_eq!(mix.overflow, false);
    assert_eq!(mix.r_a[0..=5], [1, 0, 80, 3, 5, 4]);

    mix.step().unwrap();
    assert_eq!(mix.halted, false);
    assert_eq!(mix.overflow, false);
    assert_eq!(mix.r_a[0..=5], [0, 0, 80, 3, 5, 4]);

    mix.step().unwrap();
    assert_eq!(mix.halted, false);
    assert_eq!(mix.overflow, false);
    assert_eq!(mix.r_a[0..=5], [0, 0, 0, 3, 5, 4]);

    mix.step().unwrap();
    assert_eq!(mix.halted, false);
    assert_eq!(mix.overflow, false);
    assert_eq!(mix.r_a[0..=5], [1, 0, 0, 0, 80, 3]);

    mix.step().unwrap();
    assert_eq!(mix.halted, false);
    assert_eq!(mix.overflow, false);
    assert_eq!(mix.r_a[0..=5], [0, 0, 0, 0, 0, 5]);

    mix.step().unwrap();
    assert_eq!(mix.halted, false);
    assert_eq!(mix.overflow, false);
    assert_eq!(mix.r_a[0..=5], [1, 0, 0, 0, 0, 0]);
}

#[test]
fn test_indexed_load_6b() {
    let mut mix = MixMachine::new();
    mix.reset();

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
    mix.mem[2000].set(0..=5, &[1, 0, 80, 3, 5, 4]).unwrap();
    mix.r_in[1].set(0..=2, &[0, 0x03, 0xE8]).unwrap();
    mix.r_in[2].set(0..=2, &[1, 0x03, 0xE8]).unwrap();

    mix.restart();

    mix.step().unwrap();
    assert_eq!(mix.halted, false);
    assert_eq!(mix.overflow, false);
    assert_eq!(mix.r_a[0..=5], [1, 0, 80, 3, 5, 4]);

    mix.step().unwrap();
    assert_eq!(mix.halted, false);
    assert_eq!(mix.overflow, false);
    assert_eq!(mix.r_a[0..=5], [0, 0, 80, 3, 5, 4]);

    mix.step().unwrap();
    assert_eq!(mix.halted, false);
    assert_eq!(mix.overflow, false);
    assert_eq!(mix.r_a[0..=5], [0, 0, 0, 3, 5, 4]);

    mix.step().unwrap();
    assert_eq!(mix.halted, false);
    assert_eq!(mix.overflow, false);
    assert_eq!(mix.r_a[0..=5], [1, 0, 0, 0, 80, 3]);

    mix.step().unwrap();
    assert_eq!(mix.halted, false);
    assert_eq!(mix.overflow, false);
    assert_eq!(mix.r_a[0..=5], [0, 0, 0, 0, 0, 5]);

    mix.step().unwrap();
    assert_eq!(mix.halted, false);
    assert_eq!(mix.overflow, false);
    assert_eq!(mix.r_a[0..=5], [1, 0, 0, 0, 0, 0]);
}

#[test]
fn test_load_3b() {
    let mut mix = MixMachine::new();
    mix.reset();

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

    mix.restart();

    mix.step().unwrap();
    assert_eq!(mix.halted, false);
    assert_eq!(mix.overflow, false);
    assert_eq!(mix.r_in[1][0..=2], [0, 5, 4]);

    mix.step().unwrap();
    assert_eq!(mix.halted, false);
    assert_eq!(mix.overflow, false);
    assert_eq!(mix.r_in[1][0..=2], [1, 5, 4]);

    mix.step().unwrap();
    assert_eq!(mix.halted, false);
    assert_eq!(mix.overflow, false);
    assert_eq!(mix.r_in[1][0..=2], [1, 5, 4]);

    mix.step().unwrap();
    assert_eq!(mix.halted, false);
    assert_eq!(mix.overflow, false);
    assert_eq!(mix.r_in[1][0..=2], [0, 80, 3]);

    mix.step().unwrap();
    assert_eq!(mix.halted, false);
    assert_eq!(mix.overflow, false);
    assert_eq!(mix.r_in[1][0..=2], [1, 0, 5]);

    mix.step().unwrap();
    assert_eq!(mix.halted, false);
    assert_eq!(mix.overflow, false);
    assert_eq!(mix.r_in[1][0..=2], [0, 0, 0]);
}

#[test]
fn test_load_neg_3b() {
    let mut mix = MixMachine::new();
    mix.reset();

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

    mix.restart();

    mix.step().unwrap();
    assert_eq!(mix.halted, false);
    assert_eq!(mix.overflow, false);
    assert_eq!(mix.r_in[1][0..=2], [1, 5, 4]);

    mix.step().unwrap();
    assert_eq!(mix.halted, false);
    assert_eq!(mix.overflow, false);
    assert_eq!(mix.r_in[1][0..=2], [0, 5, 4]);

    mix.step().unwrap();
    assert_eq!(mix.halted, false);
    assert_eq!(mix.overflow, false);
    assert_eq!(mix.r_in[1][0..=2], [0, 5, 4]);

    mix.step().unwrap();
    assert_eq!(mix.halted, false);
    assert_eq!(mix.overflow, false);
    assert_eq!(mix.r_in[1][0..=2], [1, 80, 3]);

    mix.step().unwrap();
    assert_eq!(mix.halted, false);
    assert_eq!(mix.overflow, false);
    assert_eq!(mix.r_in[1][0..=2], [0, 0, 5]);

    mix.step().unwrap();
    assert_eq!(mix.halted, false);
    assert_eq!(mix.overflow, false);
    assert_eq!(mix.r_in[1][0..=2], [1, 0, 0]);
}

#[test]
fn test_jmp() {
    let mut mix = MixMachine::new();
    mix.reset();

    mix.mem[0] = Instruction::new(1000, 0, 0, Opcode::Jmp)
        .try_into()
        .unwrap();
    mix.mem[1000] = Instruction::new(2000, 2, 0, Opcode::Jmp)
        .try_into()
        .unwrap();
    mix.mem[1001] = Instruction::new(0, 1, 0, Opcode::Jmp).try_into().unwrap();

    mix.restart();

    mix.step().unwrap();
    assert_eq!(mix.halted, false);
    assert_eq!(mix.pc, 1000);
    assert_eq!(mix.r_j[0..=2], [0, 0, 1]);

    mix.step().unwrap();
    assert_eq!(mix.halted, false);
    assert_eq!(mix.pc, 1001);
    assert_eq!(mix.r_j[0..=2], [0, 0, 1]);

    mix.step().unwrap();
    assert_eq!(mix.halted, false);
    assert_eq!(mix.pc, 0);
    assert_eq!(mix.r_j[0..=2], [0, 0, 1]);
}

#[test]
fn test_special() {
    let mut mix = MixMachine::new();
    mix.reset();

    mix.mem[0] = Instruction::new(0, 0, 0, Opcode::Special)
        .try_into()
        .unwrap();
    mix.mem[1] = Instruction::new(0, 1, 0, Opcode::Special)
        .try_into()
        .unwrap();
    mix.mem[2] = Instruction::new(0, 2, 0, Opcode::Special)
        .try_into()
        .unwrap();

    mix.r_a.set(0..=5, &[0, 0, 0, 31, 32, 39]).unwrap();
    mix.r_x.set(0..=5, &[1, 37, 57, 47, 30, 30]).unwrap();

    mix.restart();

    mix.step().unwrap();
    assert_eq!(mix.halted, false);
    assert_eq!(mix.r_a[0..=5], [0, 0, 0, 0xC6, 0x06, 0x24]);
    assert_eq!(mix.r_x[0..=5], [1, 37, 57, 47, 30, 30]);

    mix.step().unwrap();
    assert_eq!(mix.halted, false);
    assert_eq!(mix.r_a[0..=5], [0, 30, 30, 31, 32, 39]);
    assert_eq!(mix.r_x[0..=5], [1, 37, 37, 37, 30, 30]);

    mix.step().unwrap();
    assert_eq!(mix.halted, true);
}

#[test]
fn test_special_2() {
    let mut mix = MixMachine::new();
    mix.reset();

    mix.mem[0] = Instruction::new(0, 0, 0, Opcode::Special)
        .try_into()
        .unwrap();
    mix.mem[1] = Instruction::new(0, 1, 0, Opcode::Special)
        .try_into()
        .unwrap();
    mix.mem[2] = Instruction::new(0, 2, 0, Opcode::Special)
        .try_into()
        .unwrap();

    mix.r_a.set(0..=5, &[0, 39, 39, 39, 39, 39]).unwrap();
    mix.r_x.set(0..=5, &[1, 39, 39, 39, 39, 39]).unwrap();

    mix.restart();

    mix.step().unwrap();
    assert_eq!(mix.halted, false);
    assert_eq!(mix.r_a[0..=5], [0, 0x02, 0x54, 0x0B, 0xE3, 0xFF]);
    assert_eq!(mix.r_x[0..=5], [1, 39, 39, 39, 39, 39]);

    mix.step().unwrap();
    assert_eq!(mix.halted, false);
    assert_eq!(mix.r_a[0..=5], [0, 39, 39, 39, 39, 39]);
    assert_eq!(mix.r_x[0..=5], [1, 39, 39, 39, 39, 39]);

    mix.step().unwrap();
    assert_eq!(mix.halted, true);
}

#[test]
fn test_store_zero() {
    let mut mix = MixMachine::new();
    mix.reset();

    mix.mem[0] = Instruction::new(1000, 5, 0, Opcode::StZ)
        .try_into()
        .unwrap();
    mix.mem[1] = Instruction::new(1001, 13, 0, Opcode::StZ)
        .try_into()
        .unwrap();
    mix.mem[2] = Instruction::new(1002, 45, 0, Opcode::StZ)
        .try_into()
        .unwrap();

    mix.mem[1000].set(0..=5, &[0, 1, 2, 3, 4, 5]).unwrap();
    mix.mem[1001].set(0..=5, &[0, 1, 2, 3, 4, 5]).unwrap();
    mix.mem[1002].set(0..=5, &[0, 1, 2, 3, 4, 5]).unwrap();

    mix.restart();

    mix.step().unwrap();
    assert_eq!(mix.halted, false);
    assert_eq!(mix.mem[1000][0..=5], [1, 0, 0, 0, 0, 0]);

    mix.step().unwrap();
    assert_eq!(mix.halted, false);
    assert_eq!(mix.mem[1001][0..=5], [0, 0, 0, 0, 0, 0]);

    mix.step().unwrap();
    assert_eq!(mix.halted, false);
    assert_eq!(mix.mem[1002][0..=5], [0, 1, 2, 3, 4, 0]);
}

#[test]
fn test_move() {
    let mut mix = MixMachine::new();
    mix.reset();

    mix.mem[0] = Instruction::new(1000, 3, 0, Opcode::Move)
        .try_into()
        .unwrap();

    mix.r_in[1].set(1..=2, &[0x03, 0xE7]).unwrap();
    mix.mem[1000].set(0..=5, &[1, 1, 1, 1, 1, 1]).unwrap();
    mix.mem[1001].set(0..=5, &[1, 2, 2, 2, 2, 2]).unwrap();
    mix.mem[1002].set(0..=5, &[1, 3, 3, 3, 3, 3]).unwrap();

    mix.restart();

    mix.step().unwrap();
    assert_eq!(mix.halted, false);
    assert_eq!(mix.mem[999][0..=5], [1, 1, 1, 1, 1, 1]);
    assert_eq!(mix.mem[1000][0..=5], [1, 2, 2, 2, 2, 2]);
    assert_eq!(mix.mem[1001][0..=5], [1, 3, 3, 3, 3, 3]);
    assert_eq!(mix.mem[1002][0..=5], [1, 3, 3, 3, 3, 3]);
}

#[test]
fn test_store_6b() {
    let mut mix = MixMachine::new();
    mix.reset();

    mix.mem[0] = Instruction::new(2000, 5, 0, Opcode::StA)
        .try_into()
        .unwrap();
    mix.mem[1] = Instruction::new(2001, 13, 0, Opcode::StA)
        .try_into()
        .unwrap();
    mix.mem[2] = Instruction::new(2002, 45, 0, Opcode::StA)
        .try_into()
        .unwrap();
    mix.mem[3] = Instruction::new(2003, 18, 0, Opcode::StA)
        .try_into()
        .unwrap();
    mix.mem[4] = Instruction::new(2004, 19, 0, Opcode::StA)
        .try_into()
        .unwrap();
    mix.mem[5] = Instruction::new(2005, 1, 0, Opcode::StA)
        .try_into()
        .unwrap();

    mix.r_a.set(0..=5, &[1, 6, 7, 8, 9, 0]).unwrap();
    mix.mem[2000].set(0..=5, &[0, 1, 2, 3, 4, 5]).unwrap();
    mix.mem[2001].set(0..=5, &[0, 1, 2, 3, 4, 5]).unwrap();
    mix.mem[2002].set(0..=5, &[0, 1, 2, 3, 4, 5]).unwrap();
    mix.mem[2003].set(0..=5, &[0, 1, 2, 3, 4, 5]).unwrap();
    mix.mem[2004].set(0..=5, &[0, 1, 2, 3, 4, 5]).unwrap();
    mix.mem[2005].set(0..=5, &[0, 1, 2, 3, 4, 5]).unwrap();

    mix.restart();

    mix.step().unwrap();
    assert_eq!(mix.halted, false);
    assert_eq!(mix.mem[2000][0..=5], [1, 6, 7, 8, 9, 0]);

    mix.step().unwrap();
    assert_eq!(mix.halted, false);
    assert_eq!(mix.mem[2001][0..=5], [0, 6, 7, 8, 9, 0]);

    mix.step().unwrap();
    assert_eq!(mix.halted, false);
    assert_eq!(mix.mem[2002][0..=5], [0, 1, 2, 3, 4, 0]);

    mix.step().unwrap();
    assert_eq!(mix.halted, false);
    assert_eq!(mix.mem[2003][0..=5], [0, 1, 0, 3, 4, 5]);

    mix.step().unwrap();
    assert_eq!(mix.halted, false);
    assert_eq!(mix.mem[2004][0..=5], [0, 1, 9, 0, 4, 5]);

    mix.step().unwrap();
    assert_eq!(mix.halted, false);
    assert_eq!(mix.mem[2005][0..=5], [1, 0, 2, 3, 4, 5]);
}

#[test]
fn test_store_3b() {
    let mut mix = MixMachine::new();
    mix.reset();

    mix.mem[0] = Instruction::new(2000, 5, 0, Opcode::St1)
        .try_into()
        .unwrap();
    mix.mem[1] = Instruction::new(2001, 13, 0, Opcode::St1)
        .try_into()
        .unwrap();
    mix.mem[2] = Instruction::new(2002, 45, 0, Opcode::St1)
        .try_into()
        .unwrap();
    mix.mem[3] = Instruction::new(2003, 18, 0, Opcode::St1)
        .try_into()
        .unwrap();
    mix.mem[4] = Instruction::new(2004, 19, 0, Opcode::St1)
        .try_into()
        .unwrap();
    mix.mem[5] = Instruction::new(2005, 1, 0, Opcode::St1)
        .try_into()
        .unwrap();

    mix.r_in[1].set(0..=2, &[1, 6, 7]).unwrap();
    mix.mem[2000].set(0..=5, &[0, 1, 2, 3, 4, 5]).unwrap();
    mix.mem[2001].set(0..=5, &[0, 1, 2, 3, 4, 5]).unwrap();
    mix.mem[2002].set(0..=5, &[0, 1, 2, 3, 4, 5]).unwrap();
    mix.mem[2003].set(0..=5, &[0, 1, 2, 3, 4, 5]).unwrap();
    mix.mem[2004].set(0..=5, &[0, 1, 2, 3, 4, 5]).unwrap();
    mix.mem[2005].set(0..=5, &[0, 1, 2, 3, 4, 5]).unwrap();

    mix.restart();

    mix.step().unwrap();
    assert_eq!(mix.halted, false);
    assert_eq!(mix.mem[2000][0..=5], [1, 0, 0, 0, 6, 7]);

    mix.step().unwrap();
    assert_eq!(mix.halted, false);
    assert_eq!(mix.mem[2001][0..=5], [0, 0, 0, 0, 6, 7]);

    mix.step().unwrap();
    assert_eq!(mix.halted, false);
    assert_eq!(mix.mem[2002][0..=5], [0, 1, 2, 3, 4, 7]);

    mix.step().unwrap();
    assert_eq!(mix.halted, false);
    assert_eq!(mix.mem[2003][0..=5], [0, 1, 7, 3, 4, 5]);

    mix.step().unwrap();
    assert_eq!(mix.halted, false);
    assert_eq!(mix.mem[2004][0..=5], [0, 1, 6, 7, 4, 5]);

    mix.step().unwrap();
    assert_eq!(mix.halted, false);
    assert_eq!(mix.mem[2005][0..=5], [1, 7, 2, 3, 4, 5]);
}

#[test]
fn test_modify_6b() {
    let mut mix = MixMachine::new();
    mix.reset();

    mix.mem[0] = Instruction::new(1, 0, 0, Opcode::ModifyA)
        .try_into()
        .unwrap();
    mix.mem[1] = Instruction::new(2, 0, 0, Opcode::ModifyA)
        .try_into()
        .unwrap();
    mix.mem[2] = Instruction::new(2, 1, 0, Opcode::ModifyA)
        .try_into()
        .unwrap();
    mix.mem[3] = Instruction::new(1, 0, 1, Opcode::ModifyA)
        .try_into()
        .unwrap();
    mix.mem[4] = Instruction::new(1, 1, 1, Opcode::ModifyA)
        .try_into()
        .unwrap();
    mix.mem[5] = Instruction::new(1234, 2, 0, Opcode::ModifyA)
        .try_into()
        .unwrap();
    mix.mem[6] = Instruction::new(4321, 3, 0, Opcode::ModifyA)
        .try_into()
        .unwrap();

    mix.r_a.set(0..=5, &[0, 6, 7, 8, 9, 0]).unwrap();
    mix.r_in[1].set(0..=2, &[0, 0, 5]).unwrap();

    mix.restart();

    mix.step().unwrap();
    assert_eq!(mix.halted, false);
    assert_eq!(mix.r_a[0..=5], [0, 6, 7, 8, 9, 1]);

    mix.step().unwrap();
    assert_eq!(mix.halted, false);
    assert_eq!(mix.r_a[0..=5], [0, 6, 7, 8, 9, 3]);

    mix.step().unwrap();
    assert_eq!(mix.halted, false);
    assert_eq!(mix.r_a[0..=5], [0, 6, 7, 8, 9, 1]);

    mix.step().unwrap();
    assert_eq!(mix.halted, false);
    assert_eq!(mix.r_a[0..=5], [0, 6, 7, 8, 9, 7]);

    mix.step().unwrap();
    assert_eq!(mix.halted, false);
    assert_eq!(mix.r_a[0..=5], [0, 6, 7, 8, 9, 1]);

    mix.step().unwrap();
    assert_eq!(mix.halted, false);
    assert_eq!(mix.r_a[0..=5], [0, 0, 0, 0, 0x04, 0xD2]);

    mix.step().unwrap();
    assert_eq!(mix.halted, false);
    assert_eq!(mix.r_a[0..=5], [1, 0, 0, 0, 0x10, 0xE1]);
}

#[test]
fn test_modify_3b() {
    let mut mix = MixMachine::new();
    mix.reset();

    mix.mem[0] = Instruction::new(1, 0, 0, Opcode::Modify1)
        .try_into()
        .unwrap();
    mix.mem[1] = Instruction::new(2, 0, 0, Opcode::Modify1)
        .try_into()
        .unwrap();
    mix.mem[2] = Instruction::new(2, 1, 0, Opcode::Modify1)
        .try_into()
        .unwrap();
    mix.mem[3] = Instruction::new(1234, 2, 0, Opcode::Modify1)
        .try_into()
        .unwrap();
    mix.mem[4] = Instruction::new(4321, 3, 0, Opcode::Modify1)
        .try_into()
        .unwrap();
    mix.mem[5] = Instruction::new(0, 3, 1, Opcode::Modify1)
        .try_into()
        .unwrap();

    mix.r_in[1].set(0..=2, &[0, 9, 5]).unwrap();

    mix.restart();

    mix.step().unwrap();
    assert_eq!(mix.halted, false);
    assert_eq!(mix.r_in[1][0..=2], [0, 9, 6]);

    mix.step().unwrap();
    assert_eq!(mix.halted, false);
    assert_eq!(mix.r_in[1][0..=2], [0, 9, 8]);

    mix.step().unwrap();
    assert_eq!(mix.halted, false);
    assert_eq!(mix.r_in[1][0..=2], [0, 9, 6]);

    mix.step().unwrap();
    assert_eq!(mix.halted, false);
    assert_eq!(mix.r_in[1][0..=2], [0, 0x04, 0xD2]);

    mix.step().unwrap();
    assert_eq!(mix.halted, false);
    assert_eq!(mix.r_in[1][0..=2], [1, 0x10, 0xE1]);

    mix.step().unwrap();
    assert_eq!(mix.halted, false);
    assert_eq!(mix.r_in[1][0..=2], [0, 0x10, 0xE1]);
}

#[test]
fn test_add_sub() {
    let mut mix = MixMachine::new();
    mix.reset();

    mix.mem[0] = Instruction::new(1000, 5, 0, Opcode::Add)
        .try_into()
        .unwrap();
    mix.mem[1] = Instruction::new(1000, 5, 0, Opcode::Sub)
        .try_into()
        .unwrap();

    mix.mem[1000].set(0..=5, &[0, 0, 0x64, 5, 0, 0x32]).unwrap();
    mix.r_a.set(0..=5, &[0, 0x04, 0xD2, 1, 0, 0x96]).unwrap();

    mix.restart();

    mix.step().unwrap();
    assert_eq!(mix.halted, false);
    assert_eq!(mix.r_a[0..=5], [0, 0x05, 0x36, 6, 0, 0xC8]);

    mix.step().unwrap();
    assert_eq!(mix.halted, false);
    assert_eq!(mix.r_a[0..=5], [0, 0x04, 0xD2, 1, 0, 0x96]);

    mix.reset();

    mix.mem[0] = Instruction::new(1000, 5, 0, Opcode::Sub)
        .try_into()
        .unwrap();
    mix.mem[1] = Instruction::new(1000, 5, 0, Opcode::Add)
        .try_into()
        .unwrap();

    mix.mem[1000]
        .set(0..=5, &[1, 0x07, 0xD0, 0, 0x96, 0])
        .unwrap();
    mix.r_a.set(0..=5, &[1, 0x04, 0xD2, 0, 0, 9]).unwrap();

    mix.restart();

    mix.step().unwrap();
    assert_eq!(mix.halted, false);
    assert_eq!(mix.r_a[0..=4], [0, 0x02, 0xFE, 0, 0x95]);
}

#[test]
fn test_mul() {
    let mut mix = MixMachine::new();
    mix.reset();

    mix.mem[0] = Instruction::new(1000, 9, 0, Opcode::Mul)
        .try_into()
        .unwrap();

    mix.mem[1000].set(0..=5, &[0, 2, 0, 0, 0, 0]).unwrap();
    mix.r_a.set(0..=5, &[1, 0, 0, 0, 0, 0x70]).unwrap();

    mix.restart();

    mix.step().unwrap();
    assert_eq!(mix.halted, false);
    assert_eq!(mix.r_a[0..=5], [1, 0, 0, 0, 0, 0]);
    assert_eq!(mix.r_x[0..=5], [1, 0, 0, 0, 0, 0xE0]);

    mix.reset();

    mix.mem[0] = Instruction::new(1000, 5, 0, Opcode::Mul)
        .try_into()
        .unwrap();

    mix.mem[1000].set(0..=5, &[0, 1, 1, 1, 1, 1]).unwrap();
    mix.r_a.set(0..=5, &[0, 1, 1, 1, 1, 1]).unwrap();

    mix.restart();

    mix.step().unwrap();
    assert_eq!(mix.halted, false);
    assert_eq!(mix.r_a[0..=5], [0, 0, 1, 2, 3, 4]);
    assert_eq!(mix.r_x[0..=5], [0, 5, 4, 3, 2, 1]);
}

#[test]
fn test_div() {
    let mut mix = MixMachine::new();
    mix.reset();

    mix.mem[0] = Instruction::new(1000, 5, 0, Opcode::Div)
        .try_into()
        .unwrap();

    mix.mem[1000].set(0..=5, &[0, 0, 0, 0, 0, 3]).unwrap();
    mix.r_a.set(0..=5, &[0, 0, 0, 0, 0, 0]).unwrap();
    mix.r_x.set(0..=5, &[1, 0, 0, 0, 0, 0x11]).unwrap();

    mix.restart();

    mix.step().unwrap();
    assert_eq!(mix.halted, false);
    assert_eq!(mix.r_a[0..=5], [0, 0, 0, 0, 0, 5]);
    assert_eq!(mix.r_x[0..=5], [0, 0, 0, 0, 0, 2]);

    mix.reset();

    mix.mem[0] = Instruction::new(1000, 5, 0, Opcode::Div)
        .try_into()
        .unwrap();

    mix.mem[1000].set(0..=5, &[1, 0, 0, 0, 2, 0]).unwrap();
    mix.r_a.set(0..=5, &[1, 0, 0, 0, 0, 0]).unwrap();
    mix.r_x.set(0..=5, &[0, 0x04, 0xD3, 0, 3, 1]).unwrap();

    mix.restart();

    mix.step().unwrap();
    assert_eq!(mix.halted, false);
    assert_eq!(mix.r_a[0..=3], [0, 0, 0x02, 0x69]);
    assert_eq!(mix.r_x[0], 1);
    assert_eq!(mix.r_x[5], 1);
}

#[test]
fn test_cmp_6b() {
    let mut mix = MixMachine::new();
    mix.reset();

    mix.mem[0] = Instruction::new(1000, 5, 0, Opcode::CmpA)
        .try_into()
        .unwrap();
    mix.mem[1] = Instruction::new(1001, 5, 0, Opcode::CmpA)
        .try_into()
        .unwrap();
    mix.mem[2] = Instruction::new(1002, 5, 0, Opcode::CmpA)
        .try_into()
        .unwrap();
    mix.mem[3] = Instruction::new(1003, 5, 0, Opcode::CmpX)
        .try_into()
        .unwrap();

    mix.mem[1000].set(0..=5, &[0, 0, 0, 0, 0, 2]).unwrap();
    mix.mem[1001].set(0..=5, &[1, 0, 0, 0, 0, 2]).unwrap();
    mix.mem[1002].set(0..=5, &[0, 0, 0, 0, 0, 1]).unwrap();
    mix.mem[1003].set(0..=5, &[1, 0, 0, 0, 0, 0]).unwrap();
    mix.r_a.set(0..=5, &[0, 0, 0, 0, 0, 1]).unwrap();
    mix.r_x.set(0..=5, &[0, 0, 0, 0, 0, 0]).unwrap();

    mix.restart();

    mix.step().unwrap();
    assert_eq!(mix.halted, false);
    assert_eq!(mix.indicator_comp, ComparisonIndicatorValue::Lesser);

    mix.step().unwrap();
    assert_eq!(mix.halted, false);
    assert_eq!(mix.indicator_comp, ComparisonIndicatorValue::Greater);

    mix.step().unwrap();
    assert_eq!(mix.halted, false);
    assert_eq!(mix.indicator_comp, ComparisonIndicatorValue::Equal);

    mix.step().unwrap();
    assert_eq!(mix.halted, false);
    assert_eq!(mix.indicator_comp, ComparisonIndicatorValue::Equal);
}

#[test]
fn test_cmp_3b() {
    let mut mix = MixMachine::new();
    mix.reset();

    mix.mem[0] = Instruction::new(1000, 5, 0, Opcode::Cmp1)
        .try_into()
        .unwrap();
    mix.mem[1] = Instruction::new(1001, 5, 0, Opcode::Cmp1)
        .try_into()
        .unwrap();
    mix.mem[2] = Instruction::new(1002, 5, 0, Opcode::Cmp1)
        .try_into()
        .unwrap();
    mix.mem[3] = Instruction::new(1003, 5, 0, Opcode::Cmp2)
        .try_into()
        .unwrap();

    mix.mem[1000].set(0..=5, &[0, 0, 0, 0, 0, 2]).unwrap();
    mix.mem[1001].set(0..=5, &[1, 0, 0, 0, 0, 2]).unwrap();
    mix.mem[1002].set(0..=5, &[0, 0, 0, 0, 0, 1]).unwrap();
    mix.mem[1003].set(0..=5, &[1, 0, 0, 0, 0, 0]).unwrap();
    mix.r_in[1].set(0..=2, &[0, 0, 1]).unwrap();
    mix.r_in[2].set(0..=2, &[0, 0, 0]).unwrap();

    mix.restart();

    mix.step().unwrap();
    assert_eq!(mix.halted, false);
    assert_eq!(mix.indicator_comp, ComparisonIndicatorValue::Lesser);

    mix.step().unwrap();
    assert_eq!(mix.halted, false);
    assert_eq!(mix.indicator_comp, ComparisonIndicatorValue::Greater);

    mix.step().unwrap();
    assert_eq!(mix.halted, false);
    assert_eq!(mix.indicator_comp, ComparisonIndicatorValue::Equal);

    mix.step().unwrap();
    assert_eq!(mix.halted, false);
    assert_eq!(mix.indicator_comp, ComparisonIndicatorValue::Equal);
}

#[test]
fn test_jmp_reg_6b() {
    let mut mix = MixMachine::new();
    mix.reset();

    mix.mem[0] = Instruction::new(1000, 0, 0, Opcode::JA).try_into().unwrap();
    mix.mem[1000] = Instruction::new(2000, 2, 0, Opcode::JX).try_into().unwrap();
    mix.mem[1001] = Instruction::new(0, 1, 0, Opcode::JX).try_into().unwrap();
    mix.r_a.set(0..=5, &[1, 0, 0, 0, 0, 1]).unwrap();
    mix.r_x.set(0..=5, &[0, 0, 0, 0, 0, 0]).unwrap();

    mix.restart();

    mix.step().unwrap();
    assert_eq!(mix.halted, false);
    assert_eq!(mix.pc, 1000);
    assert_eq!(mix.r_j[0..=2], [0, 0, 1]);

    mix.step().unwrap();
    assert_eq!(mix.halted, false);
    assert_eq!(mix.pc, 1001);
    assert_eq!(mix.r_j[0..=2], [0, 0, 1]);

    mix.step().unwrap();
    assert_eq!(mix.halted, false);
    assert_eq!(mix.pc, 0);
    assert_eq!(mix.r_j[0..=2], [0, 0x03, 0xEA]);
}

#[test]
fn test_jmp_reg_3b() {
    let mut mix = MixMachine::new();
    mix.reset();

    mix.mem[0] = Instruction::new(1000, 0, 0, Opcode::J1).try_into().unwrap();
    mix.mem[1000] = Instruction::new(2000, 2, 0, Opcode::J2).try_into().unwrap();
    mix.mem[1001] = Instruction::new(0, 1, 0, Opcode::J2).try_into().unwrap();
    mix.r_in[1].set(0..=2, &[1, 0, 1]).unwrap();
    mix.r_in[2].set(0..=2, &[0, 0, 0]).unwrap();

    mix.restart();

    mix.step().unwrap();
    assert_eq!(mix.halted, false);
    assert_eq!(mix.pc, 1000);
    assert_eq!(mix.r_j[0..=2], [0, 0, 1]);

    mix.step().unwrap();
    assert_eq!(mix.halted, false);
    assert_eq!(mix.pc, 1001);
    assert_eq!(mix.r_j[0..=2], [0, 0, 1]);

    mix.step().unwrap();
    assert_eq!(mix.halted, false);
    assert_eq!(mix.pc, 0);
    assert_eq!(mix.r_j[0..=2], [0, 0x03, 0xEA]);
}

#[test]
fn test_shift() {
    let mut mix = MixMachine::new();
    mix.reset();

    mix.mem[0] = Instruction::new(1, 3, 0, Opcode::Shift).try_into().unwrap();
    mix.mem[1] = Instruction::new(2, 0, 0, Opcode::Shift).try_into().unwrap();
    mix.mem[2] = Instruction::new(4, 5, 0, Opcode::Shift).try_into().unwrap();
    mix.mem[3] = Instruction::new(2, 1, 0, Opcode::Shift).try_into().unwrap();
    mix.mem[4] = Instruction::new(501, 4, 0, Opcode::Shift)
        .try_into()
        .unwrap();
    mix.r_a.set(0..=5, &[0, 1, 2, 3, 4, 5]).unwrap();
    mix.r_x.set(0..=5, &[1, 6, 7, 8, 9, 10]).unwrap();

    mix.restart();
    mix.step().unwrap();
    assert_eq!(mix.halted, false);
    assert_eq!(mix.r_a[0..=5], [0, 0, 1, 2, 3, 4]);
    assert_eq!(mix.r_x[0..=5], [1, 5, 6, 7, 8, 9]);

    mix.step().unwrap();
    assert_eq!(mix.halted, false);
    assert_eq!(mix.r_a[0..=5], [0, 2, 3, 4, 0, 0]);
    assert_eq!(mix.r_x[0..=5], [1, 5, 6, 7, 8, 9]);

    mix.step().unwrap();
    assert_eq!(mix.halted, false);
    assert_eq!(mix.r_a[0..=5], [0, 6, 7, 8, 9, 2]);
    assert_eq!(mix.r_x[0..=5], [1, 3, 4, 0, 0, 5]);

    mix.step().unwrap();
    assert_eq!(mix.halted, false);
    assert_eq!(mix.r_a[0..=5], [0, 0, 0, 6, 7, 8]);
    assert_eq!(mix.r_x[0..=5], [1, 3, 4, 0, 0, 5]);

    mix.step().unwrap();
    assert_eq!(mix.halted, false);
    assert_eq!(mix.r_a[0..=5], [0, 0, 6, 7, 8, 3]);
    assert_eq!(mix.r_x[0..=5], [1, 4, 0, 0, 5, 0]);
}

struct BusyIODevice {}

impl IODevice for BusyIODevice {
    fn read(&mut self) -> Result<Vec<crate::sim::mem::Word<6, false>>, ()> {
        unimplemented!()
    }

    fn write(&mut self, _: &[crate::sim::mem::Word<6, false>]) -> Result<(), usize> {
        unimplemented!()
    }

    fn control(&mut self, _: i16) -> Result<(), ()> {
        unimplemented!()
    }

    fn is_busy(&self) -> Result<bool, ()> {
        Ok(true)
    }

    fn is_ready(&self) -> Result<bool, ()> {
        Ok(false)
    }

    fn get_block_size(&self) -> usize {
        0
    }
}

struct ReadyIODevice {}

impl IODevice for ReadyIODevice {
    fn read(&mut self) -> Result<Vec<crate::sim::mem::Word<6, false>>, ()> {
        unimplemented!()
    }

    fn write(&mut self, _: &[crate::sim::mem::Word<6, false>]) -> Result<(), usize> {
        unimplemented!()
    }

    fn control(&mut self, _: i16) -> Result<(), ()> {
        unimplemented!()
    }

    fn is_busy(&self) -> Result<bool, ()> {
        Ok(false)
    }

    fn is_ready(&self) -> Result<bool, ()> {
        Ok(true)
    }

    fn get_block_size(&self) -> usize {
        0
    }
}

#[test]
fn test_jbus_jred() {
    let mut mix = MixMachine::new();
    mix.reset();

    mix.io_devices[0] = Some(Box::new(ReadyIODevice {}));
    mix.io_devices[1] = Some(Box::new(BusyIODevice {}));

    mix.mem[0] = Instruction::new(100, 0, 0, Opcode::Jred)
        .try_into()
        .unwrap();
    mix.mem[100] = Instruction::new(200, 1, 0, Opcode::Jred)
        .try_into()
        .unwrap();
    mix.mem[101] = Instruction::new(200, 0, 0, Opcode::Jbus)
        .try_into()
        .unwrap();
    mix.mem[102] = Instruction::new(0, 1, 0, Opcode::Jbus).try_into().unwrap();

    mix.restart();

    mix.step().unwrap();
    assert_eq!(mix.halted, false);
    assert_eq!(mix.pc, 100);
    assert_eq!(mix.r_j[0..=2], [0, 0, 1]);

    mix.step().unwrap();
    assert_eq!(mix.halted, false);
    assert_eq!(mix.pc, 101);
    assert_eq!(mix.r_j[0..=2], [0, 0, 1]);

    mix.step().unwrap();
    assert_eq!(mix.halted, false);
    assert_eq!(mix.pc, 102);
    assert_eq!(mix.r_j[0..=2], [0, 0, 1]);

    mix.step().unwrap();
    assert_eq!(mix.halted, false);
    assert_eq!(mix.pc, 0);
    assert_eq!(mix.r_j[0..=2], [0, 0, 0x67]);
}

struct LoggedControlIODevice {
    expected_command: i16,
}

impl IODevice for LoggedControlIODevice {
    fn read(&mut self) -> Result<Vec<crate::sim::mem::Word<6, false>>, ()> {
        unimplemented!()
    }

    fn write(&mut self, _: &[crate::sim::mem::Word<6, false>]) -> Result<(), usize> {
        unimplemented!()
    }

    fn control(&mut self, command: i16) -> Result<(), ()> {
        assert_eq!(command, self.expected_command);
        Ok(())
    }

    fn is_busy(&self) -> Result<bool, ()> {
        unimplemented!()
    }

    fn is_ready(&self) -> Result<bool, ()> {
        unimplemented!()
    }

    fn get_block_size(&self) -> usize {
        0
    }
}

#[test]
fn test_ioc() {
    let mut mix = MixMachine::new();
    mix.reset();

    mix.io_devices[0] = Some(Box::new(LoggedControlIODevice {
        expected_command: -101,
    }));

    mix.mem[0] = Instruction::new(-101, 0, 0, Opcode::Ioc)
        .try_into()
        .unwrap();

    mix.restart();

    mix.step().unwrap();
    assert_eq!(mix.halted, false);
}

struct InOutIODevice {}

impl IODevice for InOutIODevice {
    fn read(&mut self) -> Result<Vec<crate::sim::mem::Word<6, false>>, ()> {
        let mut w = Word::<6, false>::new();
        w.set(0..=5, &[0, 9, 8, 7, 6, 5])?;
        Ok(vec![w])
    }

    fn write(&mut self, data: &[crate::sim::mem::Word<6, false>]) -> Result<(), usize> {
        assert_eq!(data.len(), self.get_block_size());
        assert_eq!(data[0][0..=5], [0, 1, 2, 3, 4, 5]);
        Ok(())
    }

    fn control(&mut self, _: i16) -> Result<(), ()> {
        unimplemented!()
    }

    fn is_busy(&self) -> Result<bool, ()> {
        unimplemented!()
    }

    fn is_ready(&self) -> Result<bool, ()> {
        unimplemented!()
    }

    fn get_block_size(&self) -> usize {
        1
    }
}

#[test]
fn test_in_out() {
    let mut mix = MixMachine::new();
    mix.reset();

    mix.io_devices[0] = Some(Box::new(InOutIODevice {}));

    mix.mem[0] = Instruction::new(1000, 0, 0, Opcode::In).try_into().unwrap();
    mix.mem[1] = Instruction::new(2000, 0, 0, Opcode::Out)
        .try_into()
        .unwrap();
    mix.mem[2000].set(0..=5, &[0, 1, 2, 3, 4, 5]).unwrap();

    mix.restart();

    mix.step().unwrap();
    assert_eq!(mix.halted, false);
    assert_eq!(mix.mem[1000][0..=5], [0, 9, 8, 7, 6, 5]);

    mix.step().unwrap();
    assert_eq!(mix.halted, false);
}
