pub mod arithmetic;
pub mod jump;
pub mod load;
pub mod opcode;
pub mod opcode_table;
pub mod stack;

#[cfg(test)]
mod tests;

use opcode::{Condition};

pub type ClockCycle = (u16, u16);

const HALF_CARRY_8: u8 = 0x10;
const HALF_CARRY_16: u16 = 0x1000;
const LOWER_NIBBLE_8: u8 = 0xF;
const LOWER_NIBBLE_16: u16 = 0xFFF;

impl super::Cpu {
    fn add(&mut self, target: u8, include_carry: bool) -> u8 {
        let carry = if include_carry && self.registers.f.carry() { 1 } else { 0 };
        let (result, overflow) = self.registers.a.overflowing_add(target);
        let (result_2, overflow_2) = result.overflowing_add(carry);
        self.registers.f.set_carry(overflow | overflow_2);
        self.registers.f.set_half_carry(is_half_carry_8(self.registers.a, target + carry, false));
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
        self.registers.f.set_half_carry(is_half_carry_8(target, 1, true));
        self.registers.f.set_subtraction(true);
        self.registers.f.set_zero(result == 0);

        result
    }

    fn inc_8(&mut self, target: u8) -> u8 {
        let result = target.wrapping_add(1);
        self.registers.f.set_half_carry(is_half_carry_8(target, 1, false));
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
        self.registers.f.set_half_carry(is_half_carry_8(self.registers.a, target - carry, false));
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

fn is_half_carry_8(left: u8, right: u8, subtract: bool) -> bool {
    let result = if subtract {
        (left & LOWER_NIBBLE_8).wrapping_sub(right & LOWER_NIBBLE_8)
    } else {
        (left & LOWER_NIBBLE_8).wrapping_add(right & LOWER_NIBBLE_8)
    };

    result & HALF_CARRY_8 == HALF_CARRY_8
}

fn is_half_carry_16(left: u16, right: u16) -> bool {
    let result = (left & LOWER_NIBBLE_16).wrapping_add(right & LOWER_NIBBLE_16);
    result & HALF_CARRY_16 == HALF_CARRY_16
}