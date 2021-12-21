use super::opcode::{CpuRegister, CpuRegister16};

impl super::super::Cpu {
    pub fn adc_a(&mut self, register: &CpuRegister) -> u16 {
        let target = self.registers.get_target(register);
        let result = self.add(target, true);
        self.registers.a = result as u8;
        4
    }

    pub fn adc_d8(&mut self) -> u16 {
        let target = self.read_byte();
        let result = self.add(target, true);
        self.registers.a = result;
        8
    }

    pub fn adc_hl(&mut self) -> u16 {
        let target = self.mmu.read_byte(self.registers.get_target_16(&CpuRegister16::HL));
        let result = self.add(target, true);
        self.registers.a = result;
        8
    }

    pub fn add_a(&mut self, register: &CpuRegister) -> u16 {
        let target = self.registers.get_target(register);
        let result = self.add(target, false);
        self.registers.a = result as u8;
        4
    }

    pub fn add_a_hl(&mut self) -> u16 {
        let target = self.mmu.read_byte(self.registers.get_target_16(&CpuRegister16::HL));
        let result = self.add(target, false);
        self.registers.a = result;
        8
    }

    pub fn add_d8(&mut self) -> u16 {
        let target = self.read_byte();
        let result = self.add(target, false);
        self.registers.a = result;
        8
    }

    pub fn add_hl_16(&mut self, register: &CpuRegister16) -> u16 {
        let hl_register = &CpuRegister16::HL;
        let hl_register_value = self.registers.get_target_16(hl_register);
        let value = self.registers.get_target_16(register);
        let (result, overflow) = hl_register_value.overflowing_add(value);
        self.registers.set_target_16(hl_register, result);

        self.registers.f.set_carry(overflow);
        self.registers.f.set_half_carry(super::is_half_carry_16(hl_register_value, value));
        self.registers.f.set_subtraction(false);

        8
    }

    pub fn add_hl_sp(&mut self) -> u16 {
        let register = &CpuRegister16::HL;
        let value = self.registers.get_target_16(register);
        let (result, overflow) = value.overflowing_add(self.stack_pointer);
        self.registers.set_target_16(register, result);

        self.registers.f.set_carry(overflow);
        self.registers.f.set_half_carry(super::is_half_carry_16(value, self.stack_pointer));
        self.registers.f.set_subtraction(false);

        8
    }
    
    pub fn add_sp_e8(&mut self) -> u16 {
        let e = self.read_byte() as i8;
        let value = self.stack_pointer.wrapping_add(e as u16);
        
        self.registers.f.set_carry(super::is_overflow_8(self.stack_pointer as u16, e as u16));
        self.registers.f.set_half_carry(super::is_half_carry_8(self.stack_pointer as u8, e as u8, false, 0));
        self.registers.f.set_subtraction(false);
        self.registers.f.set_zero(false);
        
        self.stack_pointer = value;
        16
    }

    pub fn and_a(&mut self, register: &CpuRegister) -> u16 {
        let target = self.registers.get_target(register);
        let result = self.and(target);
        self.registers.a = result;
        4
    }

    pub fn and_d8(&mut self) -> u16 {
        let target = self.read_byte();
        let result = self.and(target);
        self.registers.a = result;
        8
    }

    pub fn and_hl(&mut self) -> u16 {
        let target = self.mmu.read_byte(self.registers.get_target_16(&CpuRegister16::HL));
        let result = self.and(target);
        self.registers.a = result;
        8
    }

    pub fn ccf(&mut self) -> u16 {
        self.registers.f.set_carry(!self.registers.f.carry());
        self.registers.f.set_half_carry(false);
        self.registers.f.set_subtraction(false);
        4
    }

    pub fn cp_a(&mut self, register: &CpuRegister) -> u16 {
        let target = self.registers.get_target(register);
        self.sub(target, false);
        4
    }

    pub fn cp_d8(&mut self) -> u16 {
        let target = self.read_byte();
        self.sub(target, false);
        8
    }

    pub fn cp_hl(&mut self) -> u16 {
        let target = self.mmu.read_byte(self.registers.get_target_16(&CpuRegister16::HL));
        self.sub(target, false);
        8
    }

    pub fn cpl(&mut self) -> u16 {
        self.registers.a = !self.registers.a;
        self.registers.f.set_half_carry(true);
        self.registers.f.set_subtraction(true);
        4
    }

    pub fn daa(&mut self) -> u16 {
        // When this instruction is executed, the A register is BCD corrected using the contents of the flags. 
        // The exact process is the following: if the least significant four bits of A contain a non-BCD digit 
        // (i. e. it is greater than 9) or the H flag is set, then $06 is added to the register. 
        // Then the four most significant bits are checked. 
        // If this more significant digit also happens to be greater than 9 or the C flag is set, then $60 is added.
        let mut value = self.registers.a;
        let mut correction = if self.registers.f.carry() { 0x60 } else { 0x00 };
        if self.registers.f.half_carry() {
            correction |= 0x06;
        }

        if self.registers.f.subtraction() {
            value = value.wrapping_sub(correction);
        } else {
            if value & 0x0F > 0x09 { correction |= 0x06; };
            if value > 0x99 { correction |= 0x60; };
            value = value.wrapping_add(correction);
        }

        self.registers.a = value;
        self.registers.f.set_carry(correction >= 0x60);
        self.registers.f.set_half_carry(false);
        self.registers.f.set_zero(self.registers.a == 0);

        4
    }

    pub fn dec_hl(&mut self) -> u16 {
        let address = self.registers.get_target_16(&CpuRegister16::HL);
        let target = self.mmu.read_byte(address);
        let data = self.dec_8(target);
        self.mmu.write_byte(address, data);
        12
    }

    pub fn dec_r(&mut self, register: &CpuRegister) -> u16 {
        let target = self.registers.get_target(register);
        let value = self.dec_8(target);
        self.registers.set_target(register, value);
        4
    }

    pub fn dec_sp(&mut self) -> u16 {
        self.stack_pointer = self.stack_pointer.wrapping_sub(1);
        8
    }

    pub fn dec_16(&mut self, register: &CpuRegister16) -> u16 {
        let target = self.registers.get_target_16(register);
        let value = target.wrapping_sub(1);
        self.registers.set_target_16(register, value);
        8
    }

    pub fn inc_hl(&mut self) -> u16 {
        let address = self.registers.get_target_16(&CpuRegister16::HL);
        let target = self.mmu.read_byte(address);
        let data = self.inc_8(target);
        self.mmu.write_byte(address, data);
        12
    }

    pub fn inc_r(&mut self, register: &CpuRegister) -> u16 {
        let target = self.registers.get_target(register);
        let value = self.inc_8(target);
        self.registers.set_target(register, value);
        4
    }

    pub fn inc_sp(&mut self) -> u16 {
        self.stack_pointer = self.stack_pointer.wrapping_add(1);
        8
    }

    pub fn inc_16(&mut self, register: &CpuRegister16) -> u16 {
        let target = self.registers.get_target_16(register);
        let value = target.wrapping_add(1);
        self.registers.set_target_16(register, value);
        8
    }

    pub fn or_a(&mut self, register: &CpuRegister) -> u16 {
        let target = self.registers.get_target(register);
        let result = self.or(target);
        self.registers.a = result as u8;
        4
    }

    pub fn or_d8(&mut self) -> u16 {
        let target = self.read_byte();
        let result = self.or(target);
        self.registers.a = result;
        8
    }

    pub fn or_hl(&mut self) -> u16 {
        let target = self.mmu.read_byte(self.registers.get_target_16(&CpuRegister16::HL));
        let result = self.or(target);
        self.registers.a = result;
        8
    }

    pub fn sbc_a(&mut self, register: &CpuRegister) -> u16 {
        let target = self.registers.get_target(register);
        let result = self.sub(target, true);
        self.registers.a = result as u8;
        4
    }

    pub fn sbc_d8(&mut self) -> u16 {
        let target = self.read_byte();
        let result = self.sub(target, true);
        self.registers.a = result;
        8
    }

    pub fn sbc_hl(&mut self) -> u16 {
        let target = self.mmu.read_byte(self.registers.get_target_16(&CpuRegister16::HL));
        let result = self.sub(target, true);
        self.registers.a = result;
        8
    }

    pub fn scf(&mut self) -> u16 {
        self.registers.f.set_carry(true);
        self.registers.f.set_half_carry(false);
        self.registers.f.set_subtraction(false);
        4
    }

    pub fn sub_a(&mut self, register: &CpuRegister) -> u16 {
        let target = self.registers.get_target(register);
        let result = self.sub(target, false);
        self.registers.a = result as u8;
        4
    }

    pub fn sub_d8(&mut self) -> u16 {
        let target = self.read_byte();
        let result = self.sub(target, false);
        self.registers.a = result;
        8
    }

    pub fn sub_hl(&mut self) -> u16 {
        let target = self.mmu.read_byte(self.registers.get_target_16(&CpuRegister16::HL));
        let result = self.sub(target, false);
        self.registers.a = result;
        8
    }

    pub fn xor_a(&mut self, register: &CpuRegister) -> u16 {
        let target = self.registers.get_target(register);
        let result = self.xor(target);
        self.registers.a = result as u8;
        4
    }

    pub fn xor_d8(&mut self) -> u16 {
        let target = self.read_byte();
        let result = self.xor(target);
        self.registers.a = result;
        8
    }

    pub fn xor_hl(&mut self) -> u16 {
        let target = self.mmu.read_byte(self.registers.get_target_16(&CpuRegister16::HL));
        let result = self.xor(target);
        self.registers.a = result;
        8
    }
}