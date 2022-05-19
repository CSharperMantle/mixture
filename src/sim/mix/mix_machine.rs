use crate::sim::mix::instr::ToRangeInclusive;
use crate::sim::mix::*;

/// Error codes for the MIX machine.
#[derive(PartialEq, Eq, Debug)]
pub enum TrapCode {
    GeneralError,
    IllegalInstruction,
    InvalidAddress,
    InvalidField,
    MemAccessError,
    Halted,
}

/// The state of a MIX machine.
pub struct MixMachine {
    /// The register `rA`.
    pub r_a: reg::GenericRegister,
    /// The register `rX`.
    pub r_x: reg::GenericRegister,

    /// The register `rIn`, where `n = 1, 2, 3, 4, 5, 6`.
    pub r_in: [reg::IndexRegister; 6],

    /// The register `rJ`.
    pub r_j: reg::JumpRegister,

    /// The overflow toggle.
    pub toggle_overflow: bool,

    /// The comparison indicator.
    pub indicator_comp: reg::ComparisonIndicatorValue,

    /// The memory.
    pub mem: mem::Mem,

    /// The instruction pointer.
    pub pc: u16,

    /// The machine running state.
    pub halted: bool,
}

impl MixMachine {
    /// Create a new MIX machine.
    pub fn new() -> Self {
        MixMachine {
            r_a: reg::GenericRegister::new(),
            r_x: reg::GenericRegister::new(),
            r_in: [reg::IndexRegister::new(); 6],
            r_j: reg::JumpRegister::new(),
            toggle_overflow: false,
            indicator_comp: reg::ComparisonIndicatorValue::Equal,
            mem: mem::Mem::new(),
            pc: 0,
            halted: true,
        }
    }

    /// Reset the machine.
    ///
    /// This method resets the machine to its initial state,
    /// clearing the registers.
    ///
    pub fn reset(&mut self) {
        self.pc = 0;
        self.toggle_overflow = false;
        self.r_a = reg::GenericRegister::new();
        self.r_x = reg::GenericRegister::new();
        self.r_in = [reg::IndexRegister::new(); 6];
        self.r_j = reg::JumpRegister::new();
    }

    /// Restart the machine.
    ///
    /// This function un-halts the machine.
    pub fn restart(&mut self) {
        self.halted = false;
    }

    /// Run the next instruction of the machine.
    ///
    /// # Returns
    /// * `Ok(())` - The machine successfully completed its operation.
    /// * `Err(String)` - The machine encountered an error and is now halted.
    pub fn step(&mut self) -> Result<(), TrapCode> {
        if self.halted {
            return Err(TrapCode::Halted);
        }

        // Fetch the instruction.
        let instr: instr::Instruction = match self.mem[self.pc].try_into() {
            Ok(instr) => instr,
            Err(_) => {
                return Err(self.trap_illegal_instruction());
            }
        };

        self.pc += 1;

        // Run the instruction.
        match instr.opcode {
            instr::Opcode::Nop => self.handler_instr_nop(instr),

            instr::Opcode::Add => todo!(),
            instr::Opcode::Sub => todo!(),
            instr::Opcode::Mul => todo!(),
            instr::Opcode::Div => todo!(),

            instr::Opcode::Special => self.handler_instr_special(instr),
            instr::Opcode::Shift => todo!(),
            instr::Opcode::Move => self.handler_instr_move(instr),

            instr::Opcode::LdA => self.handler_instr_load_6b(instr),
            instr::Opcode::Ld1 => self.handler_instr_load_3b(instr),
            instr::Opcode::Ld2 => self.handler_instr_load_3b(instr),
            instr::Opcode::Ld3 => self.handler_instr_load_3b(instr),
            instr::Opcode::Ld4 => self.handler_instr_load_3b(instr),
            instr::Opcode::Ld5 => self.handler_instr_load_3b(instr),
            instr::Opcode::Ld6 => self.handler_instr_load_3b(instr),
            instr::Opcode::LdX => self.handler_instr_load_6b(instr),

            instr::Opcode::LdAN => self.handler_instr_load_neg_6b(instr),
            instr::Opcode::Ld1N => self.handler_instr_load_neg_3b(instr),
            instr::Opcode::Ld2N => self.handler_instr_load_neg_3b(instr),
            instr::Opcode::Ld3N => self.handler_instr_load_neg_3b(instr),
            instr::Opcode::Ld4N => self.handler_instr_load_neg_3b(instr),
            instr::Opcode::Ld5N => self.handler_instr_load_neg_3b(instr),
            instr::Opcode::Ld6N => self.handler_instr_load_neg_3b(instr),
            instr::Opcode::LdXN => self.handler_instr_load_neg_6b(instr),

            instr::Opcode::StA => self.handler_instr_store_6b(instr),
            instr::Opcode::St1 => self.handler_instr_store_3b(instr),
            instr::Opcode::St2 => self.handler_instr_store_3b(instr),
            instr::Opcode::St3 => self.handler_instr_store_3b(instr),
            instr::Opcode::St4 => self.handler_instr_store_3b(instr),
            instr::Opcode::St5 => self.handler_instr_store_3b(instr),
            instr::Opcode::St6 => self.handler_instr_store_3b(instr),
            instr::Opcode::StX => self.handler_instr_store_6b(instr),
            instr::Opcode::StJ => self.handler_instr_store_3b(instr),
            instr::Opcode::StZ => self.handler_instr_store_zero(instr),

            instr::Opcode::Jbus => todo!(),
            instr::Opcode::Ioc => todo!(),
            instr::Opcode::In => todo!(),
            instr::Opcode::Out => todo!(),
            instr::Opcode::Jred => todo!(),
            instr::Opcode::Jmp => self.handler_instr_jmp(instr),

            instr::Opcode::JA => todo!(),
            instr::Opcode::J1 => todo!(),
            instr::Opcode::J2 => todo!(),
            instr::Opcode::J3 => todo!(),
            instr::Opcode::J4 => todo!(),
            instr::Opcode::J5 => todo!(),
            instr::Opcode::J6 => todo!(),
            instr::Opcode::JX => todo!(),

            instr::Opcode::ModifyA => todo!(),
            instr::Opcode::Modify1 => todo!(),
            instr::Opcode::Modify2 => todo!(),
            instr::Opcode::Modify3 => todo!(),
            instr::Opcode::Modify4 => todo!(),
            instr::Opcode::Modify5 => todo!(),
            instr::Opcode::Modify6 => todo!(),
            instr::Opcode::ModifyX => todo!(),

            instr::Opcode::CmpA => todo!(),
            instr::Opcode::Cmp1 => todo!(),
            instr::Opcode::Cmp2 => todo!(),
            instr::Opcode::Cmp3 => todo!(),
            instr::Opcode::Cmp4 => todo!(),
            instr::Opcode::Cmp5 => todo!(),
            instr::Opcode::Cmp6 => todo!(),
            instr::Opcode::CmpX => todo!(),
        }?;

        Ok(())
    }

    /// Get indexed address.
    fn helper_get_eff_addr(&self, addr: i16, index: u8) -> Result<u16, TrapCode> {
        let eff_addr = if index == 0 {
            // Direct addressing.
            addr
        } else {
            // Indirect addressing.
            let index_sign = if self.r_in[index as usize - 1].is_positive() {
                1
            } else {
                -1
            };
            let index_value = &self.r_in[index as usize - 1];

            index_sign * i16::from_be_bytes([index_value[1], index_value[2]]) + addr
        };

        eff_addr.try_into().map_err(|_| TrapCode::InvalidAddress)
    }

    /// Handler for `NOP`.
    ///
    /// This function does nothing.
    fn handler_instr_nop(&mut self, _: instr::Instruction) -> Result<(), TrapCode> {
        // Do nothing.
        Ok(())
    }

    /// Handler for `LDA` and `LDX`.
    fn handler_instr_load_6b(&mut self, instr: instr::Instruction) -> Result<(), TrapCode> {
        // Obtain everything.
        let mut field = instr.field.to_range_inclusive();
        let memory_cell = &self.mem[self.helper_get_eff_addr(instr.addr, instr.index)?];
        let reg = match instr.opcode {
            instr::Opcode::LdA => &mut self.r_a,
            instr::Opcode::LdX => &mut self.r_x,
            _ => unreachable!(),
        };
        // Zero reg before copying. Handle 'understood' positive sign too.
        reg.set(0..=5, &[1, 0, 0, 0, 0, 0])
            .map_err(|_| TrapCode::MemAccessError)?;
        // Do we need to update the sign byte?
        let sign_copy_needed = *field.start() == 0;
        if sign_copy_needed {
            // Treat sign bit specially by moving it out.
            field = (*field.start() + 1)..=(*field.end());
        }
        // Copy bytes shifted right.
        for (memory_cell_cursor, reg_cursor) in field.rev().zip((1..=5).rev()) {
            reg[reg_cursor] = memory_cell[memory_cell_cursor];
        }
        // Copy sign byte if needed.
        if sign_copy_needed {
            reg[0] = memory_cell[0];
        }
        Ok(())
    }

    /// Handler for `LDAN` and `LDXN`.
    fn handler_instr_load_neg_6b(&mut self, instr: instr::Instruction) -> Result<(), TrapCode> {
        // Obtain everything.
        let mut field = instr.field.to_range_inclusive();
        let memory_cell = &self.mem[self.helper_get_eff_addr(instr.addr, instr.index)?];
        let reg = match instr.opcode {
            instr::Opcode::LdAN => &mut self.r_a,
            instr::Opcode::LdXN => &mut self.r_x,
            _ => unreachable!(),
        };
        // Zero reg before copying. Handle 'understood' negative sign.
        reg.set(0..=5, &[0, 0, 0, 0, 0, 0])
            .map_err(|_| TrapCode::MemAccessError)?;
        // Do we need to update the sign byte?
        let sign_copy_needed = *field.start() == 0;
        if sign_copy_needed {
            // Treat sign bit specially by moving it out.
            field = (*field.start() + 1)..=(*field.end());
        }
        // Copy bytes shifted right.
        for (memory_cell_cursor, reg_cursor) in field.rev().zip((1..=5).rev()) {
            reg[reg_cursor] = memory_cell[memory_cell_cursor];
        }
        // Copy negated sign byte if needed.
        if sign_copy_needed {
            reg[0] = if memory_cell[0] == 0 { 1 } else { 0 };
        }
        Ok(())
    }

    /// Handler for `LD1-6`.
    ///
    /// Note that this instruction only sets the first sign, 4th
    /// and 5th bits of the original memory location. This prevents
    /// the said 'undefined behavior' from happening.
    fn handler_instr_load_3b(&mut self, instr: instr::Instruction) -> Result<(), TrapCode> {
        // Obtain everything.
        let mut field = instr.field.to_range_inclusive();
        let memory_cell = &self.mem[self.helper_get_eff_addr(instr.addr, instr.index)?];
        let reg = match instr.opcode {
            instr::Opcode::Ld1 => &mut self.r_in[0],
            instr::Opcode::Ld2 => &mut self.r_in[1],
            instr::Opcode::Ld3 => &mut self.r_in[2],
            instr::Opcode::Ld4 => &mut self.r_in[3],
            instr::Opcode::Ld5 => &mut self.r_in[4],
            instr::Opcode::Ld6 => &mut self.r_in[5],
            _ => unreachable!(),
        };
        // We need to care about only the 4th, 5th and the sign byte.
        // So we make a temporary word and fill back the reg only the
        // 4th, 5th and the sign byte. Handle 'understood' positive sign.
        let mut temp = mem::Word::<6, false>::new();
        temp.set(0..=2, &[1, 0, 0])
            .map_err(|_| TrapCode::MemAccessError)?;
        // Do we need to update the sign byte?
        let sign_copy_needed = *field.start() == 0;
        if sign_copy_needed {
            // Treat sign bit specially by moving it out.
            field = (*field.start() + 1)..=(*field.end());
        }
        // Copy bytes shifted right.
        for (memory_cell_cursor, reg_cursor) in field.rev().zip((1..=5).rev()) {
            temp[reg_cursor] = memory_cell[memory_cell_cursor];
        }
        // Copy sign byte if needed.
        if sign_copy_needed {
            temp[0] = memory_cell[0];
        }
        // Fill back the reg.
        reg[0] = temp[0];
        reg[1] = temp[4];
        reg[2] = temp[5];
        Ok(())
    }

    /// Handler for `LD1-6N`.
    ///
    /// Note that this instruction only sets the first sign, 4th
    /// and 5th bits of the original memory location. This prevents
    /// the said 'undefined behavior' from happening.
    fn handler_instr_load_neg_3b(&mut self, instr: instr::Instruction) -> Result<(), TrapCode> {
        // Obtain everything.
        let mut field = instr.field.to_range_inclusive();
        let memory_cell = &self.mem[self.helper_get_eff_addr(instr.addr, instr.index)?];
        let reg = match instr.opcode {
            instr::Opcode::Ld1N => &mut self.r_in[0],
            instr::Opcode::Ld2N => &mut self.r_in[1],
            instr::Opcode::Ld3N => &mut self.r_in[2],
            instr::Opcode::Ld4N => &mut self.r_in[3],
            instr::Opcode::Ld5N => &mut self.r_in[4],
            instr::Opcode::Ld6N => &mut self.r_in[5],
            _ => unreachable!(),
        };
        // We need to care about only the 4th, 5th and the sign byte.
        // So we make a temporary word and fill back the reg only the
        // 4th, 5th and the sign byte. Handle 'understood' positive sign.
        let mut temp = mem::Word::<6, false>::new();
        temp.set(0..=2, &[0, 0, 0])
            .map_err(|_| TrapCode::MemAccessError)?;
        // Do we need to update the sign byte?
        let sign_copy_needed = *field.start() == 0;
        if sign_copy_needed {
            // Treat sign bit specially by moving it out.
            field = (*field.start() + 1)..=(*field.end());
        }

        // Copy bytes shifted right.
        for (memory_cell_cursor, reg_cursor) in field.rev().zip((1..=5).rev()) {
            temp[reg_cursor] = memory_cell[memory_cell_cursor];
        }
        // Copy negated sign byte if needed.
        if sign_copy_needed {
            temp[0] = if memory_cell[0] == 0 { 1 } else { 0 };
        }
        // Fill back the reg.
        reg[0] = temp[0];
        reg[1] = temp[4];
        reg[2] = temp[5];
        Ok(())
    }

    /// Handler for `JMP` and variants.
    fn handler_instr_jmp(&mut self, instr: instr::Instruction) -> Result<(), TrapCode> {
        let target_addr = self.helper_get_eff_addr(instr.addr, instr.index)?;
        // Match jump conditions.
        let should_jump = match instr.field {
            0 | 1 => true,
            2 => self.toggle_overflow,
            3 => !self.toggle_overflow,
            4 => self.indicator_comp == reg::ComparisonIndicatorValue::Lesser,
            5 => self.indicator_comp == reg::ComparisonIndicatorValue::Equal,
            6 => self.indicator_comp == reg::ComparisonIndicatorValue::Greater,
            7 => self.indicator_comp != reg::ComparisonIndicatorValue::Lesser,
            8 => self.indicator_comp != reg::ComparisonIndicatorValue::Equal,
            9 => self.indicator_comp != reg::ComparisonIndicatorValue::Greater,
            _ => return Err(TrapCode::InvalidField),
        };

        // Clear overflow flag.
        if instr.field == 2 || instr.field == 3 {
            self.toggle_overflow = false;
        }

        if should_jump {
            // Save PC in rJ.
            if instr.field != 1 {
                let pc_unpacked = (self.pc as u16).to_be_bytes();
                self.r_j
                    .set(1..=2, &pc_unpacked)
                    .map_err(|_| TrapCode::MemAccessError)?;
            }
            println!("{:?}", self.pc);
            // Do jump.
            self.pc = target_addr;
        }
        Ok(())
    }

    /// Handler for `CHAR`, `NUM` and `HLT`.
    fn handler_instr_special(&mut self, instr: instr::Instruction) -> Result<(), TrapCode> {
        if instr.field == 0 {
            // NUM instruction
            let a_content = &self.r_a[1..=5];
            let x_content = &self.r_x[1..=5];
            let mut result: u32 = 0;
            // For each byte, we extract its 1st position,
            // and push it to `result`.
            for byte in a_content.iter().chain(x_content.iter()) {
                let digit = *byte % 10;
                result = result * 10 + digit as u32;
            }
            // Rebuild a word of 4 bytes.
            let result_bytes = result.to_be_bytes();
            self.r_a
                .set(2..=5, &result_bytes)
                .map_err(|_| TrapCode::MemAccessError)?;
            Ok(())
        } else if instr.field == 1 {
            // CHAR instruction
            let a_content = &self.r_a[2..=5];
            // Obtain original number.
            let mut source =
                u32::from_be_bytes([a_content[0], a_content[1], a_content[2], a_content[3]]);
            let mut cursor = 9;
            let mut bytes: [u8; 10] = [0; 10];
            // Extract each digit of `source` and pack them to a
            // 10's multiplier.
            while source > 0 {
                let digit = source % 10;
                source /= 10;
                let byte = (30 + digit) as u8;
                bytes[cursor] = byte;
                cursor -= 1;
            }
            // Store them back.
            for i in 0..5 {
                self.r_a[i + 1] = bytes[i];
            }
            for i in 5..10 {
                self.r_x[i - 5 + 1] = bytes[i];
            }
            Ok(())
        } else if instr.field == 2 {
            // HLT instruction
            // Making it just like NOP if we restart the
            // machine later.
            self.halted = true;
            Ok(())
        } else {
            return Err(TrapCode::InvalidField);
        }
    }

    /// Handler for `STZ`.
    fn handler_instr_store_zero(&mut self, instr: instr::Instruction) -> Result<(), TrapCode> {
        // Obtain everything.
        let addr = self.helper_get_eff_addr(instr.addr, instr.index)?;
        let field = instr.field.to_range_inclusive();
        let memory_cell = &mut self.mem[addr];
        let start = *field.start();
        // Zero the memory cell.
        for i in field {
            memory_cell[i] = 0;
        }
        // Deal with signs.
        if start == 0 {
            memory_cell[0] = 1;
        }
        Ok(())
    }

    /// Handler for `MOVE`.
    fn handler_instr_move(&mut self, instr: instr::Instruction) -> Result<(), TrapCode> {
        // Obtain from address.
        let from_addr = self.helper_get_eff_addr(instr.addr, instr.index)?;
        // Obtain to address.
        let to_addr = u16::from_be_bytes([self.r_in[0][1], self.r_in[0][2]]);
        let num_words = instr.field;
        // Move each word.
        for i in 0..num_words {
            let orig_mem = self.mem[from_addr + i as u16];
            self.mem[to_addr + i as u16]
                .set(0..=5, &orig_mem[0..=5])
                .map_err(|_| TrapCode::MemAccessError)?;
        }
        Ok(())
    }

    /// Handler for `STA` and `STX`.
    fn handler_instr_store_6b(&mut self, instr: instr::Instruction) -> Result<(), TrapCode> {
        // Obtain everything.
        let mut field = instr.field.to_range_inclusive();
        let addr = self.helper_get_eff_addr(instr.addr, instr.index)?;
        let memory_cell = &mut self.mem[addr];
        let reg = match instr.opcode {
            instr::Opcode::StA => &self.r_a,
            instr::Opcode::StX => &self.r_x,
            _ => unreachable!(),
        };
        let sign_copy_needed = *field.start() == 0;
        if sign_copy_needed {
            // Treat sign bit specially by moving it out.
            field = (*field.start() + 1)..=(*field.end());
        }
        // Copy bytes shifted right.
        for (memory_cell_cursor, reg_cursor) in field.rev().zip((1..=5).rev()) {
            memory_cell[memory_cell_cursor] = reg[reg_cursor];
        }
        if sign_copy_needed {
            // Copy sign bit.
            memory_cell[0] = reg[0];
        }
        Ok(())
    }

    /// Handler for `ST1-6`.
    fn handler_instr_store_3b(&mut self, instr: instr::Instruction) -> Result<(), TrapCode> {
        // Obtain everything.
        let mut field = instr.field.to_range_inclusive();
        let addr = self.helper_get_eff_addr(instr.addr, instr.index)?;
        let memory_cell = &mut self.mem[addr];
        let reg = match instr.opcode {
            instr::Opcode::St1 => &self.r_in[0],
            instr::Opcode::St2 => &self.r_in[1],
            instr::Opcode::St3 => &self.r_in[2],
            instr::Opcode::St4 => &self.r_in[3],
            instr::Opcode::St5 => &self.r_in[4],
            instr::Opcode::St6 => &self.r_in[5],
            _ => unreachable!(),
        };
        let padded_reg = [reg[0], 0, 0, 0, reg[1], reg[2]];
        let sign_copy_needed = *field.start() == 0;
        if sign_copy_needed {
            // Treat sign bit specially by moving it out.
            field = (*field.start() + 1)..=(*field.end());
        }
        // Copy bytes shifted right.
        for (memory_cell_cursor, reg_cursor) in field.rev().zip((1..=5).rev()) {
            memory_cell[memory_cell_cursor] = padded_reg[reg_cursor];
        }
        if sign_copy_needed {
            // Copy sign bit.
            memory_cell[0] = padded_reg[0];
        }
        Ok(())
    }

    /// Trap handler for illegal instructions.
    ///
    /// This function is called when an illegal instruction is
    /// encountered. It halts the machine and prints the
    /// offending address.
    fn trap_illegal_instruction(&mut self) -> TrapCode {
        self.halted = true;
        TrapCode::IllegalInstruction
    }
}

#[cfg(test)]
mod tests {
    use crate::sim::mix::instr::*;
    use crate::sim::mix::mix_machine::*;

    #[test]
    fn test_reset_restart() {
        let mut mix = MixMachine::new();

        mix.halted = true;
        mix.pc = 123;
        mix.toggle_overflow = true;

        mix.reset();

        assert_eq!(mix.halted, true);
        assert_eq!(mix.pc, 0);
        assert_eq!(mix.toggle_overflow, false);

        mix.restart();

        assert_eq!(mix.halted, false);
        assert_eq!(mix.pc, 0);
        assert_eq!(mix.toggle_overflow, false);
    }

    #[test]
    fn test_simple_load_6b() {
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
        mix.mem[2000].set(0..=5, &[0, 0, 80, 3, 5, 4]).unwrap();

        mix.restart();

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
        mix.mem[2000].set(0..=5, &[0, 0, 80, 3, 5, 4]).unwrap();
        mix.r_in[0].set(0..=2, &[1, 0x03, 0xE8]).unwrap();
        mix.r_in[1].set(0..=2, &[0, 0x03, 0xE8]).unwrap();

        mix.restart();

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

    #[test]
    fn test_simple_special() {
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
        assert_eq!(mix.r_a[0..=5], [0, 0, 0, 31, 32, 39]);
        assert_eq!(mix.r_x[0..=5], [1, 37, 37, 37, 30, 30]);

        mix.step().unwrap();
        assert_eq!(mix.halted, true);
    }

    #[test]
    fn test_simple_store_zero() {
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
    fn test_simple_move() {
        let mut mix = MixMachine::new();
        mix.reset();

        mix.mem[0] = Instruction::new(1000, 3, 0, Opcode::Move)
            .try_into()
            .unwrap();

        mix.r_in[0].set(1..=2, &[0x03, 0xE7]).unwrap();
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
    fn test_simple_store_6b() {
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
    fn test_simple_store_3b() {
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

        mix.r_in[0].set(0..=2, &[1, 6, 7]).unwrap();
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
}
