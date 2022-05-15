use crate::sim::mix::*;

/// The state of a MIX machine.
pub struct MixMachine {
    /// The register `rA`.
    pub r_a: register::GenericRegister,
    /// The register `rX`.
    pub r_x: register::GenericRegister,

    /// The register `rIn`, where `n = 1, 2, 3, 4, 5, 6`.
    pub r_in: [register::IndexRegister; 6],

    /// The register `rJ`.
    pub r_j: register::JumpRegister,

    /// The overflow toggle.
    pub toggle_overflow: bool,

    /// The comparison indicator.
    pub indicator_comp: register::ComparisonIndicatorValue,

    /// The memory.
    pub mem: mem::Mem,

    /// The instruction pointer.
    pub pc: usize,

    /// The machine running state.
    pub halted: bool,
}

impl MixMachine {
    /// Create a new MIX machine.
    pub fn new() -> Self {
        MixMachine {
            r_a: register::GenericRegister::new(),
            r_x: register::GenericRegister::new(),
            r_in: [register::IndexRegister::new(); 6],
            r_j: register::JumpRegister::new(),
            toggle_overflow: false,
            indicator_comp: register::ComparisonIndicatorValue::EQUAL,
            mem: mem::Mem::new(),
            pc: 0,
            halted: true,
        }
    }

    /// Reset the machine.
    ///
    /// This function resets the machine to its initial state.
    pub fn reset(&mut self) {
        self.pc = 0;
        self.halted = false;
        self.toggle_overflow = false;
    }

    /// Run the next instruction of the machine.
    pub fn step(&mut self) {
        if self.halted {
            return;
        }

        // Fetch the instruction.
        let instr: instr::Instruction = match self.mem[self.pc].try_into() {
            Ok(instr) => instr,
            Err(_) => {
                self.trap_illegal_instruction();
                return;
            }
        };

        // Run the instruction.
        let result = match instr.opcode {
            instr::Opcode::Nop => self.handler_instr_nop(instr),

            instr::Opcode::Add => todo!(),
            instr::Opcode::Sub => todo!(),
            instr::Opcode::Mul => todo!(),
            instr::Opcode::Div => todo!(),

            instr::Opcode::Special => todo!(),
            instr::Opcode::Shift => todo!(),
            instr::Opcode::Move => todo!(),

            instr::Opcode::LdA => self.handler_instr_load_6b(instr),
            instr::Opcode::Ld1 => todo!(),
            instr::Opcode::Ld2 => todo!(),
            instr::Opcode::Ld3 => todo!(),
            instr::Opcode::Ld4 => todo!(),
            instr::Opcode::Ld5 => todo!(),
            instr::Opcode::Ld6 => todo!(),
            instr::Opcode::LdX => self.handler_instr_load_6b(instr),

            instr::Opcode::LdAN => self.handler_instr_load_neg_6b(instr),
            instr::Opcode::Ld1N => todo!(),
            instr::Opcode::Ld2N => todo!(),
            instr::Opcode::Ld3N => todo!(),
            instr::Opcode::Ld4N => todo!(),
            instr::Opcode::Ld5N => todo!(),
            instr::Opcode::Ld6N => todo!(),
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
            instr::Opcode::Jmp => todo!(),

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
        };

        // Increase the instruction pointer, or halt.
        match result {
            Ok(()) => self.pc += 1,
            Err(msg) => self.trap_general_error(msg),
        }
    }

    /// Get indexed address.
    fn helper_get_eff_addr(&self, addr: i16, index: u8) -> Result<u16, &'static str> {
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
            let index_value = &self.r_in[index as usize - 1][1..=2];
            index_sign * i16::from_le_bytes([index_value[0], index_value[1]]) + addr
        };

        eff_addr.try_into().map_err(|_| "Invalid effective address")
    }

    /// Handler for `NOP`.
    ///
    /// This function does nothing.
    fn handler_instr_nop(&mut self, _: instr::Instruction) -> Result<(), &'static str> {
        // Do nothing.
        Ok(())
    }

    /// Handler for `LDA` and `LDX`.
    fn handler_instr_load_6b(&mut self, instr: instr::Instruction) -> Result<(), &'static str> {
        let mut field = instr.field_to_range_inclusive();
        // Obtain everything.
        let memory_cell = self.mem[self.helper_get_eff_addr(instr.addr, instr.index)? as usize];
        let reg = match instr.opcode {
            instr::Opcode::LdA => &mut self.r_a,
            instr::Opcode::LdX => &mut self.r_x,
            _ => unreachable!(),
        };
        // Zero reg before copying. Handle 'understood' positive sign too.
        reg.set(0..=5, &[1, 0, 0, 0, 0, 0])
            .map_err(|_| "Failed to zero reg")?;
        // Do we need to update the sign byte?
        let sign_copy_needed = *field.start() == 0;
        if sign_copy_needed {
            // Treat sign bit specially by moving it out.
            field = (*field.start() + 1)..=(*field.end());
        }
        // Copy bytes shifted right.
        let mut reg_cursor = 5;
        for memory_cell_cursor in field.rev() {
            reg[reg_cursor as usize] = memory_cell[memory_cell_cursor];
            reg_cursor -= 1;
        }
        // Copy sign byte if needed.
        if sign_copy_needed {
            reg[0] = memory_cell[0];
        }
        Ok(())
    }

    /// Handler for `LDAN` and `LDXN`.
    fn handler_instr_load_neg_6b(&mut self, instr: instr::Instruction) -> Result<(), &'static str> {
        let mut field = instr.field_to_range_inclusive();
        // Obtain everything.
        let memory_cell = self.mem[self.helper_get_eff_addr(instr.addr, instr.index)? as usize];
        let reg = match instr.opcode {
            instr::Opcode::LdAN => &mut self.r_a,
            instr::Opcode::LdXN => &mut self.r_x,
            _ => unreachable!(),
        };
        // Zero reg before copying. Handle 'understood' negative sign.
        reg.set(0..=5, &[0, 0, 0, 0, 0, 0])
            .map_err(|_| "Failed to zero reg")?;
        // Do we need to update the sign byte?
        let sign_copy_needed = *field.start() == 0;
        if sign_copy_needed {
            // Treat sign bit specially by moving it out.
            field = (*field.start() + 1)..=(*field.end());
        }
        // Copy bytes shifted right.
        let mut reg_cursor = 5;
        for memory_cell_cursor in field.rev() {
            reg[reg_cursor as usize] = memory_cell[memory_cell_cursor];
            reg_cursor -= 1;
        }
        // Copy negated sign byte if needed.
        if sign_copy_needed {
            reg[0] = if memory_cell[0] == 0 { 1 } else { 0 };
        }
        Ok(())
    }

    /// Trap handler for illegal instructions.
    ///
    /// This function is called when an illegal instruction is
    /// encountered. It halts the machine and prints the
    /// offending address.
    fn trap_illegal_instruction(&mut self) {
        println!("HALT! Illegal instruction at address {}", self.pc);
        self.halted = true;
    }

    /// Trap handler for general error.
    /// 
    /// This function is called when any other error is encountered.
    /// It halts the machine and prints the error message.
    fn trap_general_error(&mut self, message: &str) {
        println!("HALT! General error: {}", message);
        self.halted = true;
    }
}
