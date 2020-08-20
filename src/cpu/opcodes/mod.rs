pub mod instruction;

#[cfg(test)]
mod opcodes_tests;

use instruction::ArithmeticRegister;

const LOWER_NIBBLE: u8 = 0xF;

impl super::Cpu {
    pub fn add_a(&mut self, register: ArithmeticRegister) {
        let result = self.add(register);
        self.registers.a = result as u8;
    }

    pub fn add_hl(&mut self, register: ArithmeticRegister) {
        let result = self.add(register);
        self.registers.set_hl(result);
    }

    fn add(&mut self, register: ArithmeticRegister) -> u16 {
        let target = self.registers.get_target(register);
        let (result, overflow) = self.registers.a.overflowing_add(target);
        self.registers.f.set_carry(overflow);
        self.registers.f.set_half_carry(is_half_carry(self.registers.a, target));
        self.registers.f.set_subtraction(false);
        self.registers.f.set_zero(result == 0);

        result as u16
    }
}

fn is_half_carry(register: u8, value: u8) -> bool {
    (register & LOWER_NIBBLE) + (value & LOWER_NIBBLE) > LOWER_NIBBLE
}