pub mod opcode;
pub mod opcode_table;

#[cfg(test)]
mod tests;

use opcode::{CpuRegister, CpuRegister16};

const LOWER_NIBBLE: u8 = 0xF;

impl super::Cpu {
    pub fn adc_a(&mut self, register: &CpuRegister) {
        let target = self.registers.get_target(register);
        let result = self.add(target, true);
        self.registers.a = result as u8;
    }

    pub fn adc_d8(&mut self) {
        let target = self.mmu.read_next_byte(self.program_counter);
        let result = self.add(target, true);
        self.registers.a = result;
    }

    pub fn adc_hl(&mut self) {
        let target = self.mmu.read_byte(self.registers.get_target_16(&CpuRegister16::HL));
        let result = self.add(target, true);
        self.registers.a = result;
    }

    pub fn add_a(&mut self, register: &CpuRegister) {
        let target = self.registers.get_target(register);
        let result = self.add(target, false);
        self.registers.a = result as u8;
    }

    pub fn add_d8(&mut self) {
        let target = self.mmu.read_next_byte(self.program_counter);
        let result = self.add(target, false);
        self.registers.a = result;
    }

    pub fn add_hl(&mut self) {
        let target = self.mmu.read_byte(self.registers.get_target_16(&CpuRegister16::HL));
        let result = self.add(target, false);
        self.registers.a = result;
    }

    pub fn and_a(&mut self, register: &CpuRegister) {
        let target = self.registers.get_target(register);
        let result = self.and(target);
        self.registers.a = result;
    }

    pub fn and_d8(&mut self) {
        let target = self.mmu.read_next_byte(self.program_counter);
        let result = self.and(target);
        self.registers.a = result;
    }

    pub fn and_hl(&mut self) {
        let target = self.mmu.read_byte(self.registers.get_target_16(&CpuRegister16::HL));
        let result = self.and(target);
        self.registers.a = result;
    }

    pub fn cp_a(&mut self, register: &CpuRegister) {
        let target = self.registers.get_target(register);
        self.sub(target, false);
    }

    pub fn cp_d8(&mut self) {
        let target = self.mmu.read_next_byte(self.program_counter);
        self.sub(target, false);
    }

    pub fn cp_hl(&mut self) {
        let target = self.mmu.read_byte(self.registers.get_target_16(&CpuRegister16::HL));
        self.sub(target, false);
    }

    pub fn ld(&mut self, dest: &CpuRegister, src: &CpuRegister) {
        let value = self.registers.get_target(src);
        self.registers.set_target(dest, value);
    }

    pub fn ld_d8(&mut self, dest: &CpuRegister) {
        let value = self.mmu.read_next_byte(self.program_counter);
        self.registers.set_target(dest, value);
    }

    pub fn ld_hl_d8(&mut self) {
        let address = self.registers.get_target_16(&CpuRegister16::HL);
        let data = self.mmu.read_next_byte(self.program_counter);
        self.mmu.write_byte(address, data);
    }

    pub fn ld_hl_a(&mut self, increment: &bool) {
        let data = self.registers.get_target(&CpuRegister::A);
        let address = self.registers.get_target_16(&CpuRegister16::HL);
        self.mmu.write_byte(address, data);

        if increment.clone() {
            self.registers.set_target_16(&CpuRegister16::HL, address + 1);
        } else {
            self.registers.set_target_16(&CpuRegister16::HL, address - 1);
        }
    }

    pub fn ld_a_hl(&mut self, increment: &bool) {
        let address = self.registers.get_target_16(&CpuRegister16::HL);
        let value = self.mmu.read_byte(address);
        self.registers.set_target(&CpuRegister::A, value);

        if increment.clone() {
            self.registers.set_target_16(&CpuRegister16::HL, address + 1);
        } else {
            self.registers.set_target_16(&CpuRegister16::HL, address - 1);
        }
    }

    pub fn ld_16_r(&mut self, dest: &CpuRegister16, src: &CpuRegister) {
        let address = self.registers.get_target_16(dest);
        let data = self.registers.get_target(src);
        self.mmu.write_byte(address, data);

    }

    pub fn ld_r_16(&mut self, dest: &CpuRegister, src: &CpuRegister16) {
        let address = self.registers.get_target_16(src);
        let value = self.mmu.read_byte(address);
        self.registers.set_target(dest, value);
    }

    pub fn or_a(&mut self, register: &CpuRegister) {
        let target = self.registers.get_target(register);
        let result = self.or(target);
        self.registers.a = result as u8;
    }

    pub fn or_d8(&mut self) {
        let target = self.mmu.read_next_byte(self.program_counter);
        let result = self.or(target);
        self.registers.a = result;
    }

    pub fn or_hl(&mut self) {
        let target = self.mmu.read_byte(self.registers.get_target_16(&CpuRegister16::HL));
        let result = self.or(target);
        self.registers.a = result;
    }

    pub fn sbc_a(&mut self, register: &CpuRegister) {
        let target = self.registers.get_target(register);
        let result = self.sub(target, true);
        self.registers.a = result as u8;
    }

    pub fn sbc_d8(&mut self) {
        let target = self.mmu.read_next_byte(self.program_counter);
        let result = self.sub(target, true);
        self.registers.a = result;
    }

    pub fn sbc_hl(&mut self) {
        let target = self.mmu.read_byte(self.registers.get_target_16(&CpuRegister16::HL));
        let result = self.sub(target, true);
        self.registers.a = result;
    }

    pub fn sub_a(&mut self, register: &CpuRegister) {
        let target = self.registers.get_target(register);
        let result = self.sub(target, false);
        self.registers.a = result as u8;
    }

    pub fn sub_d8(&mut self) {
        let target = self.mmu.read_next_byte(self.program_counter);
        let result = self.sub(target, false);
        self.registers.a = result;
    }

    pub fn sub_hl(&mut self) {
        let target = self.mmu.read_byte(self.registers.get_target_16(&CpuRegister16::HL));
        let result = self.sub(target, false);
        self.registers.a = result;
    }

    pub fn xor_a(&mut self, register: &CpuRegister) {
        let target = self.registers.get_target(register);
        let result = self.xor(target);
        self.registers.a = result as u8;
    }

    pub fn xor_d8(&mut self) {
        let target = self.mmu.read_next_byte(self.program_counter);
        let result = self.xor(target);
        self.registers.a = result;
    }

    pub fn xor_hl(&mut self) {
        let target = self.mmu.read_byte(self.registers.get_target_16(&CpuRegister16::HL));
        let result = self.xor(target);
        self.registers.a = result;
    }

    fn add(&mut self, target: u8, include_carry: bool) -> u8 {
        let carry = if include_carry && self.registers.f.carry() { 1 } else { 0 };
        let (result, overflow) = self.registers.a.overflowing_add(target);
        let (result_2, overflow_2) = result.overflowing_add(carry);
        self.registers.f.set_carry(overflow | overflow_2);
        self.registers.f.set_half_carry(is_half_carry(self.registers.a, target + carry));
        self.registers.f.set_subtraction(false);
        self.registers.f.set_zero(result_2 == 0);

        result_2
    }

    fn and(&mut self, target: u8) -> u8 {
        let result = self.registers.a & target;
        self.registers.f.set_carry(false);
        self.registers.f.set_half_carry(true);
        self.registers.f.set_subtraction(false);
        self.registers.f.set_zero(result == 0);

        result
    }

    fn or(&mut self, target: u8) -> u8 {
        let result = self.registers.a | target;
        self.registers.f.set_carry(false);
        self.registers.f.set_half_carry(false);
        self.registers.f.set_subtraction(false);
        self.registers.f.set_zero(result == 0);

        result
    }

    fn sub(&mut self, target: u8, include_carry: bool) -> u8 {
        let carry = if include_carry && self.registers.f.carry() { 1 } else { 0 };
        let (result, overflow) = self.registers.a.overflowing_sub(target);
        let (result_2, overflow_2) = result.overflowing_sub(carry);
        self.registers.f.set_carry(overflow | overflow_2);
        self.registers.f.set_half_carry(is_half_carry(self.registers.a, target - carry));
        self.registers.f.set_subtraction(true);
        self.registers.f.set_zero(result == 0);

        result_2
    }

    fn xor(&mut self, target: u8) -> u8 {
        let result = self.registers.a ^ target;
        self.registers.f.set_carry(false);
        self.registers.f.set_half_carry(false);
        self.registers.f.set_subtraction(false);
        self.registers.f.set_zero(result == 0);

        result
    }
}

fn is_half_carry(register: u8, value: u8) -> bool {
    (register & LOWER_NIBBLE) + (value & LOWER_NIBBLE) > LOWER_NIBBLE
}