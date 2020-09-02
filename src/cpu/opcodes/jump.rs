use super::opcode::{Condition, CpuRegister16};
use super::ClockCycle;

impl super::super::Cpu {
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
}