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
                Opcode::Adc(r, c) => {
                    self.adc_a(r);
                    clock = Some(c);
                },
                Opcode::AdcD8(c) => {
                    self.adc_d8();
                    clock = Some(c);
                },
                Opcode::AdcHl(c) => {
                    self.adc_hl();
                    clock = Some(c);
                },
                Opcode::Add(r, c) => {
                    self.add_a(r);
                    clock = Some(c);
                },
                Opcode::AddD8(c) => {
                    self.add_d8();
                    clock = Some(c);
                },
                Opcode::AddHl(c) => {
                    self.add_hl();
                    clock = Some(c);
                },
                Opcode::And(r, c) => {
                    self.and_a(r);
                    clock = Some(c);
                },
                Opcode::AndD8(c) => {
                    self.and_d8();
                    clock = Some(c);
                },
                Opcode::AndHl(c) => {
                    self.and_hl();
                    clock = Some(c);
                },
                Opcode::Ccf(c) => {
                    self.ccf();
                    clock = Some(c);
                },
                Opcode::Cp(r, c) => {
                    self.cp_a(r);
                    clock = Some(c);
                },
                Opcode::CpD8(c) => {
                    self.cp_d8();
                    clock = Some(c);
                },
                Opcode::CpHl(c) => {
                    self.cp_hl();
                    clock = Some(c);
                },
                Opcode::Cpl(c) => {
                    self.cpl();
                    clock = Some(c);
                },
                Opcode::DecHl(c) => {
                    self.dec_hl();
                    clock = Some(c);
                },
                Opcode::DecR(r, c) => {
                    self.dec_r(r);
                    clock = Some(c);
                },
                Opcode::DecSp(c) => {
                    self.dec_sp();
                    clock = Some(c);
                },
                Opcode::Dec16(r, c) => {
                    self.dec_16(r);
                    clock = Some(c);
                },
                Opcode::IncHl(c) => {
                    self.inc_hl();
                    clock = Some(c);
                },
                Opcode::IncR(r, c) => {
                    self.inc_r(r);
                    clock = Some(c);
                },
                Opcode::IncSp(c) => {
                    self.inc_sp();
                    clock = Some(c);
                },
                Opcode::Inc16(r, c) => {
                    self.inc_16(r);
                    clock = Some(c);
                },
                Opcode::Ld(dest, src, c) => {
                    self.ld(dest, src);
                    clock = Some(c);
                },
                Opcode::LdD8(r, c) => {
                    self.ld_d8(r);
                    clock = Some(c);
                },
                Opcode::LdHlD8(c) => {
                    self.ld_hl_d8();
                    clock = Some(c);
                },
                Opcode::LdHlA(increment, c) => {
                    self.ld_hl_a(increment);
                    clock = Some(c);

                },
                Opcode::LdAHl(increment, c) => {
                    self.ld_a_hl(increment);
                    clock = Some(c);
                },
                Opcode::LdA16Sp(c) => {
                    self.ld_a16_sp();
                    clock = Some(c);
                },
                Opcode::Ld16R(r16, r, c) => {
                    self.ld_16_r(r16, r);
                    clock = Some(c);
                },
                Opcode::LdR16(r, r16, c) => {
                    self.ld_r_16(r, r16);
                    clock = Some(c);
                },
                Opcode::LdR16D16(r, c) => {
                    self.ld_r16_d16(r);
                    clock = Some(c);
                },
                Opcode::LdSpD16(c) => {
                    self.ld_sp_d16();
                    clock = Some(c);
                },
                Opcode::LdSpE8(c) => {
                    self.ld_sp_e8();
                    clock = Some(c);
                },
                Opcode::LdSpHl(c) => {
                    self.ld_sp_hl();
                    clock = Some(c)
                },
                Opcode::Or(r, c) => {
                    self.or_a(r);
                    clock = Some(c);
                },
                Opcode::OrD8(c) => {
                    self.or_d8();
                    clock = Some(c);
                },
                Opcode::OrHl(c) => {
                    self.or_hl();
                    clock = Some(c);
                },
                Opcode::Pop(r, c) => {
                    self.pop(r);
                    clock = Some(c);
                },
                Opcode::Push(r, c) => {
                    self.push(r);
                    clock = Some(c);
                },
                Opcode::Sbc(r, c) => {
                    self.sbc_a(r);
                    clock = Some(c);
                },
                Opcode::SbcD8(c) => {
                    self.sbc_d8();
                    clock = Some(c);
                },
                Opcode::SbcHl(c) => {
                    self.sbc_hl();
                    clock = Some(c);
                },
                Opcode::Scf(c) => {
                    self.scf();
                    clock = Some(c);
                },
                Opcode::Sub(r, c) => {
                    self.sub_a(r);
                    clock = Some(c);
                },
                Opcode::SubD8(c) => {
                    self.sub_d8();
                    clock = Some(c);
                },
                Opcode::SubHl(c) => {
                    self.sub_hl();
                    clock = Some(c);
                },
                Opcode::XOr(r, c) => {
                    self.xor_a(r);
                    clock = Some(c);
                },
                Opcode::XOrD8(c) => {
                    self.xor_d8();
                    clock = Some(c)
                },
                Opcode::XOrHl(c) => {
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

    pub fn read_next_byte(&self) -> u8 {
        self.mmu.read_byte(self.program_counter + 1)
    }

    pub fn read_next_word(&self) -> u16 {
        self.mmu.read_word(self.program_counter + 1)
    }
}