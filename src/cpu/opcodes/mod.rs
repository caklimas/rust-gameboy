pub mod arithmetic;
pub mod cb_opcode;
pub mod cb_opcode_table;
pub mod cb_opcodes;
pub mod jump;
pub mod load;
pub mod misc;
pub mod opcode;
pub mod opcode_table;
pub mod stack;

#[cfg(test)]
mod tests;

use opcode::Condition;

const LOWER_NIBBLE_8: u8 = 0xF;
const LOWER_NIBBLE_16: u16 = 0x7FF;

impl super::Cpu {
    fn add(&mut self, target: u8, include_carry: bool) -> u8 {
        let carry = if include_carry && self.registers.f.carry() {
            1
        } else {
            0
        };
        let (result, overflow) = self.registers.a.overflowing_add(target);
        let (result_2, overflow_2) = result.overflowing_add(carry);
        self.registers.f.set_carry(overflow | overflow_2);
        self.registers
            .f
            .set_half_carry(is_half_carry_8(self.registers.a, target, false, carry));
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

    fn bit_n_set(&mut self, bit_n: u8, value: u8) {
        let result = value & (1 << bit_n);
        self.registers.f.set_half_carry(true);
        self.registers.f.set_subtraction(false);
        self.registers.f.set_zero(result == 0);
    }

    fn dec_8(&mut self, target: u8) -> u8 {
        let result = target.wrapping_sub(1);
        self.registers.f.set_half_carry((target & 0x0F) == 0);
        self.registers.f.set_subtraction(true);
        self.registers.f.set_zero(result == 0);

        result
    }

    fn inc_8(&mut self, target: u8) -> u8 {
        let result = target.wrapping_add(1);
        self.registers
            .f
            .set_half_carry(is_half_carry_8(target, 1, false, 0));
        self.registers.f.set_subtraction(false);
        self.registers.f.set_zero(result == 0);

        result
    }

    fn is_condition_met(&self, condition: &Condition) -> bool {
        match condition {
            Condition::Z => self.registers.f.zero(),
            Condition::NZ => !self.registers.f.zero(),
            Condition::C => self.registers.f.carry(),
            Condition::NC => !self.registers.f.carry(),
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
        let data = self.mmu.read_word(self.stack_pointer);
        self.stack_pointer += 2;
        data
    }

    fn push_stack(&mut self, value: u16) {
        self.stack_pointer -= 2;
        self.mmu.write_word(self.stack_pointer, value);
    }

    fn res_8(&self, value: u8, bit_n: u8) -> u8 {
        value & !(1 << bit_n)
    }

    fn rl_8(&mut self, value: u8) -> u8 {
        // C <- [7 <- 0] <- C
        let carry = self.registers.f.carry();
        let bit_7 = value & 0b1000_0000;

        let shifted = value << 1;
        let result = if carry { shifted | 0b1 } else { shifted & !0b1 };

        self.registers.f.set_carry(bit_7 > 0);
        self.registers.f.set_half_carry(false);
        self.registers.f.set_subtraction(false);
        self.registers.f.set_zero(result == 0);

        result
    }

    fn rlc_8(&mut self, value: u8) -> u8 {
        // C <- [7 <- 0] <- [7]
        let bit_7 = value & 0b1000_0000;

        let shifted = value << 1;
        let result = if bit_7 > 0 {
            shifted | 0b1
        } else {
            shifted & !0b1
        };

        self.registers.f.set_carry(bit_7 > 0);
        self.registers.f.set_half_carry(false);
        self.registers.f.set_subtraction(false);
        self.registers.f.set_zero(result == 0);
        result
    }

    fn rr_8(&mut self, value: u8) -> u8 {
        // C -> [7 -> 0] -> C
        let bit_0 = value & 0b1;
        let carry = self.registers.f.carry();

        let shifted = value >> 1;
        let result = if carry {
            shifted | 0b1000_0000
        } else {
            shifted & !0b1000_0000
        };

        self.registers.f.set_carry(bit_0 > 0);
        self.registers.f.set_half_carry(false);
        self.registers.f.set_subtraction(false);
        self.registers.f.set_zero(result == 0);
        result
    }

    fn rrc_8(&mut self, value: u8) -> u8 {
        // [0] -> [7 -> 0] -> C
        let bit_0 = value & 0b1;

        let shifted = value >> 1;
        let result = if bit_0 > 0 {
            shifted | 0b1000_0000
        } else {
            shifted & !0b1000_0000
        };

        self.registers.f.set_carry(bit_0 > 0);
        self.registers.f.set_half_carry(false);
        self.registers.f.set_subtraction(false);
        self.registers.f.set_zero(result == 0);
        result
    }

    fn set_8(&self, value: u8, bit_n: u8) -> u8 {
        value | (1 << bit_n)
    }

    fn sla_8(&mut self, value: u8) -> u8 {
        // C <- [7 <- 0] <- 0
        let bit_7 = value & 0b1000_0000;
        let result = value << 1;

        self.registers.f.set_carry(bit_7 > 0);
        self.registers.f.set_half_carry(false);
        self.registers.f.set_subtraction(false);
        self.registers.f.set_zero(result == 0);

        result
    }

    fn sra_8(&mut self, value: u8) -> u8 {
        // [7] -> [7 -> 0] -> C
        let bit_7 = value & 0b1000_0000;
        let bit_0 = value & 0b1;
        let shifted = value >> 1;
        let result = if bit_7 > 0 {
            shifted | bit_7
        } else {
            shifted & !bit_7
        };

        self.registers.f.set_carry(bit_0 > 0);
        self.registers.f.set_half_carry(false);
        self.registers.f.set_subtraction(false);
        self.registers.f.set_zero(result == 0);

        result
    }

    fn srl_8(&mut self, value: u8) -> u8 {
        // 0 -> [7 -> 0] -> C
        let bit_0 = value & 0b1;
        let result = value >> 1;

        self.registers.f.set_carry(bit_0 > 0);
        self.registers.f.set_half_carry(false);
        self.registers.f.set_subtraction(false);
        self.registers.f.set_zero(result == 0);

        result
    }

    fn sub(&mut self, target: u8, include_carry: bool) -> u8 {
        let carry = if include_carry && self.registers.f.carry() {
            1
        } else {
            0
        };
        let (result, overflow) = self.registers.a.overflowing_sub(target);
        let (result_2, overflow_2) = result.overflowing_sub(carry);
        self.registers.f.set_carry(overflow | overflow_2);
        self.registers
            .f
            .set_half_carry(is_half_carry_8(self.registers.a, target, true, carry));
        self.registers.f.set_subtraction(true);
        self.registers.f.set_zero(result_2 == 0);

        result_2
    }

    fn swap(&mut self, value: u8) -> u8 {
        let high = value & 0xF0;
        let low = value & 0x0F;
        let result = (low << 4) | (high >> 4);

        self.registers.f.set_carry(false);
        self.registers.f.set_half_carry(false);
        self.registers.f.set_subtraction(false);
        self.registers.f.set_zero(result == 0);

        result
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

fn is_half_carry_8(left: u8, right: u8, subtract: bool, carry: u8) -> bool {
    if subtract {
        (left & 0x0F) < (right & 0x0F) + carry
    } else {
        (left & LOWER_NIBBLE_8) + (right & LOWER_NIBBLE_8) + carry > LOWER_NIBBLE_8
    }
}

fn is_half_carry_16(left: u16, right: u16) -> bool {
    (left & LOWER_NIBBLE_16) + (right & LOWER_NIBBLE_16) > LOWER_NIBBLE_16
}

fn is_overflow_8(left: u16, right: u16) -> bool {
    (left & 0x00FF) + (right & 0x00FF) > 0x00FF
}
