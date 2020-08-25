use serde::{Serialize, Deserialize};
use super::flags_register::FlagsRegister;
use super::opcodes::opcode::ArithmeticRegister;

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Registers {
    pub a: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub f: FlagsRegister,
    pub h: u8,
    pub l: u8
}

impl Registers {
    pub fn get_bc(&self) -> u16 {
        let high = (self.b as u16) << 8;
        let low = self.c as u16;
        high | low
    }

    pub fn set_bc(&mut self, value: u16) {
        self.b = ((value & 0xFF00) >> 8) as u8;
        self.c = (value & 0xFF) as u8;
    }

    pub fn get_de(&self) -> u16 {
        let high = (self.d as u16) << 8;
        let low = self.e as u16;
        high | low
    }

    pub fn set_de(&mut self, value: u16) {
        self.d = ((value & 0xFF00) >> 8) as u8;
        self.e = (value & 0xFF) as u8;
    }

    pub fn get_hl(&self) -> u16 {
        let high = (self.h as u16) << 8;
        let low = self.l as u16;
        high | low
    }

    pub fn set_hl(&mut self, value: u16) {
        self.h = ((value & 0xFF00) >> 8) as u8;
        self.l = (value & 0xFF) as u8;
    }

    pub fn get_target(&mut self, register: &ArithmeticRegister) -> u8 {
        match register {
            ArithmeticRegister::A => self.a,
            ArithmeticRegister::B => self.b,
            ArithmeticRegister::C => self.c,
            ArithmeticRegister::D => self.d,
            ArithmeticRegister::E => self.e,
            ArithmeticRegister::H => self.h,
            ArithmeticRegister::L => self.l
        }
    }
}