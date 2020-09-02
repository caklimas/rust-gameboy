use super::super::super::addresses::ld_opcode::LD_ADDRESS_LOWER;
use super::opcode::{CpuRegister, CpuRegister16};
use super::ClockCycle;

impl super::super::Cpu {
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
}