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
    /// This function resets the machine to its initial state.
    /// It does not clear the memory.
    ///
    /// It is equivalent to `self.pc = 0; self.halted = false; self.toggle_overflow = false;`.
    pub fn reset(&mut self) {
        self.pc = 0;
        self.halted = false;
        self.toggle_overflow = false;
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

            instr::Opcode::Special => todo!(),
            instr::Opcode::Shift => todo!(),
            instr::Opcode::Move => todo!(),

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

            instr::Opcode::StA => todo!(),
            instr::Opcode::St1 => todo!(),
            instr::Opcode::St2 => todo!(),
            instr::Opcode::St3 => todo!(),
            instr::Opcode::St4 => todo!(),
            instr::Opcode::St5 => todo!(),
            instr::Opcode::St6 => todo!(),
            instr::Opcode::StX => todo!(),
            instr::Opcode::StJ => todo!(),
            instr::Opcode::StZ => todo!(),

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
        let mut field = instr.field_to_range_inclusive();
        // Obtain everything.
        let memory_cell = self.mem[self.helper_get_eff_addr(instr.addr, instr.index)?];
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
        let mut field = instr.field_to_range_inclusive();
        // Obtain everything.
        let memory_cell = self.mem[self.helper_get_eff_addr(instr.addr, instr.index)?];
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
        let mut field = instr.field_to_range_inclusive();
        // Obtain everything.
        let memory_cell = self.mem[self.helper_get_eff_addr(instr.addr, instr.index)?];
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
        let mut field = instr.field_to_range_inclusive();
        // Obtain everything.
        let memory_cell = self.mem[self.helper_get_eff_addr(instr.addr, instr.index)?];
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
