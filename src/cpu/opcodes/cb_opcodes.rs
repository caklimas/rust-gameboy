use super::opcode::{CpuRegister, CpuRegister16};
use super::ClockCycle;

impl super::super::Cpu {
    pub fn rl_hl(&mut self) -> ClockCycle {
        let address = self.registers.get_target_16(&CpuRegister16::HL);
        let value = self.mmu.read_byte(address);
        let data = self.rl_8(value);

        self.mmu.write_byte(address, data);

        (2, 16)
    }
    
    pub fn rl_r8(&mut self, register: &CpuRegister ) -> ClockCycle {
        let register_value = self.registers.get_target(register);
        let new_value = self.rl_8(register_value);

        self.registers.set_target(register, new_value);
        (2, 8)
    }

    pub fn rlc_hl(&mut self) -> ClockCycle {
        let address = self.registers.get_target_16(&CpuRegister16::HL);
        let value = self.mmu.read_byte(address);
        let data = self.rlc_8(value);

        self.mmu.write_byte(address, data);

        (2, 16)
    }

    pub fn rlc_r8(&mut self, register: &CpuRegister ) -> ClockCycle {
        let register_value = self.registers.get_target(register);
        let new_value = self.rlc_8(register_value);

        self.registers.set_target(register, new_value);
        (2, 8)
    }

    pub fn rr_hl(&mut self) -> ClockCycle {
        let address = self.registers.get_target_16(&CpuRegister16::HL);
        let value = self.mmu.read_byte(address);
        let data = self.rr_8(value);

        self.mmu.write_byte(address, data);

        (2, 16)
    }
    
    pub fn rr_r8(&mut self, register: &CpuRegister ) -> ClockCycle {
        let register_value = self.registers.get_target(register);
        let new_value = self.rr_8(register_value);

        self.registers.set_target(register, new_value);
        (2, 8)
    }

    pub fn rrc_hl(&mut self) -> ClockCycle {
        let address = self.registers.get_target_16(&CpuRegister16::HL);
        let value = self.mmu.read_byte(address);
        let data = self.rrc_8(value);

        self.mmu.write_byte(address, data);

        (2, 16)
    }

    pub fn rrc_r8(&mut self, register: &CpuRegister ) -> ClockCycle {
        let register_value = self.registers.get_target(register);
        let new_value = self.rrc_8(register_value);

        self.registers.set_target(register, new_value);
        (2, 8)
    }

    pub fn sla_hl(&mut self) -> ClockCycle {
        let address = self.registers.get_target_16(&CpuRegister16::HL);
        let value = self.mmu.read_byte(address);
        let data = self.sla_8(value);

        self.mmu.write_byte(address, data);

        (2, 16)
    }

    pub fn sla_r8(&mut self, register: &CpuRegister ) -> ClockCycle {
        let register_value = self.registers.get_target(register);
        let new_value = self.sla_8(register_value);

        self.registers.set_target(register, new_value);
        (2, 8)
    }

    pub fn sra_hl(&mut self) -> ClockCycle {
        let address = self.registers.get_target_16(&CpuRegister16::HL);
        let value = self.mmu.read_byte(address);
        let data = self.sra_8(value);

        self.mmu.write_byte(address, data);

        (2, 16)
    }

    pub fn sra_r8(&mut self, register: &CpuRegister ) -> ClockCycle {
        let register_value = self.registers.get_target(register);
        let new_value = self.sra_8(register_value);

        self.registers.set_target(register, new_value);
        (2, 8)
    }
}