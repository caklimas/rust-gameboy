pub mod flags_register;
pub mod index_registers;
pub mod opcodes;
pub mod registers;

#[cfg(test)]
mod tests;

use serde::{Serialize, Deserialize};
use super::mmu::MemoryManagementUnit;
use opcodes::opcode::{Clock, Opcode};

#[derive(Serialize, Deserialize, Default)]
pub struct Cpu {
    index_registers: index_registers::IndexRegisters,
    interrupt_page_address: u8,
    memory_refresh: u8,
    mmu: MemoryManagementUnit,
    opcode: usize,
    program_counter: u16,
    registers: registers::Registers,
    stack_pointer: u16,
    system_clock: u16
}

impl Cpu {
    pub fn clock(&mut self) {
        let mut clock: Option<&Clock> = None;
        if let Some(ref o) = opcodes::opcode_table::OPCODE_TABLE[self.opcode] {
            match o {
                Opcode::Adc(f, a, c) => {
                    f(self, a);
                    clock = Some(c);
                },
                Opcode::AdcD8(c) => {
                    self.adc_d8();
                    clock = Some(c);
                },
                Opcode::AdcHL(c) => {
                    self.adc_hl();
                    clock = Some(c);
                },
                Opcode::Add(f, a, c) => {
                    f(self, a);
                    clock = Some(c);
                },
                Opcode::AddD8(c) => {
                    self.add_d8();
                    clock = Some(c);
                },
                Opcode::AddHL(c) => {
                    self.add_hl();
                    clock = Some(c);
                },
                Opcode::And(f, a, c) => {
                    f(self, a);
                    clock = Some(c);
                },
                Opcode::AndD8(c) => {
                    self.and_d8();
                    clock = Some(c);
                },
                Opcode::AndHL(c) => {
                    self.and_hl();
                    clock = Some(c);
                },
                Opcode::Cp(f, a, c) => {
                    f(self, a);
                    clock = Some(c);
                },
                Opcode::CpD8(c) => {
                    self.cp_d8();
                    clock = Some(c);
                },
                Opcode::CpHL(c) => {
                    self.cp_hl();
                    clock = Some(c);
                },
                Opcode::Or(f, a, c) => {
                    f(self, a);
                    clock = Some(c);
                },
                Opcode::OrD8(c) => {
                    self.or_d8();
                    clock = Some(c);
                },
                Opcode::OrHL(c) => {
                    self.or_hl();
                    clock = Some(c);
                },
                Opcode::Sbc(f, a, c) => {
                    f(self, a);
                    clock = Some(c);
                },
                Opcode::SbcD8(c) => {
                    self.sbc_d8();
                    clock = Some(c);
                },
                Opcode::SbcHL(c) => {
                    self.sbc_hl();
                    clock = Some(c);
                },
                Opcode::Sub(f, a, c) => {
                    f(self, a);
                    clock = Some(c);
                },
                Opcode::SubD8(c) => {
                    self.sub_d8();
                    clock = Some(c);
                },
                Opcode::SubHL(c) => {
                    self.sub_hl();
                    clock = Some(c);
                },
                Opcode::XOr(f, a, c) => {
                    f(self, a);
                    clock = Some(c);
                },
                Opcode::XOrD8(c) => {
                    self.xor_d8();
                    clock = Some(c)
                },
                Opcode::XOrHL(c) => {
                    self.xor_hl();
                    clock = Some(c);
                }
            }
        }

        if let Some(ref c) = clock {
            self.program_counter += c.0;
            self.system_clock += c.1;
        }
    }
}