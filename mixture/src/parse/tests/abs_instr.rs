use crate::parse::abs_instr::*;
use crate::parse::maybe::*;
use crate::common::*;

#[test]
fn test_concretize() {
    let abs_instr_1 = AbstractInstruction {
        addr: Maybe::<i16, i32>::Concrete(2000),
        field: Maybe::<u8, i32>::Concrete(3),
        index: Maybe::<u8, i32>::Concrete(2),
        opcode: Opcode::LdA,
    };

    let instr = abs_instr_1.concretize().unwrap();
    assert_eq!(instr.opcode, Opcode::LdA);
    assert_eq!(instr.field, 3);
    assert_eq!(instr.index, 2);
    assert_eq!(instr.addr, 2000);

    let abs_instr_2 = AbstractInstruction {
        addr: Maybe::<i16, i32>::Placeholder(1),
        field: Maybe::<u8, i32>::Placeholder(2),
        index: Maybe::<u8, i32>::Placeholder(3),
        opcode: Opcode::LdA,
    };

    assert_eq!(abs_instr_2.concretize().is_err(), true);
}
