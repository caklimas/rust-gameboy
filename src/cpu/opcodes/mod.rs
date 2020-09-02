pub mod opcode;
pub mod opcode_table;

#[cfg(test)]
mod tests;

use opcode::{Condition, CpuRegister, CpuRegister16};
use super::super::addresses::ld_opcode::LD_ADDRESS_LOWER;

pub type ClockCycle = (u16, u16);

const LOWER_NIBBLE: u8 = 0xF;

impl super::Cpu {
    pub fn adc_a(&mut self, register: &CpuRegister) -> ClockCycle {
        let target = self.registers.get_target(register);
        let result = self.add(target, true);
        self.registers.a = result as u8;
        (1, 4)
    }

    pub fn adc_d8(&mut self) -> ClockCycle {
        let target = self.read_next_byte();
        let result = self.add(target, true);
        self.registers.a = result;
        (2, 8)
    }

    pub fn adc_hl(&mut self) -> ClockCycle {
        let target = self.mmu.read_byte(self.registers.get_target_16(&CpuRegister16::HL));
        let result = self.add(target, true);
        self.registers.a = result;
        (1, 8)
    }

    pub fn add_a(&mut self, register: &CpuRegister) -> ClockCycle {
        let target = self.registers.get_target(register);
        let result = self.add(target, false);
        self.registers.a = result as u8;
        (1, 4)
    }

    pub fn add_d8(&mut self) -> ClockCycle {
        let target = self.read_next_byte();
        let result = self.add(target, false);
        self.registers.a = result;
        (2, 8)
    }

    pub fn add_hl(&mut self) -> ClockCycle {
        let target = self.mmu.read_byte(self.registers.get_target_16(&CpuRegister16::HL));
        let result = self.add(target, false);
        self.registers.a = result;
        (1, 8)
    }

    pub fn and_a(&mut self, register: &CpuRegister) -> ClockCycle {
        let target = self.registers.get_target(register);
        let result = self.and(target);
        self.registers.a = result;
        (1, 4)
    }

    pub fn and_d8(&mut self) -> ClockCycle {
        let target = self.read_next_byte();
        let result = self.and(target);
        self.registers.a = result;
        (2, 8)
    }

    pub fn and_hl(&mut self) -> ClockCycle {
        let target = self.mmu.read_byte(self.registers.get_target_16(&CpuRegister16::HL));
        let result = self.and(target);
        self.registers.a = result;
        (1, 8)
    }

    pub fn call(&mut self) -> ClockCycle {
        self.push_stack(self.program_counter);
        self.program_counter = self.read_next_word();
        (3, 24)
    }

    pub fn call_cc(&mut self, condition: &Condition) -> ClockCycle {
        if !self.is_condition_met(condition) {
            return (3, 12);
        }

        self.call()
    }

    pub fn ccf(&mut self) -> ClockCycle {
        self.registers.f.set_carry(!self.registers.f.carry());
        self.registers.f.set_half_carry(false);
        self.registers.f.set_subtraction(false);
        (1, 4)
    }

    pub fn cp_a(&mut self, register: &CpuRegister) -> ClockCycle {
        let target = self.registers.get_target(register);
        self.sub(target, false);
        (1, 4)
    }

    pub fn cp_d8(&mut self) -> ClockCycle {
        let target = self.read_next_byte();
        self.sub(target, false);
        (2, 8)
    }

    pub fn cp_hl(&mut self) -> ClockCycle {
        let target = self.mmu.read_byte(self.registers.get_target_16(&CpuRegister16::HL));
        self.sub(target, false);
        (1, 8)
    }

    pub fn cpl(&mut self) -> ClockCycle {
        self.registers.a = !self.registers.a;
        self.registers.f.set_half_carry(true);
        self.registers.f.set_subtraction(true);
        (1, 4)
    }

    pub fn dec_hl(&mut self) -> ClockCycle {
        let address = self.registers.get_target_16(&CpuRegister16::HL);
        let target = self.mmu.read_byte(address);
        let data = self.dec_8(target);
        self.mmu.write_byte(address, data);
        (1, 12)
    }

    pub fn dec_r(&mut self, register: &CpuRegister) -> ClockCycle {
        let target = self.registers.get_target(register);
        let value = self.dec_8(target);
        self.registers.set_target(register, value);
        (1, 4)
    }

    pub fn dec_sp(&mut self) -> ClockCycle {
        let target = self.mmu.read_byte(self.stack_pointer);
        let data = target.wrapping_sub(1);
        self.mmu.write_byte(self.stack_pointer, data);
        (1, 8)
    }

    pub fn dec_16(&mut self, register: &CpuRegister16) -> ClockCycle {
        let address = self.registers.get_target_16(register);
        let target = self.mmu.read_byte(address);
        let data = target.wrapping_sub(1);
        self.mmu.write_byte(address, data);
        (1, 8)
    }

    pub fn ei(&mut self) -> ClockCycle {
        self.interrupt_master_enable = true;
        (1, 4)
    }

    pub fn inc_hl(&mut self) -> ClockCycle {
        let address = self.registers.get_target_16(&CpuRegister16::HL);
        let target = self.mmu.read_byte(address);
        let data = self.inc_8(target);
        self.mmu.write_byte(address, data);
        (1, 12)
    }

    pub fn inc_r(&mut self, register: &CpuRegister) -> ClockCycle {
        let target = self.registers.get_target(register);
        let value = self.inc_8(target);
        self.registers.set_target(register, value);
        (1, 4)
    }

    pub fn inc_sp(&mut self) -> ClockCycle {
        let target = self.mmu.read_byte(self.stack_pointer);
        let data = target.wrapping_add(1);
        self.mmu.write_byte(self.stack_pointer, data);
        (1, 8)
    }

    pub fn inc_16(&mut self, register: &CpuRegister16) -> ClockCycle {
        let address = self.registers.get_target_16(register);
        let target = self.mmu.read_byte(address);
        let data = target.wrapping_add(1);
        self.mmu.write_byte(address, data);
        (1, 8)
    }

    pub fn jp(&mut self) -> ClockCycle {
        self.program_counter = self.read_next_word();
        (3, 16)
    }

    pub fn jp_cc(&mut self, condition: &Condition) -> ClockCycle {
        if !self.is_condition_met(condition) {
            return (3, 12);
        }

        self.jp()
    }

    pub fn jp_hl(&mut self) -> ClockCycle {
        let address = self.registers.get_target_16(&CpuRegister16::HL);
        self.program_counter = self.mmu.read_word(address);
        (1, 4)
    }

    pub fn jr(&mut self) -> ClockCycle {
        let n = self.read_next_byte() as i8;
        self.program_counter = ((self.program_counter as u32 as i32) + (n as i32)) as u16;
        (2, 12)
    }

    pub fn jr_cc(&mut self, condition: &Condition) -> ClockCycle {
        if !self.is_condition_met(condition) {
            return(2, 8);
        }

        self.jr()
    }

    pub fn ld(&mut self, dest: &CpuRegister, src: &CpuRegister) -> ClockCycle {
        let value = self.registers.get_target(src);
        self.registers.set_target(dest, value);
        (1, 4)
    }

    pub fn ld_a8_a(&mut self) -> ClockCycle {
        let address = LD_ADDRESS_LOWER + (self.read_next_byte() as u16);
        self.mmu.write_byte(address, self.registers.a);
        (2, 12)
    }

    pub fn ld_a_a8(&mut self) -> ClockCycle {
        let address = LD_ADDRESS_LOWER + (self.read_next_byte() as u16);
        let value = self.mmu.read_byte(address);
        self.registers.set_target(&CpuRegister::A, value);
        (2, 12)
    }

    pub fn ld_a16_a(&mut self) -> ClockCycle {
        let address = self.read_next_word();
        self.mmu.write_byte(address, self.registers.a);
        (3, 16)
    }

    pub fn ld_a_a16(&mut self) -> ClockCycle {
        let address = self.read_next_word();
        let value = self.mmu.read_byte(address);
        self.registers.set_target(&CpuRegister::A, value);
        (3, 16)
    }

    pub fn ld_a_c(&mut self) -> ClockCycle {
        let address = LD_ADDRESS_LOWER + (self.registers.c as u16);
        let value = self.mmu.read_byte(address);
        self.registers.set_target(&CpuRegister::A, value);
        (2, 8)
    }

    pub fn ld_c_a(&mut self) -> ClockCycle {
        let data = self.registers.a;
        let address = LD_ADDRESS_LOWER + (self.registers.get_target(&CpuRegister::C) as u16);
        self.mmu.write_byte(address, data);
        (2, 8)
    }

    pub fn ld_d8(&mut self, dest: &CpuRegister) -> ClockCycle {
        let value = self.read_next_byte();
        self.registers.set_target(dest, value);
        (2, 8)
    }

    pub fn ld_hl_d8(&mut self) -> ClockCycle {
        let address = self.registers.get_target_16(&CpuRegister16::HL);
        let data = self.read_next_byte();
        self.mmu.write_byte(address, data);
        (2, 12)
    }

    pub fn ld_hl_a(&mut self, increment: &bool) -> ClockCycle {
        let data = self.registers.get_target(&CpuRegister::A);
        let address = self.registers.get_target_16(&CpuRegister16::HL);
        self.mmu.write_byte(address, data);

        if increment.clone() {
            self.registers.set_target_16(&CpuRegister16::HL, address + 1);
        } else {
            self.registers.set_target_16(&CpuRegister16::HL, address - 1);
        }

        (1, 8)
    }

    pub fn ld_a_hl(&mut self, increment: &bool) -> ClockCycle {
        let address = self.registers.get_target_16(&CpuRegister16::HL);
        let value = self.mmu.read_byte(address);
        self.registers.set_target(&CpuRegister::A, value);

        if increment.clone() {
            self.registers.set_target_16(&CpuRegister16::HL, address + 1);
        } else {
            self.registers.set_target_16(&CpuRegister16::HL, address - 1);
        }

        (1, 8)
    }

    pub fn ld_a16_sp(&mut self) -> ClockCycle {
        let address = self.read_next_word();
        self.mmu.write_word(address, self.stack_pointer);
        (3, 20)
    }

    pub fn ld_16_r(&mut self, dest: &CpuRegister16, src: &CpuRegister) -> ClockCycle {
        let address = self.registers.get_target_16(dest);
        let data = self.registers.get_target(src);
        self.mmu.write_byte(address, data);
        (1, 8)
    }

    pub fn ld_r_16(&mut self, dest: &CpuRegister, src: &CpuRegister16) -> ClockCycle {
        let address = self.registers.get_target_16(src);
        let value = self.mmu.read_byte(address);
        self.registers.set_target(dest, value);
        (1, 8)
    }

    pub fn ld_r16_d16(&mut self, register: &CpuRegister16) -> ClockCycle {
        let value = self.read_next_word();
        self.registers.set_target_16(register, value);
        (3, 12)
    }

    pub fn ld_sp_d16(&mut self) -> ClockCycle {
        let value = self.read_next_word();
        self.stack_pointer = value;
        (3, 12)
    }

    pub fn ld_sp_e8(&mut self) -> ClockCycle {
        let e = self.read_next_byte() as i8;
        let (value, overflow) = self.stack_pointer.overflowing_add(e as u16);
        self.registers.set_target_16(&CpuRegister16::HL, value);

        self.registers.f.set_carry(overflow);
        self.registers.f.set_half_carry(is_half_carry(self.stack_pointer as u8, e as u8, false));
        self.registers.f.set_subtraction(false);
        self.registers.f.set_zero(false);
        (2, 12)
    }

    pub fn ld_sp_hl(&mut self) -> ClockCycle {
        let address = self.registers.get_target_16(&CpuRegister16::HL);
        let data = self.mmu.read_byte(address);
        self.mmu.write_byte(self.stack_pointer, data);
        (1, 8)
    }

    pub fn or_a(&mut self, register: &CpuRegister) -> ClockCycle {
        let target = self.registers.get_target(register);
        let result = self.or(target);
        self.registers.a = result as u8;
        (1, 4)
    }

    pub fn or_d8(&mut self) -> ClockCycle {
        let target = self.read_next_byte();
        let result = self.or(target);
        self.registers.a = result;
        (2, 8)
    }

    pub fn or_hl(&mut self) -> ClockCycle {
        let target = self.mmu.read_byte(self.registers.get_target_16(&CpuRegister16::HL));
        let result = self.or(target);
        self.registers.a = result;
        (1, 8)
    }

    pub fn pop(&mut self, register: &CpuRegister16) -> ClockCycle {
        let value = self.pop_stack();
        self.registers.set_target_16(register, value);
        (1, 12)
    }

    pub fn push(&mut self, register: &CpuRegister16) -> ClockCycle {
        let data = self.registers.get_target_16(register);
        self.push_stack(data);
        (1, 12)
    }

    pub fn ret(&mut self) -> ClockCycle {
        self.program_counter = self.pop_stack();
        (1, 20)
    }

    pub fn ret_cc(&mut self, condition: &Condition) -> ClockCycle {
        if !self.is_condition_met(condition) {
            return (1, 8);
        }

        self.ret()
    }

    pub fn ret_i(&mut self) -> ClockCycle {
        self.ei();
        self.ret()
    }

    pub fn rst(&mut self, reset_value: u16) -> ClockCycle {
        self.push_stack(self.program_counter);
        self.program_counter = reset_value;
        (1, 16)
    }

    pub fn sbc_a(&mut self, register: &CpuRegister) -> ClockCycle {
        let target = self.registers.get_target(register);
        let result = self.sub(target, true);
        self.registers.a = result as u8;
        (1, 4)
    }

    pub fn sbc_d8(&mut self) -> ClockCycle {
        let target = self.read_next_byte();
        let result = self.sub(target, true);
        self.registers.a = result;
        (2, 8)
    }

    pub fn sbc_hl(&mut self) -> ClockCycle {
        let target = self.mmu.read_byte(self.registers.get_target_16(&CpuRegister16::HL));
        let result = self.sub(target, true);
        self.registers.a = result;
        (1, 8)
    }

    pub fn scf(&mut self) -> ClockCycle {
        self.registers.f.set_carry(true);
        self.registers.f.set_half_carry(false);
        self.registers.f.set_subtraction(false);
        (1, 4)
    }

    pub fn sub_a(&mut self, register: &CpuRegister) -> ClockCycle {
        let target = self.registers.get_target(register);
        let result = self.sub(target, false);
        self.registers.a = result as u8;
        (1, 4)
    }

    pub fn sub_d8(&mut self) -> ClockCycle {
        let target = self.read_next_byte();
        let result = self.sub(target, false);
        self.registers.a = result;
        (2, 8)
    }

    pub fn sub_hl(&mut self) -> ClockCycle {
        let target = self.mmu.read_byte(self.registers.get_target_16(&CpuRegister16::HL));
        let result = self.sub(target, false);
        self.registers.a = result;
        (1, 8)
    }

    pub fn xor_a(&mut self, register: &CpuRegister) -> ClockCycle {
        let target = self.registers.get_target(register);
        let result = self.xor(target);
        self.registers.a = result as u8;
        (1, 4)
    }

    pub fn xor_d8(&mut self) -> ClockCycle {
        let target = self.read_next_byte();
        let result = self.xor(target);
        self.registers.a = result;
        (2, 8)
    }

    pub fn xor_hl(&mut self) -> ClockCycle {
        let target = self.mmu.read_byte(self.registers.get_target_16(&CpuRegister16::HL));
        let result = self.xor(target);
        self.registers.a = result;
        (1, 8)
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

    fn is_condition_met(&self, condition: &Condition) -> bool {
        match condition {
            Condition::Z => {
                self.registers.f.zero()
            },
            Condition::NZ => {
                !self.registers.f.zero()
            },
            Condition::C => {
                self.registers.f.carry()
            },
            Condition::NC => {
                !self.registers.f.carry()
            }
        }
    }

    fn or(&mut self, target: u8) -> u8 {
        let result = self.registers.a | target;
        self.registers.f.set_carry(false);
        self.registers.f.set_half_carry(false);
        self.registers.f.set_subtraction(false);
        self.registers.f.set_zero(result == 0);

        result
    }

    fn pop_stack(&mut self) -> u16 {
        self.stack_pointer += 2;
        self.mmu.read_word(self.stack_pointer)
    }

    fn push_stack(&mut self, value: u16) {
        self.mmu.write_word(self.stack_pointer, value);
        self.stack_pointer -= 2;
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