use crate::sim::mix::instr::*;
use crate::sim::mix::mem::*;
use crate::sim::mix::mix_machine::*;
use crate::sim::mix::reg::*;

#[test]
fn test_euclid() {
    let mut mix = MixMachine::new();
    mix.reset();

    // * Test sequence source: D. E. Knuth,
    // * 'The Art of Computer Programming', Volume 2, pp. 337.
    // * Algorithm: Euclid's GCD algorithm. U, V are the two numbers
    // * awaiting processing.
    //     LDX U
    mix.mem[0] = Instruction::new(1000, 5, 0, Opcode::LdX)
        .try_into()
        .unwrap();
    //     JMP 2F
    mix.mem[1] = Instruction::new(5, 0, 0, Opcode::Jmp).try_into().unwrap();
    // 1H  STX V
    mix.mem[2] = Instruction::new(1001, 5, 0, Opcode::StX)
        .try_into()
        .unwrap();
    //     SRAX 5
    mix.mem[3] = Instruction::new(5, 3, 0, Opcode::Shift).try_into().unwrap();
    //     DIV V
    mix.mem[4] = Instruction::new(1001, 5, 0, Opcode::Div)
        .try_into()
        .unwrap();
    // 2H  LDA V
    mix.mem[5] = Instruction::new(1001, 5, 0, Opcode::LdA)
        .try_into()
        .unwrap();
    //     JXNZ 1B
    mix.mem[6] = Instruction::new(2, 4, 0, Opcode::JX).try_into().unwrap();
    //     HLT
    mix.mem[7] = Instruction::new(0, 2, 0, Opcode::Special)
        .try_into()
        .unwrap();
    //     ORIG 1000
    // U   CON 1360
    mix.mem[1000] = Word::<6, false>::from_i64(1360).0;
    // V   CON 646
    mix.mem[1001] = Word::<6, false>::from_i64(646).0;

    mix.restart();

    while !mix.halted {
        mix.step().unwrap();
    }

    // Correct answer is rA = 34.
    assert_eq!(mix.r_a[0..=5], [0, 0, 0, 0, 0, 0x22]);
}
