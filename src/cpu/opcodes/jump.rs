use super::opcode::{Condition, CpuRegister16};

impl super::super::Cpu {
    pub fn call(&mut self) -> u16 {
        self.push_stack(self.program_counter + 2);
        self.program_counter = self.read_word();
        24
    }

    pub fn call_cc(&mut self, condition: &Condition) -> u16 {
        if !self.is_condition_met(condition) {
            self.program_counter += 2;
            return 12;
        }

        self.call()
    }

    pub fn jp(&mut self) -> u16 {
        self.program_counter = self.read_word();
        16
    }

    pub fn jp_cc(&mut self, condition: &Condition) -> u16 {
        if !self.is_condition_met(condition) {
            self.program_counter += 2;
            return 12;
        }

        self.jp()
    }

    pub fn jp_hl(&mut self) -> u16 {
        self.program_counter = self.registers.get_target_16(&CpuRegister16::HL);
        4
    }

    pub fn jr(&mut self) -> u16 {
        let n = self.read_byte() as i8;
        self.program_counter = ((self.program_counter as u32 as i32) + (n as i32)) as u16;
        12
    }

    pub fn jr_cc(&mut self, condition: &Condition) -> u16 {
        if !self.is_condition_met(condition) {
            self.program_counter += 1;
            return 8;
        }

        self.jr()
    }

    pub fn ret(&mut self) -> u16 {
        self.program_counter = self.pop_stack();
        20
    }

    pub fn ret_cc(&mut self, condition: &Condition) -> u16 {
        if !self.is_condition_met(condition) {
            return 8;
        }

        self.ret()
    }

    pub fn ret_i(&mut self) -> u16 {
        self.ei();
        self.ret()
    }

    pub fn rst(&mut self, reset_value: u16) -> u16 {
        self.push_stack(self.program_counter);
        self.program_counter = reset_value;
        16
    }
}