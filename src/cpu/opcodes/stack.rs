use super::opcode::CpuRegister16;

impl super::super::Cpu {
    pub fn ld_a16_sp(&mut self) -> u16 {
        let address = self.read_word();
        self.mmu.write_word(address, self.stack_pointer);
        20
    }

    pub fn ld_r16_d16(&mut self, register: &CpuRegister16) -> u16 {
        let value = self.read_word();
        self.registers.set_target_16(register, value);
        12
    }

    pub fn ld_sp_d16(&mut self) -> u16 {
        let value = self.read_word();
        self.stack_pointer = value;
        12
    }

    pub fn ld_hl_sp_e8(&mut self) -> u16 {
        let e = self.read_byte() as i8;
        let value = self.stack_pointer.wrapping_add(e as u16);
        self.registers.set_target_16(&CpuRegister16::HL, value);

        self.registers
            .f
            .set_carry(super::is_overflow_8(self.stack_pointer, e as u16));
        self.registers.f.set_half_carry(super::is_half_carry_8(
            self.stack_pointer as u8,
            e as u8,
            false,
            0,
        ));
        self.registers.f.set_subtraction(false);
        self.registers.f.set_zero(false);
        12
    }

    pub fn ld_sp_hl(&mut self) -> u16 {
        self.stack_pointer = self.registers.get_target_16(&CpuRegister16::HL);
        8
    }

    pub fn pop(&mut self, register: &CpuRegister16) -> u16 {
        let value = self.pop_stack();
        self.registers.set_target_16(register, value);
        12
    }

    pub fn push(&mut self, register: &CpuRegister16) -> u16 {
        let data = self.registers.get_target_16(register);
        self.push_stack(data);
        12
    }
}
