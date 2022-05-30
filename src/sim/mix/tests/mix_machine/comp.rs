use crate::sim::mix::instr::*;
use crate::sim::mix::mem::*;
use crate::sim::mix::mix_machine::*;

#[test]
fn test_euclid() {
    let mut mix = MixMachine::new();
    mix.reset();

    // * Test sequence source: D. E. Knuth,
    // * 'The Art of Computer Programming', Volume 2, pp. 337.
    // * Algorithm: Euclid's GCD algorithm. U, V are the two numbers
    // * awaiting processing.
    //     LDX  U
    //     JMP  2F
    // 1H  STX  V
    //     SRAX 5
    //     DIV  V
    // 2H  LDA  V
    //     JXNZ 1B
    //     HLT
    //     ORIG 1000
    // U   CON  1360
    // V   CON  646
    mix.mem[0] = Instruction::new(1000, 5, 0, Opcode::LdX)
        .try_into()
        .unwrap();
    mix.mem[1] = Instruction::new(5, 0, 0, Opcode::Jmp).try_into().unwrap();
    mix.mem[2] = Instruction::new(1001, 5, 0, Opcode::StX)
        .try_into()
        .unwrap();
    mix.mem[3] = Instruction::new(5, 3, 0, Opcode::Shift).try_into().unwrap();
    mix.mem[4] = Instruction::new(1001, 5, 0, Opcode::Div)
        .try_into()
        .unwrap();
    mix.mem[5] = Instruction::new(1001, 5, 0, Opcode::LdA)
        .try_into()
        .unwrap();
    mix.mem[6] = Instruction::new(2, 4, 0, Opcode::JX).try_into().unwrap();
    mix.mem[7] = Instruction::new(0, 2, 0, Opcode::Special)
        .try_into()
        .unwrap();
    mix.mem[1000] = Word::<6, false>::from_i64(1360).0;
    mix.mem[1001] = Word::<6, false>::from_i64(646).0;

    mix.restart();

    while !mix.halted {
        mix.step().unwrap();
    }

    // Correct answer is rA = 34.
    assert_eq!(mix.r_a[0..=5], [0, 0, 0, 0, 0, 0x22]);
}

#[test]
fn test_ones() {
    let mut mix = MixMachine::new();
    mix.reset();

    // * Test sequence source: D. E. Knuth,
    // * 'The Art of Computer Programming', Volume 1, pp. 142.
    // * Algorithm: 'Ones program'
    //     ORIG 1000
    //     STZ  1
    //     ENNX 1
    //     STX  1(0:1)
    //     ENNA 1
    //     INCX 1
    //     ENT1 1
    //     SRC  1
    //     ADD  1
    //     DEC1 -1
    //     STZ  1
    //     CMPA 1
    //     MOVE -1,1(1)
    //     NUM  1
    //     CHAR 1
    //     HLT  1
    mix.mem[1000] = Instruction::new(1, 5, 0, Opcode::StZ).try_into().unwrap();
    mix.mem[1001] = Instruction::new(1, 3, 0, Opcode::ModifyX)
        .try_into()
        .unwrap();
    mix.mem[1002] = Instruction::new(1, 1, 0, Opcode::StX).try_into().unwrap();
    mix.mem[1003] = Instruction::new(1, 2, 0, Opcode::Shift).try_into().unwrap();
    mix.mem[1004] = Instruction::new(1, 3, 0, Opcode::ModifyA)
        .try_into()
        .unwrap();
    mix.mem[1005] = Instruction::new(1, 0, 0, Opcode::ModifyX)
        .try_into()
        .unwrap();
    mix.mem[1006] = Instruction::new(1, 2, 0, Opcode::Modify1)
        .try_into()
        .unwrap();
    mix.mem[1007] = Instruction::new(1, 5, 0, Opcode::Shift).try_into().unwrap();
    mix.mem[1008] = Instruction::new(1, 5, 0, Opcode::Add).try_into().unwrap();
    mix.mem[1009] = Instruction::new(-1, 1, 0, Opcode::Modify1)
        .try_into()
        .unwrap();
    mix.mem[1010] = Instruction::new(1, 5, 0, Opcode::StZ).try_into().unwrap();
    mix.mem[1011] = Instruction::new(1, 5, 0, Opcode::CmpA).try_into().unwrap();
    mix.mem[1012] = Instruction::new(-1, 1, 1, Opcode::Move).try_into().unwrap();
    mix.mem[1013] = Instruction::new(1, 0, 0, Opcode::Special)
        .try_into()
        .unwrap();
    mix.mem[1014] = Instruction::new(1, 1, 0, Opcode::Special)
        .try_into()
        .unwrap();
    mix.mem[1015] = Instruction::new(1, 2, 0, Opcode::Special)
        .try_into()
        .unwrap();

    mix.pc = 1000;

    mix.restart();

    while !mix.halted {
        mix.step().unwrap();
    }

    assert_eq!(mix.r_a[0..=5], [1, 30, 30, 30, 30, 30]);
    assert_eq!(mix.r_x[0..=5], [1, 31, 30, 30, 30, 30]);
    assert_eq!(mix.r_in[1][0..=2], [0, 0, 3]);
    assert_eq!(mix.indicator_comp, ComparisonIndicatorValue::Equal);
    assert_eq!(mix.overflow, true);
}

#[test]
fn test_exp_13() {
    let mut mix = MixMachine::new();
    mix.reset();

    // * Test sequence source: D. E. Knuth,
    // * 'The Art of Computer Programming', Volume 1, pp 509.
    // * Algorithm: 'X^13 program'
    //     ORIG 3000
    //     LDA  2000
    //     MUL  2000(1:5)
    //     STX  3500(1:1)
    //     SRC  1
    //     MUL  3500
    //     STA  3501
    //     ADD  2000
    //     MUL  3501(1:5)
    //     STX  3501
    //     MUL  3501(1:5)
    //     SLAX 1
    //     HLT  0
    //     ORIG 3500
    //     CON  0
    //     CON  0
    mix.mem[3000] = Instruction::new(2000, 5, 0, Opcode::LdA)
        .try_into()
        .unwrap();
    mix.mem[3001] = Instruction::new(2000, 13, 0, Opcode::Mul)
        .try_into()
        .unwrap();
    mix.mem[3002] = Instruction::new(3500, 9, 0, Opcode::StX)
        .try_into()
        .unwrap();
    mix.mem[3003] = Instruction::new(1, 5, 0, Opcode::Shift).try_into().unwrap();
    mix.mem[3004] = Instruction::new(3500, 5, 0, Opcode::Mul)
        .try_into()
        .unwrap();
    mix.mem[3005] = Instruction::new(3501, 5, 0, Opcode::StA)
        .try_into()
        .unwrap();
    mix.mem[3006] = Instruction::new(2000, 5, 0, Opcode::Add)
        .try_into()
        .unwrap();
    mix.mem[3007] = Instruction::new(3501, 13, 0, Opcode::Mul)
        .try_into()
        .unwrap();
    mix.mem[3008] = Instruction::new(3501, 5, 0, Opcode::StX)
        .try_into()
        .unwrap();
    mix.mem[3009] = Instruction::new(3501, 13, 0, Opcode::Mul)
        .try_into()
        .unwrap();
    mix.mem[3010] = Instruction::new(1, 2, 0, Opcode::Shift).try_into().unwrap();
    mix.mem[3011] = Instruction::new(0, 2, 0, Opcode::Special)
        .try_into()
        .unwrap();
    mix.mem[2000] = Word::from_i64(3).0;

    mix.pc = 3000;

    mix.restart();

    while !mix.halted {
        mix.step().unwrap();
    }

    assert_eq!(mix.r_a[0..=5], [0, 0, 0, 0x18, 0x53, 0xD3]);
}
