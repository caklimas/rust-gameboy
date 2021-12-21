use super::opcode::{CpuRegister, CpuRegister16};

impl super::super::Cpu {
    pub fn bit_n_set_hl(&mut self, bit_n: u8) -> u16 {
        let address = self.registers.get_target_16(&CpuRegister16::HL);
        let value = self.mmu.read_byte(address);
        self.bit_n_set(bit_n, value);

        16
    }

    pub fn bit_n_set_r8(&mut self, register: &CpuRegister, bit_n: u8) -> u16 {
        let value = self.registers.get_target(register);
        self.bit_n_set(bit_n, value);

        8
    }

    pub fn res_hl(&mut self, bit_n: u8) -> u16 {
        let address = self.registers.get_target_16(&CpuRegister16::HL);
        let value = self.mmu.read_byte(address);
        let data = self.res_8(value, bit_n);
        
        self.mmu.write_byte(address, data);

        16
    }

    pub fn res_r8(&mut self, register: &CpuRegister, bit_n: u8) -> u16 {
        let value = self.registers.get_target(register);
        let result = self.res_8(value, bit_n);
        self.registers.set_target(register, result);

        8
    }

    pub fn rl_a(&mut self) -> u16 {
        let register_value = self.registers.a;
        self.registers.a =  self.rl_8(register_value);
        self.registers.f.set_zero(false);

        4
    }

    pub fn rl_hl(&mut self) -> u16 {
        let address = self.registers.get_target_16(&CpuRegister16::HL);
        let value = self.mmu.read_byte(address);
        let data = self.rl_8(value);

        self.mmu.write_byte(address, data);

        16
    }
    
    pub fn rl_r8(&mut self, register: &CpuRegister ) -> u16 {
        let register_value = self.registers.get_target(register);
        let new_value = self.rl_8(register_value);

        self.registers.set_target(register, new_value);
        8
    }

    pub fn rlc_a(&mut self) -> u16 {
        let register_value = self.registers.a;
        self.registers.a = self.rlc_8(register_value);
        self.registers.f.set_zero(false);

        4
    }

    pub fn rlc_hl(&mut self) -> u16 {
        let address = self.registers.get_target_16(&CpuRegister16::HL);
        let value = self.mmu.read_byte(address);
        let data = self.rlc_8(value);

        self.mmu.write_byte(address, data);

        16
    }

    pub fn rlc_r8(&mut self, register: &CpuRegister) -> u16 {
        let register_value = self.registers.get_target(register);
        let new_value = self.rlc_8(register_value);
        self.registers.set_target(register, new_value);
        
        8
    }

    pub fn rr_a(&mut self) -> u16 {
        let register_value = self.registers.a;
        self.registers.a =  self.rr_8(register_value);
        self.registers.f.set_zero(false);

        8
    }

    pub fn rr_hl(&mut self) -> u16 {
        let address = self.registers.get_target_16(&CpuRegister16::HL);
        let value = self.mmu.read_byte(address);
        let data = self.rr_8(value);

        self.mmu.write_byte(address, data);

        16
    }
    
    pub fn rr_r8(&mut self, register: &CpuRegister) -> u16 {
        let register_value = self.registers.get_target(register);
        let new_value = self.rr_8(register_value);

        self.registers.set_target(register, new_value);
        8
    }

    pub fn rrc_a(&mut self) -> u16 {
        let register_value = self.registers.a;
        self.registers.a = self.rrc_8(register_value);
        self.registers.f.set_zero(false);

        8
    }

    pub fn rrc_hl(&mut self) -> u16 {
        let address = self.registers.get_target_16(&CpuRegister16::HL);
        let value = self.mmu.read_byte(address);
        let data = self.rrc_8(value);

        self.mmu.write_byte(address, data);

        16
    }

    pub fn rrc_r8(&mut self, register: &CpuRegister ) -> u16 {
        let register_value = self.registers.get_target(register);
        let new_value = self.rrc_8(register_value);

        self.registers.set_target(register, new_value);
        8
    }

    pub fn set_hl(&mut self, bit_n: u8) -> u16 {
        let address = self.registers.get_target_16(&CpuRegister16::HL);
        let value = self.mmu.read_byte(address);
        let data = self.set_8(value, bit_n);
        
        self.mmu.write_byte(address, data);

        16
    }

    pub fn set_r8(&mut self, register: &CpuRegister, bit_n: u8) -> u16 {
        let value = self.registers.get_target(register);
        let result = self.set_8(value, bit_n);
        self.registers.set_target(register, result);

        8
    }

    pub fn sla_hl(&mut self) -> u16 {
        let address = self.registers.get_target_16(&CpuRegister16::HL);
        let value = self.mmu.read_byte(address);
        let data = self.sla_8(value);

        self.mmu.write_byte(address, data);

        16
    }

    pub fn sla_r8(&mut self, register: &CpuRegister ) -> u16 {
        let register_value = self.registers.get_target(register);
        let new_value = self.sla_8(register_value);

        self.registers.set_target(register, new_value);
        8
    }

    pub fn sra_hl(&mut self) -> u16 {
        let address = self.registers.get_target_16(&CpuRegister16::HL);
        let value = self.mmu.read_byte(address);
        let data = self.sra_8(value);

        self.mmu.write_byte(address, data);

        16
    }

    pub fn sra_r8(&mut self, register: &CpuRegister ) -> u16 {
        let register_value = self.registers.get_target(register);
        let new_value = self.sra_8(register_value);

        self.registers.set_target(register, new_value);
        8
    }

    pub fn srl_hl(&mut self) -> u16 {
        let address = self.registers.get_target_16(&CpuRegister16::HL);
        let value = self.mmu.read_byte(address);
        let data = self.srl_8(value);

        self.mmu.write_byte(address, data);

        16
    }

    pub fn srl_r8(&mut self, register: &CpuRegister ) -> u16 {
        let register_value = self.registers.get_target(register);
        let new_value = self.srl_8(register_value);

        self.registers.set_target(register, new_value);
        8
    }

    pub fn swap_hl(&mut self) -> u16 {
        let address = self.registers.get_target_16(&CpuRegister16::HL);
        let value = self.mmu.read_byte(address);
        let new_value = self.swap(value);
        self.mmu.write_byte(address, new_value);

        16
    }

    pub fn swap_r8(&mut self, register: &CpuRegister) -> u16 {
        let register_value = self.registers.get_target(register);
        let new_value = self.swap(register_value);

        self.registers.set_target(register, new_value);
        8
    }
}