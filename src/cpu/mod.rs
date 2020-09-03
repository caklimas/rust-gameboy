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
    cb_opcode::CbOpcode,
    cb_opcode_table::CB_OPCODE_TABLE,
    opcode::Opcode,
    opcode_table::OPCODE_TABLE
};

#[derive(Serialize, Deserialize, Default)]
pub struct Cpu {
    cb_opcode: bool,
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
        let clock = if self.cb_opcode {
            Some(self.execute_cb_opcode())
        } else {
            self.execute_opcode()
        };

        if let Some(ref c) = clock {
            self.program_counter += c.0;
            self.system_clock += c.1;
        } else {
            self.program_counter += 1;
        }
    }

    pub fn read_next_byte(&self) -> u8 {
        self.mmu.read_byte(self.program_counter + 1)
    }

    pub fn read_next_word(&self) -> u16 {
        self.mmu.read_word(self.program_counter + 1)
    }

    fn execute_opcode(&mut self) -> Option<ClockCycle> {
        if let Some(ref o) = OPCODE_TABLE[self.opcode] {
            match o {
                Opcode::Adc(r) => {
                    let c = self.adc_a(r);
                    Some(c)
                },
                Opcode::AdcD8 => {
                    let c = self.adc_d8();
                    Some(c)
                },
                Opcode::AdcHl => {
                    let c = self.adc_hl();
                    Some(c)
                },
                Opcode::Add(r) => {
                    let c = self.add_a(r);
                    Some(c)
                },
                Opcode::AddD8 => {
                    let c = self.add_d8();
                    Some(c)
                },
                Opcode::AddAHl => {
                    let c = self.add_a_hl();
                    Some(c)
                },
                Opcode::AddHl16(r) => {
                    let c = self.add_hl_16(r);
                    Some(c)
                },
                Opcode::AddHl16Sp => {
                    let c = self.add_hl_16_sp();
                    Some(c)
                },
                Opcode::AddSpE8 => {
                    let c = self.add_sp_e8();
                    Some(c)
                },
                Opcode::And(r) => {
                    let c = self.and_a(r);
                    Some(c)
                },
                Opcode::AndD8 => {
                    let c = self.and_d8();
                    Some(c)
                },
                Opcode::AndHl => {
                    let c = self.and_hl();
                    Some(c)
                },
                Opcode::Call => {
                    let c = self.call();
                    Some(c)
                },
                Opcode::CallCc(condition) => {
                    let c = self.call_cc(condition);
                    Some(c)
                },
                Opcode::Cb => {
                    self.cb_opcode = true;
                    Some((1, 4))
                },
                Opcode::Ccf => {
                    let c = self.ccf();
                    Some(c)
                },
                Opcode::Cp(r) => {
                    let c = self.cp_a(r);
                    Some(c)
                },
                Opcode::CpD8 => {
                    let c = self.cp_d8();
                    Some(c)
                },
                Opcode::CpHl => {
                    let c = self.cp_hl();
                    Some(c)
                },
                Opcode::Cpl => {
                    let c = self.cpl();
                    Some(c)
                },
                Opcode::DecHl => {
                    let c = self.dec_hl();
                    Some(c)
                },
                Opcode::DecR(r) => {
                    let c = self.dec_r(r);
                    Some(c)
                },
                Opcode::DecSp => {
                    let c = self.dec_sp();
                    Some(c)
                },
                Opcode::Dec16(r) => {
                    let c = self.dec_16(r);
                    Some(c)
                },
                Opcode::Ei => {
                    let c = self.ei();
                    Some(c)
                },
                Opcode::IncHl => {
                    let c = self.inc_hl();
                    Some(c)
                },
                Opcode::IncR(r) => {
                    let c = self.inc_r(r);
                    Some(c)
                },
                Opcode::IncSp => {
                    let c = self.inc_sp();
                    Some(c)
                },
                Opcode::Inc16(r) => {
                    let c = self.inc_16(r);
                    Some(c)
                },
                Opcode::Jp => {
                    let c = self.jp();
                    Some(c)
                },
                Opcode::JpCc(condition) => {
                    let c = self.jp_cc(condition);
                    Some(c)
                },
                Opcode::JpHl => {
                    let c = self.jp_hl();
                    Some(c)
                },
                Opcode::Jr => {
                    let c = self.jr();
                    Some(c)
                },
                Opcode::JrCc(condition) => {
                    let c = self.jr_cc(condition);
                    Some(c)
                },
                Opcode::Ld(dest, src) => {
                    let c = self.ld(dest, src);
                    Some(c)
                },
                Opcode::LdA8A => {
                    let c = self.ld_a8_a();
                    Some(c)
                },
                Opcode::LdAA8 => {
                    let c = self.ld_a_a8();
                    Some(c)
                },
                Opcode::LdA16A => {
                    let c = self.ld_a16_a();
                    Some(c)
                },
                Opcode::LdAA16 => {
                    let c = self.ld_a_a16();
                    Some(c)
                },
                Opcode::LdAC => {
                    let c = self.ld_a_c();
                    Some(c)
                },
                Opcode::LdCA => {
                    let c = self.ld_c_a();
                    Some(c)
                },
                Opcode::LdD8(r) => {
                    let c = self.ld_d8(r);
                    Some(c)
                },
                Opcode::LdHlD8 => {
                    let c = self.ld_hl_d8();
                    Some(c)
                },
                Opcode::LdHlA(increment) => {
                    let c = self.ld_hl_a(increment);
                    Some(c)

                },
                Opcode::LdAHl(increment) => {
                    let c = self.ld_a_hl(increment);
                    Some(c)
                },
                Opcode::LdA16Sp => {
                    let c = self.ld_a16_sp();
                    Some(c)
                },
                Opcode::Ld16R(r16, r) => {
                    let c = self.ld_16_r(r16, r);
                    Some(c)
                },
                Opcode::LdR16(r, r16) => {
                    let c = self.ld_r_16(r, r16);
                    Some(c)
                },
                Opcode::LdR16D16(r) => {
                    let c = self.ld_r16_d16(r);
                    Some(c)
                },
                Opcode::LdSpD16 => {
                    let c = self.ld_sp_d16();
                    Some(c)
                },
                Opcode::LdSpE8 => {
                    let c = self.ld_sp_e8();
                    Some(c)
                },
                Opcode::LdSpHl => {
                    let c = self.ld_sp_hl();
                    Some(c)
                },
                Opcode::Or(r) => {
                    let c = self.or_a(r);
                    Some(c)
                },
                Opcode::OrD8 => {
                    let c = self.or_d8();
                    Some(c)
                },
                Opcode::OrHl => {
                    let c = self.or_hl();
                    Some(c)
                },
                Opcode::Pop(r) => {
                    let c = self.pop(r);
                    Some(c)
                },
                Opcode::Push(r) => {
                    let c = self.push(r);
                    Some(c)
                },
                Opcode::Ret => {
                    let c = self.ret();
                    Some(c)
                },
                Opcode::RetCc(condition) => {
                    let c = self.ret_cc(condition);
                    Some(c)
                },
                Opcode::RetI => {
                    let c = self.ret_i();
                    Some(c)
                },
                Opcode::Rst(v) => {
                    let c = self.rst(v.clone());
                    Some(c)
                },
                Opcode::Sbc(r) => {
                    let c = self.sbc_a(r);
                    Some(c)
                },
                Opcode::SbcD8 => {
                    let c = self.sbc_d8();
                    Some(c)
                },
                Opcode::SbcHl => {
                    let c = self.sbc_hl();
                    Some(c)
                },
                Opcode::Scf => {
                    let c = self.scf();
                    Some(c)
                },
                Opcode::Sub(r) => {
                    let c = self.sub_a(r);
                    Some(c)
                },
                Opcode::SubD8 => {
                    let c = self.sub_d8();
                    Some(c)
                },
                Opcode::SubHl => {
                    let c = self.sub_hl();
                    Some(c)
                },
                Opcode::XOr(r) => {
                    let c = self.xor_a(r);
                    Some(c)
                },
                Opcode::XOrD8 => {
                    let c = self.xor_d8();
                    Some(c)
                },
                Opcode::XOrHl => {
                    let c = self.xor_hl();
                    Some(c)
                }
            }
        } else {
            None
        }
    }

    fn execute_cb_opcode(&mut self) -> ClockCycle {
        let o = &CB_OPCODE_TABLE[self.opcode];
        match o {
            CbOpcode::RlHl => {
                let c = self.rl_hl();
                c
            },
            CbOpcode::RlR8(r) => {
                let c = self.rl_r8(r);
                c
            },
            CbOpcode::RlcHl => {
                let c = self.rlc_hl();
                c
            },
            CbOpcode::RlcR8(r) => {
                let c = self.rlc_r8(r);
                c
            },
            CbOpcode::RrHl => {
                let c = self.rr_hl();
                c
            },
            CbOpcode::RrR8(r) => {
                let c = self.rr_r8(r);
                c
            },
            CbOpcode::RrcHl => {
                let c = self.rrc_hl();
                c
            },
            CbOpcode::RrcR8(r) => {
                let c = self.rrc_r8(r);
                c
            },
            CbOpcode::SlaHl => {
                let c = self.sla_hl();
                c
            },
            CbOpcode::SlaR8(r) => {
                let c = self.sla_r8(r);
                c
            },
            CbOpcode::SraHl => {
                let c = self.sra_hl();
                c
            },
            CbOpcode::SraR8(r) => {
                let c = self.sra_r8(r);
                c
            },
            CbOpcode::Unknown => {
                panic!("Unrecognized cb opcode");
            }
        }
    }
}