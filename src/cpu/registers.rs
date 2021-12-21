use super::flags_register::FlagsRegister;
use super::opcodes::opcode::{CpuRegister, CpuRegister16};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Registers {
    pub a: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub f: FlagsRegister,
    pub h: u8,
    pub l: u8,
}

impl Registers {
    pub fn get_target(&self, register: &CpuRegister) -> u8 {
        match register {
            CpuRegister::A => self.a,
            CpuRegister::B => self.b,
            CpuRegister::C => self.c,
            CpuRegister::D => self.d,
            CpuRegister::E => self.e,
            CpuRegister::H => self.h,
            CpuRegister::L => self.l,
        }
    }

    pub fn get_target_16(&self, register: &CpuRegister16) -> u16 {
        match register {
            CpuRegister16::AF => self.get_af(),
            CpuRegister16::BC => self.get_bc(),
            CpuRegister16::DE => self.get_de(),
            CpuRegister16::HL => self.get_hl(),
        }
    }

    pub fn set_target(&mut self, register: &CpuRegister, value: u8) {
        match register {
            CpuRegister::A => self.a = value,
            CpuRegister::B => self.b = value,
            CpuRegister::C => self.c = value,
            CpuRegister::D => self.d = value,
            CpuRegister::E => self.e = value,
            CpuRegister::H => self.h = value,
            CpuRegister::L => self.l = value,
        }
    }

    pub fn set_target_16(&mut self, register: &CpuRegister16, value: u16) {
        match register {
            CpuRegister16::AF => self.set_af(value),
            CpuRegister16::BC => self.set_bc(value),
            CpuRegister16::DE => self.set_de(value),
            CpuRegister16::HL => self.set_hl(value),
        }
    }

    fn get_af(&self) -> u16 {
        let high = (self.a as u16) << 8;
        let low = self.f.get() as u16;
        high | low
    }

    fn set_af(&mut self, value: u16) {
        self.a = ((value & 0xFF00) >> 8) as u8;
        self.f.set((value & 0xF0) as u8);
    }

    fn get_bc(&self) -> u16 {
        let high = (self.b as u16) << 8;
        let low = self.c as u16;
        high | low
    }

    fn set_bc(&mut self, value: u16) {
        self.b = ((value & 0xFF00) >> 8) as u8;
        self.c = (value & 0xFF) as u8;
    }

    fn get_de(&self) -> u16 {
        let high = (self.d as u16) << 8;
        let low = self.e as u16;
        high | low
    }

    fn set_de(&mut self, value: u16) {
        self.d = ((value & 0xFF00) >> 8) as u8;
        self.e = (value & 0xFF) as u8;
    }

    fn get_hl(&self) -> u16 {
        let high = (self.h as u16) << 8;
        let low = self.l as u16;
        high | low
    }

    fn set_hl(&mut self, value: u16) {
        self.h = ((value & 0xFF00) >> 8) as u8;
        self.l = (value & 0xFF) as u8;
    }
}
