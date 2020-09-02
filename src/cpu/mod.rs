pub mod flags_register;
pub mod index_registers;
pub mod opcodes;
pub mod registers;

#[cfg(test)]
mod tests;

use serde::{Serialize, Deserialize};
use super::mmu::MemoryManagementUnit;
use opcodes::{
    ClockCycle,
    opcode::Opcode
};

#[derive(Serialize, Deserialize, Default)]
pub struct Cpu {
    index_registers: index_registers::IndexRegisters,
    interrupt_master_enable: bool,
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
        let mut clock: Option<ClockCycle> = None;
        if let Some(ref o) = opcodes::opcode_table::OPCODE_TABLE[self.opcode] {
            match o {
                Opcode::Adc(r) => {
                    let c = self.adc_a(r);
                    clock = Some(c);
                },
                Opcode::AdcD8 => {
                    let c = self.adc_d8();
                    clock = Some(c);
                },
                Opcode::AdcHl => {
                    let c = self.adc_hl();
                    clock = Some(c);
                },
                Opcode::Add(r) => {
                    let c = self.add_a(r);
                    clock = Some(c);
                },
                Opcode::AddD8 => {
                    let c = self.add_d8();
                    clock = Some(c);
                },
                Opcode::AddHl => {
                    let c = self.add_hl();
                    clock = Some(c);
                },
                Opcode::And(r) => {
                    let c = self.and_a(r);
                    clock = Some(c);
                },
                Opcode::AndD8 => {
                    let c = self.and_d8();
                    clock = Some(c);
                },
                Opcode::AndHl => {
                    let c = self.and_hl();
                    clock = Some(c);
                },
                Opcode::Call => {
                    let c = self.call();
                    clock = Some(c);
                },
                Opcode::CallCc(condition) => {
                    let c = self.call_cc(condition);
                    clock = Some(c);
                },
                Opcode::Ccf => {
                    let c = self.ccf();
                    clock = Some(c);
                },
                Opcode::Cp(r) => {
                    let c = self.cp_a(r);
                    clock = Some(c);
                },
                Opcode::CpD8 => {
                    let c = self.cp_d8();
                    clock = Some(c);
                },
                Opcode::CpHl => {
                    let c = self.cp_hl();
                    clock = Some(c);
                },
                Opcode::Cpl => {
                    let c = self.cpl();
                    clock = Some(c);
                },
                Opcode::DecHl => {
                    let c = self.dec_hl();
                    clock = Some(c);
                },
                Opcode::DecR(r) => {
                    let c = self.dec_r(r);
                    clock = Some(c);
                },
                Opcode::DecSp => {
                    let c = self.dec_sp();
                    clock = Some(c);
                },
                Opcode::Dec16(r) => {
                    let c = self.dec_16(r);
                    clock = Some(c);
                },
                Opcode::Ei => {
                    let c = self.ei();
                    clock = Some(c);
                },
                Opcode::IncHl => {
                    let c = self.inc_hl();
                    clock = Some(c);
                },
                Opcode::IncR(r) => {
                    let c = self.inc_r(r);
                    clock = Some(c);
                },
                Opcode::IncSp => {
                    let c = self.inc_sp();
                    clock = Some(c);
                },
                Opcode::Inc16(r) => {
                    let c = self.inc_16(r);
                    clock = Some(c);
                },
                Opcode::Jp => {
                    let c = self.jp();
                    clock = Some(c);
                },
                Opcode::JpCc(condition) => {
                    let c = self.jp_cc(condition);
                    clock = Some(c);
                },
                Opcode::JpHl => {
                    let c = self.jp_hl();
                    clock = Some(c);
                },
                Opcode::Jr => {
                    let c = self.jr();
                    clock = Some(c);
                },
                Opcode::JrCc(condition) => {
                    let c = self.jr_cc(condition);
                    clock = Some(c);
                },
                Opcode::Ld(dest, src) => {
                    let c = self.ld(dest, src);
                    clock = Some(c);
                },
                Opcode::LdA8A => {
                    let c = self.ld_a8_a();
                    clock = Some(c);
                },
                Opcode::LdAA8 => {
                    let c = self.ld_a_a8();
                    clock = Some(c);
                },
                Opcode::LdA16A => {
                    let c = self.ld_a16_a();
                    clock = Some(c);
                },
                Opcode::LdAA16 => {
                    let c = self.ld_a_a16();
                    clock = Some(c);
                },
                Opcode::LdAC => {
                    let c = self.ld_a_c();
                    clock = Some(c);
                },
                Opcode::LdCA => {
                    let c = self.ld_c_a();
                    clock = Some(c);
                },
                Opcode::LdD8(r) => {
                    let c = self.ld_d8(r);
                    clock = Some(c);
                },
                Opcode::LdHlD8 => {
                    let c = self.ld_hl_d8();
                    clock = Some(c);
                },
                Opcode::LdHlA(increment) => {
                    let c = self.ld_hl_a(increment);
                    clock = Some(c);

                },
                Opcode::LdAHl(increment) => {
                    let c = self.ld_a_hl(increment);
                    clock = Some(c);
                },
                Opcode::LdA16Sp => {
                    let c = self.ld_a16_sp();
                    clock = Some(c);
                },
                Opcode::Ld16R(r16, r) => {
                    let c = self.ld_16_r(r16, r);
                    clock = Some(c);
                },
                Opcode::LdR16(r, r16) => {
                    let c = self.ld_r_16(r, r16);
                    clock = Some(c);
                },
                Opcode::LdR16D16(r) => {
                    let c = self.ld_r16_d16(r);
                    clock = Some(c);
                },
                Opcode::LdSpD16 => {
                    let c = self.ld_sp_d16();
                    clock = Some(c);
                },
                Opcode::LdSpE8 => {
                    let c = self.ld_sp_e8();
                    clock = Some(c);
                },
                Opcode::LdSpHl => {
                    let c = self.ld_sp_hl();
                    clock = Some(c)
                },
                Opcode::Or(r) => {
                    let c = self.or_a(r);
                    clock = Some(c);
                },
                Opcode::OrD8 => {
                    let c = self.or_d8();
                    clock = Some(c);
                },
                Opcode::OrHl => {
                    let c = self.or_hl();
                    clock = Some(c);
                },
                Opcode::Pop(r) => {
                    let c = self.pop(r);
                    clock = Some(c);
                },
                Opcode::Push(r) => {
                    let c = self.push(r);
                    clock = Some(c);
                },
                Opcode::Ret => {
                    let c = self.ret();
                    clock = Some(c);
                },
                Opcode::RetCc(condition) => {
                    let c = self.ret_cc(condition);
                    clock = Some(c);
                },
                Opcode::RetI => {
                    let c = self.ret_i();
                    clock = Some(c);
                },
                Opcode::Rst(v) => {
                    let c = self.rst(v.clone());
                    clock = Some(c);
                },
                Opcode::Sbc(r) => {
                    let c = self.sbc_a(r);
                    clock = Some(c);
                },
                Opcode::SbcD8 => {
                    let c = self.sbc_d8();
                    clock = Some(c);
                },
                Opcode::SbcHl => {
                    let c = self.sbc_hl();
                    clock = Some(c);
                },
                Opcode::Scf => {
                    let c = self.scf();
                    clock = Some(c);
                },
                Opcode::Sub(r) => {
                    let c = self.sub_a(r);
                    clock = Some(c);
                },
                Opcode::SubD8 => {
                    let c = self.sub_d8();
                    clock = Some(c);
                },
                Opcode::SubHl => {
                    let c = self.sub_hl();
                    clock = Some(c);
                },
                Opcode::XOr(r) => {
                    let c = self.xor_a(r);
                    clock = Some(c);
                },
                Opcode::XOrD8 => {
                    let c = self.xor_d8();
                    clock = Some(c)
                },
                Opcode::XOrHl => {
                    let c = self.xor_hl();
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