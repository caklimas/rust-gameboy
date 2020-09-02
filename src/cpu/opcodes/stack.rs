use super::opcode::{CpuRegister16};
use super::ClockCycle;

impl super::super::Cpu {
    pub fn ld_a16_sp(&mut self) -> ClockCycle {
        let address = self.read_next_word();
        self.mmu.write_word(address, self.stack_pointer);
        (3, 20)
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
        self.registers.f.set_half_carry(super::is_half_carry_8(self.stack_pointer as u8, e as u8, false));
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
}