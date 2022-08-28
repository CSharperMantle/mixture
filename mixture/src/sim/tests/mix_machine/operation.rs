use crate::sim::mix_machine::*;

#[test]
fn test_reset_restart() {
    let mut mix = MixMachine::new();

    mix.halted = true;
    mix.pc = 123;
    mix.overflow = true;

    mix.reset();

    assert_eq!(mix.halted, true);
    assert_eq!(mix.pc, 0);
    assert_eq!(mix.overflow, false);

    mix.restart();

    assert_eq!(mix.halted, false);
    assert_eq!(mix.pc, 0);
    assert_eq!(mix.overflow, false);
}
