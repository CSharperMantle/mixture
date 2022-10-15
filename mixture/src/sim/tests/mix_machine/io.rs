use std::prelude::v1::*;

use crate::common::*;
use crate::sim::io::*;
use crate::sim::mix_machine::*;

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

struct ErrorIODevice {}

impl IODevice for ErrorIODevice {
    fn read(&mut self, _: &mut [FullWord]) -> Result<(), ()> {
        Err(())
    }

    fn write(&mut self, _: &[FullWord]) -> Result<(), usize> {
        Err(0)
    }

    fn control(&mut self, _: i16) -> Result<(), ()> {
        Err(())
    }

    fn is_busy(&self) -> Result<bool, ()> {
        Err(())
    }

    fn is_ready(&self) -> Result<bool, ()> {
        Err(())
    }

    fn get_block_size(&self) -> usize {
        0
    }
}

#[test]
fn test_io_device_error() {
    let dev_err = ErrorIODevice {};

    let mut mix = MixMachine::new();
    mix.reset();

    mix.io_devices[0] = Some(Box::new(dev_err));

    mix.mem[0] = Instruction::new(1000, 0, 0, Opcode::In).try_into().unwrap();

    mix.restart();

    let err = mix.step().expect_err("Expect error");
    assert_eq!(err, ErrorCode::IOError);
    assert_eq!(mix.halted, true);
}

struct BusyIODevice {}

impl IODevice for BusyIODevice {
    fn read(&mut self, _: &mut [FullWord]) -> Result<(), ()> {
        unimplemented!()
    }

    fn write(&mut self, _: &[FullWord]) -> Result<(), usize> {
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
    fn read(&mut self, _: &mut [FullWord]) -> Result<(), ()> {
        unimplemented!()
    }

    fn write(&mut self, _: &[FullWord]) -> Result<(), usize> {
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
    fn read(&mut self, _: &mut [FullWord]) -> Result<(), ()> {
        unimplemented!()
    }

    fn write(&mut self, _: &[FullWord]) -> Result<(), usize> {
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
    fn read(&mut self, buffer: &mut [FullWord]) -> Result<(), ()> {
        let mut w = FullWord::new();
        w.set(0..=5, &[0, 9, 8, 7, 6, 5])?;
        buffer[0] = w;
        Ok(())
    }

    fn write(&mut self, data: &[FullWord]) -> Result<(), usize> {
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
