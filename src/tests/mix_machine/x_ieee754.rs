use crate::*;

use core::f32::consts;

#[test]
fn test_ieee754_add_sub() {
    let mut mix = MixVM::new();
    mix.reset();

    let pi_bytes = consts::PI.to_be_bytes();
    let neg_pi_bytes = (-consts::PI).to_be_bytes();
    let zero_bytes = 0f32.to_be_bytes();

    mix.mem[0] = Instruction::new(1000, 7, 0, Opcode::Add).into();
    mix.mem[1] = Instruction::new(1000, 7, 0, Opcode::Sub).into();
    mix.r_a
        .set_all([0, 0, pi_bytes[0], pi_bytes[1], pi_bytes[2], pi_bytes[3]]);
    mix.mem[1000].set_all([
        1,
        0,
        neg_pi_bytes[0],
        neg_pi_bytes[1],
        neg_pi_bytes[2],
        neg_pi_bytes[3],
    ]);

    mix.restart();

    mix.step().unwrap();
    assert_eq!(mix.halted, false);
    assert_eq!(
        mix.r_a[..],
        [
            0,
            0,
            zero_bytes[0],
            zero_bytes[1],
            zero_bytes[2],
            zero_bytes[3]
        ]
    );

    mix.step().unwrap();
    assert_eq!(mix.halted, false);
    assert_eq!(
        mix.r_a[..],
        [0, 0, pi_bytes[0], pi_bytes[1], pi_bytes[2], pi_bytes[3]]
    );
}

#[test]
fn test_ieee754_mul_div() {
    let mut mix = MixVM::new();
    mix.reset();

    let pi_bytes = consts::PI.to_be_bytes();
    let e_bytes = consts::E.to_be_bytes();

    let e_pi_bytes = (consts::E * consts::PI).to_be_bytes();
    let e_div_pi_bytes = (consts::E / consts::PI).to_be_bytes();

    mix.mem[0] = Instruction::new(1001, 5, 0, Opcode::LdA).into();
    mix.mem[1] = Instruction::new(1000, 7, 0, Opcode::Mul).into();
    mix.mem[2] = Instruction::new(1001, 5, 0, Opcode::LdA).into();
    mix.mem[3] = Instruction::new(1000, 7, 0, Opcode::Div).into();
    mix.mem[1000].set_all([0, 0, pi_bytes[0], pi_bytes[1], pi_bytes[2], pi_bytes[3]]);
    mix.mem[1001].set_all([0, 0, e_bytes[0], e_bytes[1], e_bytes[2], e_bytes[3]]);

    mix.restart();

    mix.step().unwrap();
    assert_eq!(mix.halted, false);

    mix.step().unwrap();
    assert_eq!(mix.halted, false);
    assert_eq!(
        mix.r_a[..],
        [
            0,
            0,
            e_pi_bytes[0],
            e_pi_bytes[1],
            e_pi_bytes[2],
            e_pi_bytes[3]
        ]
    );

    mix.step().unwrap();
    assert_eq!(mix.halted, false);

    mix.step().unwrap();
    assert_eq!(mix.halted, false);
    assert_eq!(
        mix.r_a[..],
        [
            0,
            0,
            e_div_pi_bytes[0],
            e_div_pi_bytes[1],
            e_div_pi_bytes[2],
            e_div_pi_bytes[3]
        ]
    );
}

#[test]
fn test_ieee754_cmp() {
    let mut mix = MixVM::new();
    mix.reset();

    let e_bytes = consts::E.to_be_bytes();
    let pi_bytes = consts::PI.to_be_bytes();
    let neg_pi_bytes = (-consts::E).to_be_bytes();
    let pos_inf_bytes = f32::INFINITY.to_be_bytes();
    let neg_inf_bytes = f32::NEG_INFINITY.to_be_bytes();
    let nan_bytes = f32::NAN.to_be_bytes();

    mix.mem[0] = Instruction::new(1000, 5, 0, Opcode::LdA).into();
    mix.mem[1] = Instruction::new(1001, 7, 0, Opcode::CmpA).into();
    mix.mem[2] = Instruction::new(1002, 7, 0, Opcode::CmpA).into();
    mix.mem[3] = Instruction::new(1003, 7, 0, Opcode::CmpA).into();
    mix.mem[4] = Instruction::new(1004, 7, 0, Opcode::CmpA).into();
    mix.mem[5] = Instruction::new(1005, 7, 0, Opcode::CmpA).into();
    mix.mem[1000].set_all([0, 0, e_bytes[0], e_bytes[1], e_bytes[2], e_bytes[3]]);
    mix.mem[1001].set_all([0, 0, pi_bytes[0], pi_bytes[1], pi_bytes[2], pi_bytes[3]]);
    mix.mem[1002].set_all([
        1,
        0,
        neg_pi_bytes[0],
        neg_pi_bytes[1],
        neg_pi_bytes[2],
        neg_pi_bytes[3],
    ]);
    mix.mem[1003].set_all([
        0,
        0,
        pos_inf_bytes[0],
        pos_inf_bytes[1],
        pos_inf_bytes[2],
        pos_inf_bytes[3],
    ]);
    mix.mem[1004].set_all([
        1,
        0,
        neg_inf_bytes[0],
        neg_inf_bytes[1],
        neg_inf_bytes[2],
        neg_inf_bytes[3],
    ]);
    mix.mem[1005].set_all([0, 0, nan_bytes[0], nan_bytes[1], nan_bytes[2], nan_bytes[3]]);

    mix.restart();

    mix.step().unwrap();
    assert_eq!(mix.halted, false);

    mix.step().unwrap();
    assert_eq!(mix.halted, false);
    assert_eq!(mix.comp, CompIndicator::Less);

    mix.step().unwrap();
    assert_eq!(mix.halted, false);
    assert_eq!(mix.comp, CompIndicator::Greater);

    mix.step().unwrap();
    assert_eq!(mix.halted, false);
    assert_eq!(mix.comp, CompIndicator::Less);

    mix.step().unwrap();
    assert_eq!(mix.halted, false);
    assert_eq!(mix.comp, CompIndicator::Greater);

    mix.step().unwrap();
    assert_eq!(mix.halted, false);
    assert_eq!(mix.comp, CompIndicator::Unordered);
}

#[test]
fn test_ieee754_jmp_ord_unord() {
    let mut mix = MixVM::new();
    mix.reset();

    let pi_bytes = consts::PI.to_be_bytes();
    let e_bytes = consts::E.to_be_bytes();
    let nan_bytes = f32::NAN.to_be_bytes();

    mix.mem[0] = Instruction::new(1000, 5, 0, Opcode::LdA).into();
    mix.mem[1] = Instruction::new(1001, 7, 0, Opcode::CmpA).into();
    mix.mem[2] = Instruction::new(1000, 11, 0, Opcode::Jmp).into();
    mix.mem[3] = Instruction::new(10, 10, 0, Opcode::Jmp).into();
    mix.mem[10] = Instruction::new(1002, 5, 0, Opcode::LdA).into();
    mix.mem[11] = Instruction::new(1002, 7, 0, Opcode::CmpA).into();
    mix.mem[12] = Instruction::new(2000, 10, 0, Opcode::Jmp).into();
    mix.mem[13] = Instruction::new(0, 11, 0, Opcode::Jmp).into();
    mix.mem[1000].set_all([0, 0, pi_bytes[0], pi_bytes[1], pi_bytes[2], pi_bytes[3]]);
    mix.mem[1001].set_all([0, 0, e_bytes[0], e_bytes[1], e_bytes[2], e_bytes[3]]);
    mix.mem[1002].set_all([0, 0, nan_bytes[0], nan_bytes[1], nan_bytes[2], nan_bytes[3]]);

    mix.restart();

    mix.step().unwrap();
    assert_eq!(mix.halted, false);
    assert_eq!(mix.pc, 1);

    mix.step().unwrap();
    assert_eq!(mix.halted, false);
    assert_eq!(mix.pc, 2);

    mix.step().unwrap();
    assert_eq!(mix.halted, false);
    assert_eq!(mix.pc, 3);

    mix.step().unwrap();
    assert_eq!(mix.halted, false);
    assert_eq!(mix.pc, 10);

    mix.step().unwrap();
    assert_eq!(mix.halted, false);
    assert_eq!(mix.pc, 11);

    mix.step().unwrap();
    assert_eq!(mix.halted, false);
    assert_eq!(mix.pc, 12);

    mix.step().unwrap();
    assert_eq!(mix.halted, false);
    assert_eq!(mix.pc, 13);

    mix.step().unwrap();
    assert_eq!(mix.halted, false);
    assert_eq!(mix.pc, 0);
}

#[test]
fn test_ieee754_convert() {
    let mut mix = MixVM::new();
    mix.reset();

    let a = -1_234_456f32;
    let a_bytes = a.to_be_bytes();

    let a_32 = a.abs() as u32;
    let a_32_bytes = a_32.to_be_bytes();
    let a_16 = a.abs() as u16;
    let a_16_bytes = a_16.to_be_bytes();
    let a_8 = a.abs() as u8;
    let a_8_bytes = a_8.to_be_bytes();

    let expected_floats = [0x3456789A as f32, 0x789A as f32, 0x9A as f32];

    mix.mem[0] = Instruction::new(1000, 5, 0, Opcode::LdA).into();
    mix.mem[1] = Instruction::new(0, 3, 0, Opcode::Special).into();
    mix.mem[2] = Instruction::new(1000, 5, 0, Opcode::LdA).into();
    mix.mem[3] = Instruction::new(0, 4, 0, Opcode::Special).into();
    mix.mem[4] = Instruction::new(1000, 5, 0, Opcode::LdA).into();
    mix.mem[5] = Instruction::new(0, 5, 0, Opcode::Special).into();
    mix.mem[6] = Instruction::new(1001, 5, 0, Opcode::LdA).into();
    mix.mem[7] = Instruction::new(0, 6, 0, Opcode::Special).into();
    mix.mem[8] = Instruction::new(1001, 5, 0, Opcode::LdA).into();
    mix.mem[9] = Instruction::new(0, 7, 0, Opcode::Special).into();
    mix.mem[10] = Instruction::new(1001, 5, 0, Opcode::LdA).into();
    mix.mem[11] = Instruction::new(0, 8, 0, Opcode::Special).into();
    mix.mem[1000].set_all([1, 0, a_bytes[0], a_bytes[1], a_bytes[2], a_bytes[3]]);
    mix.mem[1001].set_all([1, 0x12, 0x34, 0x56, 0x78, 0x9A]);

    mix.restart();

    mix.step().unwrap();
    assert_eq!(mix.halted, false);
    mix.step().unwrap();
    assert_eq!(mix.halted, false);
    assert_eq!(
        mix.r_a[..],
        [
            1,
            0,
            a_32_bytes[0],
            a_32_bytes[1],
            a_32_bytes[2],
            a_32_bytes[3]
        ]
    );

    mix.step().unwrap();
    assert_eq!(mix.halted, false);
    mix.step().unwrap();
    assert_eq!(mix.halted, false);
    assert_eq!(mix.r_a[..], [1, 0, 0, 0, a_16_bytes[0], a_16_bytes[1]]);

    mix.step().unwrap();
    assert_eq!(mix.halted, false);
    mix.step().unwrap();
    assert_eq!(mix.halted, false);
    assert_eq!(mix.r_a[..], [1, 0, 0, 0, 0, a_8_bytes[0]]);

    for expected in expected_floats {
        let bytes = expected.to_be_bytes();

        mix.step().unwrap();
        assert_eq!(mix.halted, false);
        mix.step().unwrap();
        assert_eq!(mix.halted, false);
        assert_eq!(mix.r_a[..], [0, 0, bytes[0], bytes[1], bytes[2], bytes[3]]);
    }
}
