use crate::sim::instr::ToRangeInclusive;
use crate::sim::*;

/// A internal shortcut to a 6-byte Word.
type FullWord = mem::Word<6, false>;

/// A internal shortcut to a 3-byte Word.
type HalfWord = mem::Word<3, false>;

/// A internal shortcut to a always-positive 3-byte Word.
type PosHalfWord = mem::Word<3, true>;

/// Error codes for the MIX machine.
#[derive(PartialEq, Eq, Debug)]
pub enum ErrorCode {
    GeneralError,
    IllegalInstruction,
    InvalidAddress,
    InvalidField,
    InvalidIndex,
    MemAccessError,
    UnknownDevice,
    IOError,
    Halted,
}

/// Values of the comparison indicator.
#[derive(PartialEq, Eq, Debug)]
pub enum ComparisonIndicatorValue {
    Equal,
    Lesser,
    Greater,
}

/// The state of a MIX machine.
///
/// # Example
/// ```rust
/// use mixture::sim::*;
///
/// let mut machine = MixMachine::new();
/// machine.reset();
/// machine.restart();
///
/// assert_eq!(machine.halted, false);
///
/// machine.step().unwrap();
/// ```
pub struct MixMachine {
    /// The register `rA`.
    pub r_a: FullWord,

    /// The register `rX`.
    pub r_x: FullWord,

    /// The register `rIn`, where `n = 1, 2, 3, 4, 5, 6`.
    /// `r_in[0]` should always used as a source of 0.
    pub r_in: [HalfWord; 7],

    /// The register `rJ`.
    pub r_j: PosHalfWord,

    /// The overflow toggle.
    pub overflow: bool,

    /// The comparison indicator.
    pub indicator_comp: ComparisonIndicatorValue,

    /// The memory.
    pub mem: mem::Mem,

    /// The instruction pointer.
    pub pc: u16,

    /// The machine running state.
    pub halted: bool,

    /// IO devices.
    pub io_devices: [Option<Box<dyn io::IODevice>>; 21],
}

impl MixMachine {
    /// Create a new MIX machine.
    pub fn new() -> Self {
        MixMachine {
            r_a: FullWord::new(),
            r_x: FullWord::new(),
            r_in: [HalfWord::new(); 7],
            r_j: PosHalfWord::new(),
            overflow: false,
            indicator_comp: ComparisonIndicatorValue::Equal,
            mem: mem::Mem::new(),
            pc: 0,
            halted: true,
            io_devices: Default::default(),
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
        self.r_a = FullWord::new();
        self.r_x = FullWord::new();
        self.r_in = [HalfWord::new(); 7];
        self.r_j = PosHalfWord::new();
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
    /// * [`Ok(())`] - The machine successfully completed its operation.
    /// * [`Err(ErrorCode)`] - The machine encountered an error and is now halted.
    pub fn step(&mut self) -> Result<(), ErrorCode> {
        if self.halted {
            return Err(ErrorCode::Halted);
        }

        // Fetch the instruction.
        let instr: instr::Instruction = self.mem[self.pc].try_into().map_err(|_| {
            self.halt();
            ErrorCode::IllegalInstruction
        })?;

        self.pc += 1;

        // Run the instruction.
        match instr.opcode {
            instr::Opcode::Nop => self.handler_instr_nop(&instr),

            instr::Opcode::Add => self.handler_instr_add_sub(&instr),
            instr::Opcode::Sub => self.handler_instr_add_sub(&instr),
            instr::Opcode::Mul => self.handler_instr_mul(&instr),
            instr::Opcode::Div => self.handler_instr_div(&instr),

            instr::Opcode::Special => self.handler_instr_special(&instr),
            instr::Opcode::Shift => self.handler_instr_shift(&instr),
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

            instr::Opcode::Jbus => self.handler_instr_jbus_jred(&instr),
            instr::Opcode::Ioc => self.handler_instr_ioc(&instr),
            instr::Opcode::In => self.handler_instr_in_out(&instr),
            instr::Opcode::Out => self.handler_instr_in_out(&instr),
            instr::Opcode::Jred => self.handler_instr_jbus_jred(&instr),
            instr::Opcode::Jmp => self.handler_instr_jmp(&instr),

            instr::Opcode::JA => self.handler_instr_jmp_reg_6b(&instr),
            instr::Opcode::J1 => self.handler_instr_jmp_reg_3b(&instr),
            instr::Opcode::J2 => self.handler_instr_jmp_reg_3b(&instr),
            instr::Opcode::J3 => self.handler_instr_jmp_reg_3b(&instr),
            instr::Opcode::J4 => self.handler_instr_jmp_reg_3b(&instr),
            instr::Opcode::J5 => self.handler_instr_jmp_reg_3b(&instr),
            instr::Opcode::J6 => self.handler_instr_jmp_reg_3b(&instr),
            instr::Opcode::JX => self.handler_instr_jmp_reg_6b(&instr),

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
        }
        .map_err(|err| {
            self.halt();
            err
        })?;

        Ok(())
    }

    /// Halt the machine.
    pub fn halt(&mut self) {
        self.halted = true;
    }

    /// Get indexed address.
    fn helper_get_eff_addr(&self, addr: i16, index: u8) -> Result<u16, ErrorCode> {
        // Direct or indirect addressing.
        // r_in[0] is always zero.
        if !(0..=6).contains(&index) {
            // We have been provided a bad index.
            return Err(ErrorCode::InvalidIndex);
        }
        let reg = self.r_in[index as usize];
        let (reg_val, _) = reg.to_i64();
        (reg_val + addr as i64)
            .try_into()
            .map_err(|_| ErrorCode::InvalidAddress)
    }

    /// Get indexed address. May panic or return negative value.
    fn helper_get_eff_addr_unchecked(&self, addr: i16, index: u8) -> i16 {
        let reg = self.r_in[index as usize];
        let (reg_val, _) = reg.to_i64();
        reg_val as i16 + addr
    }

    /// Do actual jump.
    fn helper_do_jump(&mut self, location: u16, save_r_j: bool) -> Result<(), ErrorCode> {
        if save_r_j {
            let pc_unpacked = (self.pc as u16).to_be_bytes();
            self.r_j
                .set(1..=2, &pc_unpacked)
                .map_err(|_| ErrorCode::MemAccessError)?;
        }
        // Do jump.
        self.pc = location;
        Ok(())
    }

    /// Get IO device.
    fn helper_get_io_device(&self, dev_id: usize) -> Result<&Box<dyn IODevice>, ErrorCode> {
        let dev = self
            .io_devices
            .get(dev_id)
            .ok_or(ErrorCode::InvalidField)?
            .as_ref()
            .ok_or(ErrorCode::UnknownDevice)?;
        Ok(dev)
    }

    /// Get IO device.
    fn helper_get_io_device_mut(
        &mut self,
        dev_id: usize,
    ) -> Result<&mut Box<dyn IODevice>, ErrorCode> {
        let dev = self
            .io_devices
            .get_mut(dev_id)
            .ok_or(ErrorCode::InvalidField)?
            .as_mut()
            .ok_or(ErrorCode::UnknownDevice)?;
        Ok(dev)
    }

    /// Handler for `NOP`.
    ///
    /// This function does nothing.
    fn handler_instr_nop(&mut self, _: &instr::Instruction) -> Result<(), ErrorCode> {
        // Do nothing.
        Ok(())
    }

    /// Handler for `LDA` and `LDX`.
    fn handler_instr_load_6b(&mut self, instr: &instr::Instruction) -> Result<(), ErrorCode> {
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
            .map_err(|_| ErrorCode::MemAccessError)?;
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
    fn handler_instr_load_neg_6b(&mut self, instr: &instr::Instruction) -> Result<(), ErrorCode> {
        // Obtain everything.
        let (field, sign_copy_needed) = instr.field.to_range_inclusive_signless();
        let memory_cell = &self.mem[self.helper_get_eff_addr(instr.addr, instr.index)?];
        let reg = match instr.opcode {
            instr::Opcode::LdAN => &mut self.r_a,
            instr::Opcode::LdXN => &mut self.r_x,
            _ => unreachable!(),
        };
        // Zero reg before copying. Handle 'understood' negative sign.
        reg.set(0..=5, &[0; 6])
            .map_err(|_| ErrorCode::MemAccessError)?;
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
    fn handler_instr_load_3b(&mut self, instr: &instr::Instruction) -> Result<(), ErrorCode> {
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
        for (reg_cursor, memory_cell_cursor) in (1..=5).rev().zip(field.rev()) {
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
    fn handler_instr_load_neg_3b(&mut self, instr: &instr::Instruction) -> Result<(), ErrorCode> {
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
        let mut temp = FullWord::from_bytes([0; 6]);
        // Copy bytes shifted right.
        for (reg_cursor, memory_cell_cursor) in (1..=5).rev().zip(field.rev()) {
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
    fn handler_instr_jmp(&mut self, instr: &instr::Instruction) -> Result<(), ErrorCode> {
        let target_addr = self.helper_get_eff_addr(instr.addr, instr.index)?;
        // Match jump conditions.
        let should_jump = match instr.field {
            0 | 1 => true,
            2 => self.overflow,
            3 => !self.overflow,
            4 => self.indicator_comp == ComparisonIndicatorValue::Lesser,
            5 => self.indicator_comp == ComparisonIndicatorValue::Equal,
            6 => self.indicator_comp == ComparisonIndicatorValue::Greater,
            7 => self.indicator_comp != ComparisonIndicatorValue::Lesser,
            8 => self.indicator_comp != ComparisonIndicatorValue::Equal,
            9 => self.indicator_comp != ComparisonIndicatorValue::Greater,
            _ => return Err(ErrorCode::InvalidField),
        };
        // Clear overflow flag.
        if instr.field == 2 || instr.field == 3 {
            self.overflow = false;
        }
        if should_jump {
            self.helper_do_jump(target_addr, instr.field != 1)?;
        }
        Ok(())
    }

    /// Handler for `CHAR`, `NUM` and `HLT`.
    fn handler_instr_special(&mut self, instr: &instr::Instruction) -> Result<(), ErrorCode> {
        if instr.field == 0 {
            // NUM instruction
            let a_content = &self.r_a[1..=5];
            let x_content = &self.r_x[1..=5];
            let mut result: i64 = 0;
            // For each byte, we extract its 1st position,
            // and push it to `result`.
            for &byte in a_content.iter().chain(x_content.iter()) {
                let digit = byte % 10;
                result = result * 10 + digit as i64;
            }
            // Rebuild a word of 4 bytes.
            let (result_word, _) = FullWord::from_i64(result);
            // We do not modify the sign byte.
            self.r_a
                .set(1..=5, &result_word[1..=5])
                .map_err(|_| ErrorCode::MemAccessError)?;
            return Ok(());
        } else if instr.field == 1 {
            // CHAR instruction
            // Obtain original number.
            let mut source = self.r_a.to_i64().0.abs();
            // Extract each digit.
            for reg_i in (0..10).rev() {
                if reg_i >= 5 {
                    self.r_x[reg_i - 5 + 1] = (source % 10 + 30) as u8;
                } else {
                    self.r_a[reg_i + 1] = (source % 10 + 30) as u8;
                }
                source /= 10;
            }
            return Ok(());
        } else if instr.field == 2 {
            // HLT instruction
            // Making it just like NOP if we restart the
            // machine later.
            self.halted = true;
            return Ok(());
        } else {
            return Err(ErrorCode::InvalidField);
        }
    }

    /// Handler for `STZ`.
    fn handler_instr_store_zero(&mut self, instr: &instr::Instruction) -> Result<(), ErrorCode> {
        // Obtain everything.
        let addr = self.helper_get_eff_addr(instr.addr, instr.index)?;
        let field = instr.field.to_range_inclusive();
        let memory_cell = &mut self.mem[addr];
        // Zero the memory cell.
        for i in field {
            if i == 0 {
                // Deal with signs.
                memory_cell[0] = 1;
            } else {
                memory_cell[i] = 0;
            }
        }
        Ok(())
    }

    /// Handler for `MOVE`.
    fn handler_instr_move(&mut self, instr: &instr::Instruction) -> Result<(), ErrorCode> {
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
                .map_err(|_| ErrorCode::MemAccessError)?;
        }
        let new_r_i1_val = self.r_in[1].to_i64().0 + num_words as i64;
        let (new_r_i1, overflow) = HalfWord::from_i64(new_r_i1_val);
        self.r_in[1]
            .set(0..=2, &new_r_i1[0..=2])
            .map_err(|_| ErrorCode::MemAccessError)?;
        if overflow {
            self.overflow = overflow;
        }
        Ok(())
    }

    /// Handler for `STA` and `STX`.
    fn handler_instr_store_6b(&mut self, instr: &instr::Instruction) -> Result<(), ErrorCode> {
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
        for (reg_cursor, memory_cell_cursor) in (1..=5).rev().zip(field.rev()) {
            memory_cell[memory_cell_cursor] = reg[reg_cursor];
        }
        if sign_copy_needed {
            // Copy sign bit.
            memory_cell[0] = reg[0];
        }
        Ok(())
    }

    /// Handler for `ST1-6`.
    fn handler_instr_store_3b(&mut self, instr: &instr::Instruction) -> Result<(), ErrorCode> {
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
        for (reg_cursor, memory_cell_cursor) in (1..=5).rev().zip(field.rev()) {
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
    fn handler_instr_modify_6b(&mut self, instr: &instr::Instruction) -> Result<(), ErrorCode> {
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
                .map_err(|_| ErrorCode::MemAccessError)?;
            // Should we overflow?
            if overflow {
                self.overflow = overflow;
            }
            return Ok(());
        } else if instr.field == 2 || instr.field == 3 {
            // ENTx and ENNx
            let new_word = FullWord::from_i64(addr as i64).0;
            // Copy new word into reg.
            reg.set(0..=5, &new_word[0..=5])
                .map_err(|_| ErrorCode::MemAccessError)?;
            if instr.field == 3 {
                reg.toggle_sign();
            }
            return Ok(());
        } else {
            return Err(ErrorCode::InvalidField);
        }
    }

    /// Handler for `INC1-6`, `DEC1-6`, `ENT1-6`, `ENN1-6`.
    fn handler_instr_modify_3b(&mut self, instr: &instr::Instruction) -> Result<(), ErrorCode> {
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
                .map_err(|_| ErrorCode::MemAccessError)?;
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
                .map_err(|_| ErrorCode::MemAccessError)?;
            if instr.field == 3 {
                reg.toggle_sign();
            }
            return Ok(());
        } else {
            return Err(ErrorCode::InvalidField);
        }
    }

    /// Handler for `ADD` and `SUB`.
    fn handler_instr_add_sub(&mut self, instr: &instr::Instruction) -> Result<(), ErrorCode> {
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
            .map_err(|_| ErrorCode::MemAccessError)?;
        // Should we overflow?
        if overflow {
            self.overflow = overflow;
        }

        Ok(())
    }

    /// Handler for `MUL`.
    fn handler_instr_mul(&mut self, instr: &instr::Instruction) -> Result<(), ErrorCode> {
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

    /// Handler for `DIV`.
    fn handler_instr_div(&mut self, instr: &instr::Instruction) -> Result<(), ErrorCode> {
        // TODO: Make a generic version of Word::from_int().
        // Obtain value.
        let target_mem = &self.mem[self.helper_get_eff_addr(instr.addr, instr.index)?];
        let target_value = target_mem.to_i64_ranged(instr.field.to_range_inclusive()).0 as i128;
        let orig_value = i128::from_be_bytes([
            0,
            0,
            0,
            0,
            0,
            0,
            self.r_a[1],
            self.r_a[2],
            self.r_a[3],
            self.r_a[4],
            self.r_a[5],
            self.r_x[1],
            self.r_x[2],
            self.r_x[3],
            self.r_x[4],
            self.r_x[5],
        ]) * if self.r_a.is_positive() { 1 } else { -1 };
        // Calculate results.
        let quotient: i64 = (orig_value / target_value)
            .abs()
            .try_into()
            .map_err(|_| {
                // Conversion overflowed.
                self.overflow = true;
            })
            .unwrap_or(0);
        let remainder: i64 = (orig_value % target_value)
            .abs()
            .try_into()
            .map_err(|_| {
                // Conversion overflowed.
                self.overflow = true;
            })
            .unwrap_or(0);
        // Calculate new sign.
        let new_sign_positive = if orig_value.signum() == target_value.signum() {
            true
        } else {
            false
        };
        // Copy results into registers.
        let (new_a, overflow_a) = FullWord::from_i64(quotient);
        let (new_x, overflow_x) = FullWord::from_i64(remainder);
        self.r_x[0] = self.r_a[0];
        self.r_a[0] = if new_sign_positive {
            FullWord::POS
        } else {
            FullWord::NEG
        };
        self.r_a
            .set(1..=5, &new_a[1..=5])
            .map_err(|_| ErrorCode::MemAccessError)?;
        self.r_x
            .set(1..=5, &new_x[1..=5])
            .map_err(|_| ErrorCode::MemAccessError)?;
        if overflow_a || overflow_x {
            // Copy overflowed.
            self.overflow = true;
        }
        Ok(())
    }

    /// Handler for `CMPA` and `CMPX`.
    fn handler_instr_cmp_6b(&mut self, instr: &instr::Instruction) -> Result<(), ErrorCode> {
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
            ComparisonIndicatorValue::Equal
        } else if reg_value == target_value {
            ComparisonIndicatorValue::Equal
        } else if reg_value > target_value {
            ComparisonIndicatorValue::Greater
        } else {
            ComparisonIndicatorValue::Lesser
        };
        Ok(())
    }

    /// Handler for `CMP1-6`.
    fn handler_instr_cmp_3b(&mut self, instr: &instr::Instruction) -> Result<(), ErrorCode> {
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
            ComparisonIndicatorValue::Equal
        } else if reg_value == target_value {
            ComparisonIndicatorValue::Equal
        } else if reg_value > target_value {
            ComparisonIndicatorValue::Greater
        } else {
            ComparisonIndicatorValue::Lesser
        };
        Ok(())
    }

    /// Handler for `JA` and `JX`.
    fn handler_instr_jmp_reg_6b(&mut self, instr: &instr::Instruction) -> Result<(), ErrorCode> {
        let target_addr = self.helper_get_eff_addr(instr.addr, instr.index)?;
        let reg = match instr.opcode {
            instr::Opcode::JA => &self.r_a,
            instr::Opcode::JX => &self.r_x,
            _ => unreachable!(),
        };
        let reg_value_sign = reg.to_i64().0.signum();
        let should_jump = match instr.field {
            0 => reg_value_sign == -1,
            1 => reg_value_sign == 0,
            2 => reg_value_sign == 1,
            3 => reg_value_sign != -1,
            4 => reg_value_sign != 0,
            5 => reg_value_sign != 1,
            _ => return Err(ErrorCode::InvalidField),
        };
        if should_jump {
            self.helper_do_jump(target_addr, true)?;
        }
        Ok(())
    }

    /// Handler for `J1-6`.
    fn handler_instr_jmp_reg_3b(&mut self, instr: &instr::Instruction) -> Result<(), ErrorCode> {
        let target_addr = self.helper_get_eff_addr(instr.addr, instr.index)?;
        let reg = match instr.opcode {
            instr::Opcode::J1 => &self.r_in[1],
            instr::Opcode::J2 => &self.r_in[2],
            instr::Opcode::J3 => &self.r_in[3],
            instr::Opcode::J4 => &self.r_in[4],
            instr::Opcode::J5 => &self.r_in[5],
            instr::Opcode::J6 => &self.r_in[6],
            _ => unreachable!(),
        };
        let reg_value_sign = reg.to_i64().0.signum();
        let should_jump = match instr.field {
            0 => reg_value_sign == -1,
            1 => reg_value_sign == 0,
            2 => reg_value_sign == 1,
            3 => reg_value_sign != -1,
            4 => reg_value_sign != 0,
            5 => reg_value_sign != 1,
            _ => return Err(ErrorCode::InvalidField),
        };
        if should_jump {
            self.helper_do_jump(target_addr, true)?;
        }
        Ok(())
    }

    /// Handler for `SLA`, `SRA`, `SLAX`, `SRAX`, `SLC` and `SRC`.
    fn handler_instr_shift(&mut self, instr: &instr::Instruction) -> Result<(), ErrorCode> {
        let count = self.helper_get_eff_addr(instr.addr, instr.index)?;
        if instr.field == 0 || instr.field == 1 {
            // SLA and SRA.
            // Spread original register to bytes.
            let orig_value = u64::from_be_bytes(self.r_a.to_i64().0.to_be_bytes());
            // Shift the value in bits (count * 8, count is in bytes).
            let shifted_value = match instr.field {
                0 => orig_value << count * 8,
                1 => orig_value >> count * 8,
                _ => unreachable!(),
            };
            // Store back.
            self.r_a
                .set(1..=5, &shifted_value.to_be_bytes()[3..=7])
                .map_err(|_| ErrorCode::MemAccessError)?;
        } else if instr.field == 2 || instr.field == 3 {
            // SLAX and SRAX.
            // Spread original register to bytes.
            let orig_a_bytes = &self.r_a[1..=5];
            let orig_x_bytes = &self.r_x[1..=5];
            let orig_value = u128::from_be_bytes([
                0,
                0,
                0,
                0,
                0,
                0,
                orig_a_bytes[0],
                orig_a_bytes[1],
                orig_a_bytes[2],
                orig_a_bytes[3],
                orig_a_bytes[4],
                orig_x_bytes[0],
                orig_x_bytes[1],
                orig_x_bytes[2],
                orig_x_bytes[3],
                orig_x_bytes[4],
            ]);
            // Shift.
            let shifted_value = match instr.field {
                2 => orig_value << count * 8,
                3 => orig_value >> count * 8,
                _ => unreachable!(),
            };
            // Store back.
            let shifted_bytes = shifted_value.to_be_bytes();
            self.r_a
                .set(1..=5, &shifted_bytes[6..=10])
                .map_err(|_| ErrorCode::MemAccessError)?;
            self.r_x
                .set(1..=5, &shifted_bytes[11..=15])
                .map_err(|_| ErrorCode::MemAccessError)?;
        } else if instr.field == 4 || instr.field == 5 {
            // SLC and SRC.
            // Spread out bytes.
            let orig_bytes = [
                self.r_a[1],
                self.r_a[2],
                self.r_a[3],
                self.r_a[4],
                self.r_a[5],
                self.r_x[1],
                self.r_x[2],
                self.r_x[3],
                self.r_x[4],
                self.r_x[5],
            ];
            // Zero the registers.
            self.r_a
                .set(1..=5, &[0; 5])
                .map_err(|_| ErrorCode::MemAccessError)?;
            self.r_x
                .set(1..=5, &[0; 5])
                .map_err(|_| ErrorCode::MemAccessError)?;
            // Create cyclic iterator.
            let mut orig_bytes_iter = orig_bytes.iter().cycle();
            // Get shift count.
            let offset = if instr.field == 4 {
                // SLC.
                count % 10
            } else {
                // SRC.
                10 - count % 10
            };
            for _ in 0..offset {
                // Advance the iterator by `offset` steps,
                // to simulate shifting.
                orig_bytes_iter.next().unwrap();
            }
            // Write back.
            for (reg_i, &digit) in (0..10).zip(orig_bytes_iter) {
                if reg_i >= 5 {
                    self.r_x[reg_i - 5 + 1] = digit;
                } else {
                    self.r_a[reg_i + 1] = digit;
                }
            }
        } else {
            return Err(ErrorCode::InvalidField);
        }
        Ok(())
    }

    /// Handler for `JBUS` and `JRED`.
    fn handler_instr_jbus_jred(&mut self, instr: &instr::Instruction) -> Result<(), ErrorCode> {
        // Get device ID.
        let dev_id: usize = instr.field as usize;
        // Get device reference.
        let dev = self.helper_get_io_device(dev_id)?;
        // Call appropriate callbacks.
        let should_jump = match instr.opcode {
            instr::Opcode::Jbus => dev.is_busy().map_err(|_| ErrorCode::IOError)?,
            instr::Opcode::Jred => dev.is_ready().map_err(|_| ErrorCode::IOError)?,
            _ => unreachable!(),
        };
        if should_jump {
            // Do jump.
            let jump_addr = self.helper_get_eff_addr(instr.addr, instr.index)?;
            self.helper_do_jump(jump_addr, true)?;
        }
        Ok(())
    }

    /// Handler for `IOC`.
    fn handler_instr_ioc(&mut self, instr: &instr::Instruction) -> Result<(), ErrorCode> {
        // Get command.
        let command = self.helper_get_eff_addr_unchecked(instr.addr, instr.index);
        // Get device ID.
        let dev_id: usize = instr.field as usize;
        // Get device reference.
        let dev = self.helper_get_io_device_mut(dev_id)?;
        // Call appropriate callbacks.
        dev.control(command).map_err(|_| ErrorCode::IOError)?;
        Ok(())
    }

    /// Handler for `IN` and `OUT`.
    fn handler_instr_in_out(&mut self, instr: &instr::Instruction) -> Result<(), ErrorCode> {
        // Check starting address.
        let addr_start = self.helper_get_eff_addr(instr.addr, instr.index)?;
        if !(0..mem::Mem::SIZE as u16).contains(&addr_start) {
            return Err(ErrorCode::InvalidAddress);
        }
        // Get device ID.
        let dev_id: usize = instr.field as usize;
        // Get device reference.
        let dev = self
            .io_devices
            .get_mut(dev_id)
            .ok_or(ErrorCode::InvalidField)?
            .as_mut()
            .ok_or(ErrorCode::UnknownDevice)?;
        let dev_blk_size = dev.get_block_size();
        // Check ending address.
        let addr_end = addr_start + dev_blk_size as u16;
        if !(0..mem::Mem::SIZE as u16).contains(&addr_end) {
            return Err(ErrorCode::InvalidAddress);
        }
        // Call appropriate callbacks.
        match instr.opcode {
            instr::Opcode::In => {
                let words = dev.read().map_err(|_| ErrorCode::IOError)?;
                // Reject blocks with wrong length.
                if words.len() != dev_blk_size {
                    return Err(ErrorCode::IOError);
                }
                // Copy words to memory.
                for (i, word) in words.iter().enumerate() {
                    self.mem[addr_start + i as u16] = *word;
                }
            }
            instr::Opcode::Out => {
                // Clone words.
                let words = self.mem[addr_start as usize..addr_end as usize].to_vec();
                dev.write(&words).map_err(|_| ErrorCode::IOError)?;
            }
            _ => unreachable!(),
        };
        Ok(())
    }
}
