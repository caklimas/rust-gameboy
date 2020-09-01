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
        let target = self.read_next_byte();
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
        let target = self.read_next_byte();
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
        let target = self.read_next_byte();
        let result = self.and(target);
        self.registers.a = result;
    }

    pub fn and_hl(&mut self) {
        let target = self.mmu.read_byte(self.registers.get_target_16(&CpuRegister16::HL));
        let result = self.and(target);
        self.registers.a = result;
    }

    pub fn ccf(&mut self) {
        self.registers.f.set_carry(!self.registers.f.carry());
        self.registers.f.set_half_carry(false);
        self.registers.f.set_subtraction(false);
    }

    pub fn cp_a(&mut self, register: &CpuRegister) {
        let target = self.registers.get_target(register);
        self.sub(target, false);
    }

    pub fn cp_d8(&mut self) {
        let target = self.read_next_byte();
        self.sub(target, false);
    }

    pub fn cp_hl(&mut self) {
        let target = self.mmu.read_byte(self.registers.get_target_16(&CpuRegister16::HL));
        self.sub(target, false);
    }

    pub fn cpl(&mut self) {
        self.registers.a = !self.registers.a;
        self.registers.f.set_half_carry(true);
        self.registers.f.set_subtraction(true);
    }

    pub fn dec_hl(&mut self) {
        let address = self.registers.get_target_16(&CpuRegister16::HL);
        let target = self.mmu.read_byte(address);
        let data = self.dec_8(target);
        self.mmu.write_byte(address, data);
    }

    pub fn dec_r(&mut self, register: &CpuRegister) {
        let target = self.registers.get_target(register);
        let value = self.dec_8(target);
        self.registers.set_target(register, value);
    }

    pub fn dec_sp(&mut self) {
        let target = self.mmu.read_byte(self.stack_pointer);
        let data = target.wrapping_sub(1);
        self.mmu.write_byte(self.stack_pointer, data);
    }

    pub fn dec_16(&mut self, register: &CpuRegister16) {
        let address = self.registers.get_target_16(register);
        let target = self.mmu.read_byte(address);
        let data = target.wrapping_sub(1);
        self.mmu.write_byte(address, data);
    }

    pub fn inc_hl(&mut self) {
        let address = self.registers.get_target_16(&CpuRegister16::HL);
        let target = self.mmu.read_byte(address);
        let data = self.inc_8(target);
        self.mmu.write_byte(address, data);
    }

    pub fn inc_r(&mut self, register: &CpuRegister) {
        let target = self.registers.get_target(register);
        let value = self.inc_8(target);
        self.registers.set_target(register, value);
    }

    pub fn inc_sp(&mut self) {
        let target = self.mmu.read_byte(self.stack_pointer);
        let data = target.wrapping_add(1);
        self.mmu.write_byte(self.stack_pointer, data);
    }

    pub fn inc_16(&mut self, register: &CpuRegister16) {
        let address = self.registers.get_target_16(register);
        let target = self.mmu.read_byte(address);
        let data = target.wrapping_add(1);
        self.mmu.write_byte(address, data);
    }

    pub fn ld(&mut self, dest: &CpuRegister, src: &CpuRegister) {
        let value = self.registers.get_target(src);
        self.registers.set_target(dest, value);
    }

    pub fn ld_d8(&mut self, dest: &CpuRegister) {
        let value = self.read_next_byte();
        self.registers.set_target(dest, value);
    }

    pub fn ld_hl_d8(&mut self) {
        let address = self.registers.get_target_16(&CpuRegister16::HL);
        let data = self.read_next_byte();
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

    pub fn ld_a16_sp(&mut self) {
        let address = self.read_next_word();
        self.mmu.write_word(address, self.stack_pointer);
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

    pub fn ld_r16_d16(&mut self, register: &CpuRegister16) {
        let value = self.read_next_word();
        self.registers.set_target_16(register, value);
    }

    pub fn ld_sp_d16(&mut self) {
        let value = self.read_next_word();
        self.stack_pointer = value;
    }

    pub fn ld_sp_e8(&mut self) {
        let e = self.read_next_byte() as i8;
        let (value, overflow) = self.stack_pointer.overflowing_add(e as u16);
        self.registers.set_target_16(&CpuRegister16::HL, value);

        self.registers.f.set_carry(overflow);
        self.registers.f.set_half_carry(is_half_carry(self.stack_pointer as u8, e as u8, false));
        self.registers.f.set_subtraction(false);
        self.registers.f.set_zero(false);
    }

    pub fn ld_sp_hl(&mut self) {
        let address = self.registers.get_target_16(&CpuRegister16::HL);
        let data = self.mmu.read_byte(address);
        self.mmu.write_byte(self.stack_pointer, data);
    }

    pub fn or_a(&mut self, register: &CpuRegister) {
        let target = self.registers.get_target(register);
        let result = self.or(target);
        self.registers.a = result as u8;
    }

    pub fn or_d8(&mut self) {
        let target = self.read_next_byte();
        let result = self.or(target);
        self.registers.a = result;
    }

    pub fn or_hl(&mut self) {
        let target = self.mmu.read_byte(self.registers.get_target_16(&CpuRegister16::HL));
        let result = self.or(target);
        self.registers.a = result;
    }

    pub fn pop(&mut self, register: &CpuRegister16) {
        self.stack_pointer += 2;
        let value = self.mmu.read_word(self.stack_pointer);
        self.registers.set_target_16(register, value)
    }

    pub fn push(&mut self, register: &CpuRegister16) {
        let data = self.registers.get_target_16(register);
        self.mmu.write_word(self.stack_pointer, data);
        self.stack_pointer -= 2;
    }

    pub fn sbc_a(&mut self, register: &CpuRegister) {
        let target = self.registers.get_target(register);
        let result = self.sub(target, true);
        self.registers.a = result as u8;
    }

    pub fn sbc_d8(&mut self) {
        let target = self.read_next_byte();
        let result = self.sub(target, true);
        self.registers.a = result;
    }

    pub fn sbc_hl(&mut self) {
        let target = self.mmu.read_byte(self.registers.get_target_16(&CpuRegister16::HL));
        let result = self.sub(target, true);
        self.registers.a = result;
    }

    pub fn scf(&mut self) {
        self.registers.f.set_carry(true);
        self.registers.f.set_half_carry(false);
        self.registers.f.set_subtraction(false);
    }

    pub fn sub_a(&mut self, register: &CpuRegister) {
        let target = self.registers.get_target(register);
        let result = self.sub(target, false);
        self.registers.a = result as u8;
    }

    pub fn sub_d8(&mut self) {
        let target = self.read_next_byte();
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
        let target = self.read_next_byte();
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
        self.registers.f.set_half_carry(is_half_carry(self.registers.a, target + carry, false));
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

    fn dec_8(&mut self, target: u8) -> u8 {
        let result = target.wrapping_sub(1);
        self.registers.f.set_half_carry(is_half_carry(target, 1, true));
        self.registers.f.set_subtraction(true);
        self.registers.f.set_zero(result == 0);

        result
    }

    fn inc_8(&mut self, target: u8) -> u8 {
        let result = target.wrapping_add(1);
        self.registers.f.set_half_carry(is_half_carry(target, 1, false));
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
        self.registers.f.set_half_carry(is_half_carry(self.registers.a, target - carry, false));
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

fn is_half_carry(register: u8, value: u8, subtract: bool) -> bool {
    let result = if subtract {
        (register & LOWER_NIBBLE).wrapping_sub(value & LOWER_NIBBLE)
    } else {
        (register & LOWER_NIBBLE).wrapping_add(value & LOWER_NIBBLE)
    };

    result & 0x10 == 0x10
}