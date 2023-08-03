use crate::sim::*;

#[test]
fn test_binary_jmp_reg_6b() {
    let mut mix = MixVM::new();
    mix.reset();

    mix.mem[0] = Instruction::new(1000, 6, 0, Opcode::JA).into();
    mix.mem[1000] = Instruction::new(2000, 6, 0, Opcode::JX).into();
    mix.mem[1001] = Instruction::new(0, 7, 0, Opcode::JX).into();
    mix.r_a.set_all([1, 0, 0, 0, 0, 0]);
    mix.r_x.set_all([0, 0, 0, 0, 0, 1]);

    mix.restart();

    mix.step().unwrap();
    assert_eq!(mix.halted, false);
    assert_eq!(mix.pc, 1000);
    assert_eq!(mix.r_j[..], [0, 0, 1]);

    mix.step().unwrap();
    assert_eq!(mix.halted, false);
    assert_eq!(mix.pc, 1001);
    assert_eq!(mix.r_j[..], [0, 0, 1]);

    mix.step().unwrap();
    assert_eq!(mix.halted, false);
    assert_eq!(mix.pc, 0);
    assert_eq!(mix.r_j[..], [0, 0x03, 0xEA]);
}

#[test]
fn test_binary_shift() {
    let mut mix = MixVM::new();
    mix.reset();

    mix.mem[0] = Instruction::new(1, 6, 0, Opcode::Shift).into();
    mix.mem[1] = Instruction::new(2, 7, 0, Opcode::Shift).into();
    mix.mem[4] = Instruction::new(501, 4, 0, Opcode::Shift).into();
    mix.r_a.set_all([0, 0, 0, 0, 0, 0b00000110]);
    mix.r_x.set_all([1, 0, 0, 0, 0, 0b00000001]);

    mix.restart();
    mix.step().unwrap();
    assert_eq!(mix.halted, false);
    assert_eq!(mix.r_a[..], [0, 0, 0, 0, 0, 0b00001100]);
    assert_eq!(mix.r_x[..], [1, 0, 0, 0, 0, 0b00000010]);

    mix.step().unwrap();
    assert_eq!(mix.halted, false);
    assert_eq!(mix.r_a[..], [0, 0, 0, 0, 0, 0b00000011]);
    assert_eq!(mix.r_x[..], [1, 0, 0, 0, 0, 0b00000000]);
}

#[test]
fn test_binary_gcd() {
    let mut mix = MixVM::new();
    mix.reset();

    // * Test sequence source: D. E. Knuth,
    // * 'The Art of Computer Programming', Volume 2, pp. 340.
    // * Algorithm: Binary GCD algorithm. U, V are the two numbers
    // * awaiting processing.
    // B1  ENT1 0
    //     LDX  U
    //     LDAN V
    //     JMP  1F
    // 2H  SRB  1
    //     INC1 1
    //     STX  U
    //     STA  V(1:5)
    // 1H  JXO  B4
    // B2  JAE  2B
    //     LDA  U
    // B3  SRB  1
    // B4  JAE  B3
    // B5  JAN  1F
    //     STA  U
    //     SUB  V
    //     JMP  2F
    // 1H  STA  V(1:5)
    // B6  ADD  U
    // 2H  JANZ B3
    //     LDA  U
    //     ENTX 0
    //     SLB  0,1
    const U: i16 = 1000;
    const V: i16 = 1001;
    mix.mem[0] = Instruction::new(0, 2, 0, Opcode::Modify1).into();
    mix.mem[1] = Instruction::new(U, 5, 0, Opcode::LdX).into();
    mix.mem[2] = Instruction::new(V, 5, 0, Opcode::LdAN).into();
    mix.mem[3] = Instruction::new(8, 0, 0, Opcode::Jmp).into();
    mix.mem[4] = Instruction::new(1, 7, 0, Opcode::Shift).into();
    mix.mem[5] = Instruction::new(1, 0, 0, Opcode::Modify1).into();
    mix.mem[6] = Instruction::new(U, 5, 0, Opcode::StX).into();
    mix.mem[7] = Instruction::new(V, 13, 0, Opcode::StA).into();
    mix.mem[8] = Instruction::new(12, 7, 0, Opcode::JX).into();
    mix.mem[9] = Instruction::new(4, 6, 0, Opcode::JA).into();
    mix.mem[10] = Instruction::new(U, 5, 0, Opcode::LdA).into();
    mix.mem[11] = Instruction::new(1, 7, 0, Opcode::Shift).into();
    mix.mem[12] = Instruction::new(11, 6, 0, Opcode::JA).into();
    mix.mem[13] = Instruction::new(17, 0, 0, Opcode::JA).into();
    mix.mem[14] = Instruction::new(U, 5, 0, Opcode::StA).into();
    mix.mem[15] = Instruction::new(V, 5, 0, Opcode::Sub).into();
    mix.mem[16] = Instruction::new(19, 0, 0, Opcode::Jmp).into();
    mix.mem[17] = Instruction::new(V, 13, 0, Opcode::StA).into();
    mix.mem[18] = Instruction::new(U, 5, 0, Opcode::Add).into();
    mix.mem[19] = Instruction::new(11, 4, 0, Opcode::JA).into();
    mix.mem[20] = Instruction::new(U, 5, 0, Opcode::LdA).into();
    mix.mem[21] = Instruction::new(0, 2, 0, Opcode::ModifyX).into();
    mix.mem[22] = Instruction::new(0, 6, 1, Opcode::Shift).into();
    mix.mem[23] = Instruction::new(0, 2, 0, Opcode::Special).into();

    mix.mem[U as u16] = Word::from_i64(1360).0;
    mix.mem[V as u16] = Word::from_i64(646).0;

    mix.restart();

    while !mix.halted {
        mix.step().unwrap();
    }

    // Correct answer is rA = 34.
    assert_eq!(mix.r_a[..], [0, 0, 0, 0, 0, 0x22]);
}
