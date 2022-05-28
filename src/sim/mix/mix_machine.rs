use crate::sim::mix::instr::ToRangeInclusive;
use crate::sim::mix::*;

/// A internal shortcut to a 6-byte Word.
type FullWord = mem::Word<6, false>;

/// A internal shortcut to a 3-byte Word.
type HalfWord = mem::Word<3, false>;

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
    /// `r_in[0]` should always used as a source of 0.
    pub r_in: [reg::IndexRegister; 7],

    /// The register `rJ`.
    pub r_j: reg::JumpRegister,

    /// The overflow toggle.
    pub overflow: bool,

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
            r_in: [reg::IndexRegister::new(); 7],
            r_j: reg::JumpRegister::new(),
            overflow: false,
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
        self.overflow = false;
        self.r_a = reg::GenericRegister::new();
        self.r_x = reg::GenericRegister::new();
        self.r_in = [reg::IndexRegister::new(); 7];
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
            instr::Opcode::Nop => self.handler_instr_nop(&instr),

            instr::Opcode::Add => self.handler_instr_add_sub(&instr),
            instr::Opcode::Sub => self.handler_instr_add_sub(&instr),
            instr::Opcode::Mul => self.handler_instr_mul(&instr),
            instr::Opcode::Div => todo!(),

            instr::Opcode::Special => self.handler_instr_special(&instr),
            instr::Opcode::Shift => todo!(),
            instr::Opcode::Move => self.handler_instr_move(&instr),

            instr::Opcode::LdA => self.handler_instr_load_6b(&instr),
            instr::Opcode::Ld1 => self.handler_instr_load_3b(&instr),
            instr::Opcode::Ld2 => self.handler_instr_load_3b(&instr),
            instr::Opcode::Ld3 => self.handler_instr_load_3b(&instr),
            instr::Opcode::Ld4 => self.handler_instr_load_3b(&instr),
            instr::Opcode::Ld5 => self.handler_instr_load_3b(&instr),
            instr::Opcode::Ld6 => self.handler_instr_load_3b(&instr),
            instr::Opcode::LdX => self.handler_instr_load_6b(&instr),

            instr::Opcode::LdAN => self.handler_instr_load_neg_6b(&instr),
            instr::Opcode::Ld1N => self.handler_instr_load_neg_3b(&instr),
            instr::Opcode::Ld2N => self.handler_instr_load_neg_3b(&instr),
            instr::Opcode::Ld3N => self.handler_instr_load_neg_3b(&instr),
            instr::Opcode::Ld4N => self.handler_instr_load_neg_3b(&instr),
            instr::Opcode::Ld5N => self.handler_instr_load_neg_3b(&instr),
            instr::Opcode::Ld6N => self.handler_instr_load_neg_3b(&instr),
            instr::Opcode::LdXN => self.handler_instr_load_neg_6b(&instr),

            instr::Opcode::StA => self.handler_instr_store_6b(&instr),
            instr::Opcode::St1 => self.handler_instr_store_3b(&instr),
            instr::Opcode::St2 => self.handler_instr_store_3b(&instr),
            instr::Opcode::St3 => self.handler_instr_store_3b(&instr),
            instr::Opcode::St4 => self.handler_instr_store_3b(&instr),
            instr::Opcode::St5 => self.handler_instr_store_3b(&instr),
            instr::Opcode::St6 => self.handler_instr_store_3b(&instr),
            instr::Opcode::StX => self.handler_instr_store_6b(&instr),
            instr::Opcode::StJ => self.handler_instr_store_3b(&instr),
            instr::Opcode::StZ => self.handler_instr_store_zero(&instr),

            instr::Opcode::Jbus => todo!(),
            instr::Opcode::Ioc => todo!(),
            instr::Opcode::In => todo!(),
            instr::Opcode::Out => todo!(),
            instr::Opcode::Jred => todo!(),
            instr::Opcode::Jmp => self.handler_instr_jmp(&instr),

            instr::Opcode::JA => todo!(),
            instr::Opcode::J1 => todo!(),
            instr::Opcode::J2 => todo!(),
            instr::Opcode::J3 => todo!(),
            instr::Opcode::J4 => todo!(),
            instr::Opcode::J5 => todo!(),
            instr::Opcode::J6 => todo!(),
            instr::Opcode::JX => todo!(),

            instr::Opcode::ModifyA => self.handler_instr_modify_6b(&instr),
            instr::Opcode::Modify1 => self.handler_instr_modify_3b(&instr),
            instr::Opcode::Modify2 => self.handler_instr_modify_3b(&instr),
            instr::Opcode::Modify3 => self.handler_instr_modify_3b(&instr),
            instr::Opcode::Modify4 => self.handler_instr_modify_3b(&instr),
            instr::Opcode::Modify5 => self.handler_instr_modify_3b(&instr),
            instr::Opcode::Modify6 => self.handler_instr_modify_3b(&instr),
            instr::Opcode::ModifyX => self.handler_instr_modify_6b(&instr),

            instr::Opcode::CmpA => self.handler_instr_cmp_6b(&instr),
            instr::Opcode::Cmp1 => self.handler_instr_cmp_3b(&instr),
            instr::Opcode::Cmp2 => self.handler_instr_cmp_3b(&instr),
            instr::Opcode::Cmp3 => self.handler_instr_cmp_3b(&instr),
            instr::Opcode::Cmp4 => self.handler_instr_cmp_3b(&instr),
            instr::Opcode::Cmp5 => self.handler_instr_cmp_3b(&instr),
            instr::Opcode::Cmp6 => self.handler_instr_cmp_3b(&instr),
            instr::Opcode::CmpX => self.handler_instr_cmp_6b(&instr),
        }?;

        Ok(())
    }

    /// Get indexed address.
    fn helper_get_eff_addr(&self, addr: i16, index: u8) -> Result<u16, TrapCode> {
        // Direct or indirect addressing.
        // r_in[0] is always zero.
        let reg = self.r_in[index as usize];
        let (reg_val, _) = reg.to_i64();
        (reg_val + addr as i64)
            .try_into()
            .map_err(|_| TrapCode::InvalidAddress)
    }

    /// Get indexed address. May panic or return negative value.
    fn helper_get_eff_addr_unchecked(&self, addr: i16, index: u8) -> i16 {
        let reg = self.r_in[index as usize];
        let (reg_val, _) = reg.to_i64();
        reg_val as i16 + addr
    }

    /// Handler for `NOP`.
    ///
    /// This function does nothing.
    fn handler_instr_nop(&mut self, _: &instr::Instruction) -> Result<(), TrapCode> {
        // Do nothing.
        Ok(())
    }

    /// Handler for `LDA` and `LDX`.
    fn handler_instr_load_6b(&mut self, instr: &instr::Instruction) -> Result<(), TrapCode> {
        // Obtain everything.
        let (field, sign_copy_needed) = instr.field.to_range_inclusive_signless();
        let memory_cell = &self.mem[self.helper_get_eff_addr(instr.addr, instr.index)?];
        let reg = match instr.opcode {
            instr::Opcode::LdA => &mut self.r_a,
            instr::Opcode::LdX => &mut self.r_x,
            _ => unreachable!(),
        };
        // Zero reg before copying. Handle 'understood' positive sign too.
        reg.set(0..=5, &[FullWord::POS, 0, 0, 0, 0, 0])
            .map_err(|_| TrapCode::MemAccessError)?;
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
    fn handler_instr_load_neg_6b(&mut self, instr: &instr::Instruction) -> Result<(), TrapCode> {
        // Obtain everything.
        let (field, sign_copy_needed) = instr.field.to_range_inclusive_signless();
        let memory_cell = &self.mem[self.helper_get_eff_addr(instr.addr, instr.index)?];
        let reg = match instr.opcode {
            instr::Opcode::LdAN => &mut self.r_a,
            instr::Opcode::LdXN => &mut self.r_x,
            _ => unreachable!(),
        };
        // Zero reg before copying. Handle 'understood' negative sign.
        reg.set(0..=5, &[0, 0, 0, 0, 0, 0])
            .map_err(|_| TrapCode::MemAccessError)?;
        // Copy bytes shifted right.
        for (memory_cell_cursor, reg_cursor) in field.rev().zip((1..=5).rev()) {
            reg[reg_cursor] = memory_cell[memory_cell_cursor];
        }
        // Copy negated sign byte if needed.
        if sign_copy_needed {
            reg[0] = if memory_cell.is_positive() {
                FullWord::NEG
            } else {
                FullWord::POS
            };
        }
        Ok(())
    }

    /// Handler for `LD1-6`.
    ///
    /// Note that this instruction only sets the first sign, 4th
    /// and 5th bits of the original memory location. This prevents
    /// the said 'undefined behavior' from happening.
    fn handler_instr_load_3b(&mut self, instr: &instr::Instruction) -> Result<(), TrapCode> {
        // Obtain everything.
        let (field, sign_copy_needed) = instr.field.to_range_inclusive_signless();
        let memory_cell = &self.mem[self.helper_get_eff_addr(instr.addr, instr.index)?];
        let reg = match instr.opcode {
            instr::Opcode::Ld1 => &mut self.r_in[1],
            instr::Opcode::Ld2 => &mut self.r_in[2],
            instr::Opcode::Ld3 => &mut self.r_in[3],
            instr::Opcode::Ld4 => &mut self.r_in[4],
            instr::Opcode::Ld5 => &mut self.r_in[5],
            instr::Opcode::Ld6 => &mut self.r_in[6],
            _ => unreachable!(),
        };
        // We need to care about only the 4th, 5th and the sign byte.
        // So we make a temporary word and fill back the reg only the
        // 4th, 5th and the sign byte. Handle 'understood' positive sign.
        let mut temp = FullWord::from_bytes([1, 0, 0, 0, 0, 0]);
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
    fn handler_instr_load_neg_3b(&mut self, instr: &instr::Instruction) -> Result<(), TrapCode> {
        // Obtain everything.
        let (field, sign_copy_needed) = instr.field.to_range_inclusive_signless();
        let memory_cell = &self.mem[self.helper_get_eff_addr(instr.addr, instr.index)?];
        let reg = match instr.opcode {
            instr::Opcode::Ld1N => &mut self.r_in[1],
            instr::Opcode::Ld2N => &mut self.r_in[2],
            instr::Opcode::Ld3N => &mut self.r_in[3],
            instr::Opcode::Ld4N => &mut self.r_in[4],
            instr::Opcode::Ld5N => &mut self.r_in[5],
            instr::Opcode::Ld6N => &mut self.r_in[6],
            _ => unreachable!(),
        };
        // We need to care about only the 4th, 5th and the sign byte.
        // So we make a temporary word and fill back the reg only the
        // 4th, 5th and the sign byte. Handle 'understood' positive sign.
        let mut temp = FullWord::from_bytes([0, 0, 0, 0, 0, 0]);
        // Copy bytes shifted right.
        for (memory_cell_cursor, reg_cursor) in field.rev().zip((1..=5).rev()) {
            temp[reg_cursor] = memory_cell[memory_cell_cursor];
        }
        // Copy negated sign byte if needed.
        if sign_copy_needed {
            temp[0] = if memory_cell.is_positive() {
                FullWord::NEG
            } else {
                FullWord::POS
            };
        }
        // Fill back the reg.
        reg[0] = temp[0];
        reg[1] = temp[4];
        reg[2] = temp[5];
        Ok(())
    }

    /// Handler for `JMP` and variants.
    fn handler_instr_jmp(&mut self, instr: &instr::Instruction) -> Result<(), TrapCode> {
        let target_addr = self.helper_get_eff_addr(instr.addr, instr.index)?;
        // Match jump conditions.
        let should_jump = match instr.field {
            0 | 1 => true,
            2 => self.overflow,
            3 => !self.overflow,
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
            self.overflow = false;
        }

        if should_jump {
            // Save PC in rJ.
            if instr.field != 1 {
                let pc_unpacked = (self.pc as u16).to_be_bytes();
                self.r_j
                    .set(1..=2, &pc_unpacked)
                    .map_err(|_| TrapCode::MemAccessError)?;
            }
            // Do jump.
            self.pc = target_addr;
        }
        Ok(())
    }

    /// Handler for `CHAR`, `NUM` and `HLT`.
    fn handler_instr_special(&mut self, instr: &instr::Instruction) -> Result<(), TrapCode> {
        if instr.field == 0 {
            // NUM instruction
            let a_content = &self.r_a[1..=5];
            let x_content = &self.r_x[1..=5];
            let mut result: i64 = 0;
            // For each byte, we extract its 1st position,
            // and push it to `result`.
            for byte in a_content.iter().chain(x_content.iter()) {
                let digit = *byte % 10;
                result = result * 10 + digit as i64;
            }
            // Rebuild a word of 4 bytes.
            let (result_word, _) = FullWord::from_i64(result);
            self.r_a
                .set(0..=5, &result_word[0..=5])
                .map_err(|_| TrapCode::MemAccessError)?;
            return Ok(());
        } else if instr.field == 1 {
            // CHAR instruction
            // Obtain original number.
            let (source, _) = self.r_a.to_i64();
            let digits = source
                .abs()
                .to_string()
                .chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<u32>>();
            for (reg_i, digit) in (0..10).rev().zip(digits.iter().rev()) {
                if reg_i >= 5 {
                    self.r_x[reg_i - 5 + 1] = *digit as u8 + 30;
                } else {
                    self.r_a[reg_i + 1] = *digit as u8 + 30;
                }
            }
            return Ok(());
        } else if instr.field == 2 {
            // HLT instruction
            // Making it just like NOP if we restart the
            // machine later.
            self.halted = true;
            return Ok(());
        } else {
            return Err(TrapCode::InvalidField);
        }
    }

    /// Handler for `STZ`.
    fn handler_instr_store_zero(&mut self, instr: &instr::Instruction) -> Result<(), TrapCode> {
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
    fn handler_instr_move(&mut self, instr: &instr::Instruction) -> Result<(), TrapCode> {
        // Obtain from address.
        let from_addr = self.helper_get_eff_addr(instr.addr, instr.index)?;
        // Obtain to address.
        let to_addr = u16::from_be_bytes([self.r_in[1][1], self.r_in[1][2]]);
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
    fn handler_instr_store_6b(&mut self, instr: &instr::Instruction) -> Result<(), TrapCode> {
        // Obtain everything.
        let (field, sign_copy_needed) = instr.field.to_range_inclusive_signless();
        let addr = self.helper_get_eff_addr(instr.addr, instr.index)?;
        let memory_cell = &mut self.mem[addr];
        let reg = match instr.opcode {
            instr::Opcode::StA => &self.r_a,
            instr::Opcode::StX => &self.r_x,
            _ => unreachable!(),
        };
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
    fn handler_instr_store_3b(&mut self, instr: &instr::Instruction) -> Result<(), TrapCode> {
        // Obtain everything.
        let (field, sign_copy_needed) = instr.field.to_range_inclusive_signless();
        let addr = self.helper_get_eff_addr(instr.addr, instr.index)?;
        let memory_cell = &mut self.mem[addr];
        let reg = match instr.opcode {
            instr::Opcode::St1 => &self.r_in[1],
            instr::Opcode::St2 => &self.r_in[2],
            instr::Opcode::St3 => &self.r_in[3],
            instr::Opcode::St4 => &self.r_in[4],
            instr::Opcode::St5 => &self.r_in[5],
            instr::Opcode::St6 => &self.r_in[6],
            _ => unreachable!(),
        };
        let padded_reg = [reg[0], 0, 0, 0, reg[1], reg[2]];
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

    /// Handler for `INCA`, `DECA`, `ENTA`, `ENNA`, `INCX`,
    /// `DECX`, `ENTX` and `ENNX`.
    fn handler_instr_modify_6b(&mut self, instr: &instr::Instruction) -> Result<(), TrapCode> {
        let addr = self.helper_get_eff_addr(instr.addr, instr.index)?;
        let reg = match instr.opcode {
            instr::Opcode::ModifyA => &mut self.r_a,
            instr::Opcode::ModifyX => &mut self.r_x,
            _ => unreachable!(),
        };

        if instr.field == 0 || instr.field == 1 {
            // INCx and DECx
            // Add or subtract one.
            let addr = addr as i64;
            let offset = if instr.field == 0 { addr } else { -addr };
            // Convert to i64.
            let value = reg.to_i64().0;
            // Convert back modified value.
            let (new_word, overflow) = FullWord::from_i64(value + offset);
            reg.set(0..=5, &new_word[0..=5])
                .map_err(|_| TrapCode::MemAccessError)?;
            // Should we overflow?
            if overflow {
                self.overflow = overflow;
            }
            return Ok(());
        } else if instr.field == 2 || instr.field == 3 {
            // ENTx and ENNx
            let (new_word, _) = FullWord::from_i64(addr as i64);
            // Copy new word into reg.
            reg.set(0..=5, &new_word[0..=5])
                .map_err(|_| TrapCode::MemAccessError)?;
            if instr.field == 3 {
                reg.toggle_sign();
            }
            return Ok(());
        } else {
            return Err(TrapCode::InvalidField);
        }
    }

    /// Handler for `INC1-6`, `DEC1-6`, `ENT1-6`, `ENN1-6`.
    fn handler_instr_modify_3b(&mut self, instr: &instr::Instruction) -> Result<(), TrapCode> {
        let addr = self.helper_get_eff_addr_unchecked(instr.addr, instr.index);
        let reg = match instr.opcode {
            instr::Opcode::Modify1 => &mut self.r_in[1],
            instr::Opcode::Modify2 => &mut self.r_in[2],
            instr::Opcode::Modify3 => &mut self.r_in[3],
            instr::Opcode::Modify4 => &mut self.r_in[4],
            instr::Opcode::Modify5 => &mut self.r_in[5],
            instr::Opcode::Modify6 => &mut self.r_in[6],
            _ => unreachable!(),
        };

        if instr.field == 0 || instr.field == 1 {
            // INCx and DECx
            // Add or subtract one.
            let addr = addr as i64;
            let offset = if instr.field == 0 { addr } else { -addr };
            // Convert to i64.
            let value = reg.to_i64().0;
            // Convert back modified value.
            let (new_word, overflow) = HalfWord::from_i64(value + offset);
            reg.set(0..=2, &new_word[0..=2])
                .map_err(|_| TrapCode::MemAccessError)?;
            // Should we overflow?
            if overflow {
                self.overflow = overflow;
            }
            return Ok(());
        } else if instr.field == 2 || instr.field == 3 {
            // ENTx and ENNx
            let (new_word, _) = HalfWord::from_i64(addr as i64);
            // Copy new word into reg.
            reg.set(0..=2, &new_word[0..=2])
                .map_err(|_| TrapCode::MemAccessError)?;
            if instr.field == 3 {
                reg.toggle_sign();
            }
            return Ok(());
        } else {
            return Err(TrapCode::InvalidField);
        }
    }

    /// Handler for `ADD` and `SUB`.
    fn handler_instr_add_sub(&mut self, instr: &instr::Instruction) -> Result<(), TrapCode> {
        // Obtain V from memory.
        let target_mem = &self.mem[self.helper_get_eff_addr(instr.addr, instr.index)?];
        let orig_value = self.r_a.to_i64().0;
        // Are we adding or subtracting?
        let coefficient = match instr.opcode {
            instr::Opcode::Add => 1,
            instr::Opcode::Sub => -1,
            _ => unreachable!(),
        };
        let target_value =
            target_mem.to_i64_ranged(instr.field.to_range_inclusive()).0 * coefficient;
        // Calculate and pack new value.
        let new_value = orig_value + target_value;
        let (new_word, overflow) = FullWord::from_i64(new_value);
        // Set new value.
        self.r_a
            .set(0..=5, &new_word[0..=5])
            .map_err(|_| TrapCode::MemAccessError)?;
        // Should we overflow?
        if overflow {
            self.overflow = overflow;
        }

        Ok(())
    }

    /// Handler for `MUL`.
    fn handler_instr_mul(&mut self, instr: &instr::Instruction) -> Result<(), TrapCode> {
        // Obtain V from memory.
        let target_mem = &self.mem[self.helper_get_eff_addr(instr.addr, instr.index)?];
        let orig_value = self.r_a.to_i64().0;
        let target_value = target_mem.to_i64_ranged(instr.field.to_range_inclusive()).0;
        // Copy value into registers.
        let new_val = orig_value as i128 * target_value as i128;
        let new_val_bytes = new_val.abs().to_be_bytes();
        let mut new_val_bytes_dirty = new_val_bytes.map(|b| b != 0);
        for (reg_i, byte_i) in (0..10).rev().zip((0..16).rev()) {
            let byte = new_val_bytes[byte_i];
            if reg_i >= 5 {
                self.r_x[reg_i - 5 + 1] = byte;
            } else {
                self.r_a[reg_i + 1] = byte;
            }
            new_val_bytes_dirty[byte_i] = false;
        }
        // Treat sign.
        let new_sign = if new_val < 0 {
            FullWord::NEG
        } else {
            FullWord::POS
        };
        self.r_a[0] = new_sign;
        self.r_x[0] = new_sign;
        // Should we overflow?
        let overflow = new_val_bytes_dirty.iter().any(|&b| b);
        if overflow {
            self.overflow = overflow;
        }
        Ok(())
    }

    /// Handler for `CMPA` and `CMPX`.
    fn handler_instr_cmp_6b(&mut self, instr: &instr::Instruction) -> Result<(), TrapCode> {
        // Obtain CONTENT(M).
        let target_mem = &self.mem[self.helper_get_eff_addr(instr.addr, instr.index)?];
        let target_value = target_mem.to_i64_ranged(instr.field.to_range_inclusive()).0;
        let reg = match instr.opcode {
            instr::Opcode::CmpA => &self.r_a,
            instr::Opcode::CmpX => &self.r_x,
            _ => unreachable!(),
        };
        let reg_value = reg.to_i64_ranged(instr.field.to_range_inclusive()).0;
        // Calculate and set flags.
        self.indicator_comp = if reg_value.abs() == 0 && target_value.abs() == 0 {
            // +0 and -0 are equal.
            reg::ComparisonIndicatorValue::Equal
        } else if reg_value == target_value {
            reg::ComparisonIndicatorValue::Equal
        } else if reg_value > target_value {
            reg::ComparisonIndicatorValue::Greater
        } else {
            reg::ComparisonIndicatorValue::Lesser
        };
        Ok(())
    }

    /// Handler for `CMP1-6`.
    fn handler_instr_cmp_3b(&mut self, instr: &instr::Instruction) -> Result<(), TrapCode> {
        // Obtain CONTENT(M).
        let target_mem = &self.mem[self.helper_get_eff_addr(instr.addr, instr.index)?];
        let target_value = target_mem.to_i64_ranged(instr.field.to_range_inclusive()).0;
        let reg = match instr.opcode {
            instr::Opcode::Cmp1 => &self.r_in[1],
            instr::Opcode::Cmp2 => &self.r_in[2],
            instr::Opcode::Cmp3 => &self.r_in[3],
            instr::Opcode::Cmp4 => &self.r_in[4],
            instr::Opcode::Cmp5 => &self.r_in[5],
            instr::Opcode::Cmp6 => &self.r_in[6],
            _ => unreachable!(),
        };
        let padded_reg = FullWord::from_bytes([reg[0], 0, 0, 0, reg[1], reg[2]]);
        let reg_value = padded_reg.to_i64_ranged(instr.field.to_range_inclusive()).0;
        // Calculate and set flags.
        self.indicator_comp = if reg_value.abs() == 0 && target_value.abs() == 0 {
            // +0 and -0 are equal.
            reg::ComparisonIndicatorValue::Equal
        } else if reg_value == target_value {
            reg::ComparisonIndicatorValue::Equal
        } else if reg_value > target_value {
            reg::ComparisonIndicatorValue::Greater
        } else {
            reg::ComparisonIndicatorValue::Lesser
        };
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
