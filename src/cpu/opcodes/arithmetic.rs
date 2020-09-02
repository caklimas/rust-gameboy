use super::opcode::{CpuRegister, CpuRegister16};
use super::ClockCycle;

impl super::super::Cpu {
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

    pub fn add_hl_16(&mut self, register: &CpuRegister16) -> ClockCycle {
        let hl_register = &CpuRegister16::HL;
        let hl_register_value = self.registers.get_target_16(hl_register);
        let value = self.registers.get_target_16(register);
        let (result, overflow) = hl_register_value.overflowing_add(value);
        self.registers.set_target_16(hl_register, result);

        self.registers.f.set_carry(overflow);
        self.registers.f.set_half_carry(super::is_half_carry_16(hl_register_value, value));
        self.registers.f.set_subtraction(false);

        (1, 8)
    }

    pub fn add_hl_16_sp(&mut self) -> ClockCycle {
        let hl_register = &CpuRegister16::HL;
        let hl_register_value = self.registers.get_target_16(hl_register);
        let (result, overflow) = hl_register_value.overflowing_add(self.stack_pointer);
        self.registers.set_target_16(hl_register, result);

        self.registers.f.set_carry(overflow);
        self.registers.f.set_half_carry(super::is_half_carry_16(hl_register_value, self.stack_pointer));
        self.registers.f.set_subtraction(false);

        (1, 8)
    }

    pub fn add_d8(&mut self) -> ClockCycle {
        let target = self.read_next_byte();
        let result = self.add(target, false);
        self.registers.a = result;
        (2, 8)
    }

    pub fn add_a_hl(&mut self) -> ClockCycle {
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
}