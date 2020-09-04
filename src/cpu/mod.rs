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
                    Some(self.adc_a(r))
                },
                Opcode::AdcD8 => {
                    Some(self.adc_d8())
                },
                Opcode::AdcHl => {
                    Some(self.adc_hl())
                },
                Opcode::Add(r) => {
                    Some(self.add_a(r))
                },
                Opcode::AddD8 => {
                    Some(self.add_d8())
                },
                Opcode::AddAHl => {
                    Some(self.add_a_hl())
                },
                Opcode::AddHl16(r) => {
                    Some(self.add_hl_16(r))
                },
                Opcode::AddHl16Sp => {
                    Some(self.add_hl_16_sp())
                },
                Opcode::AddSpE8 => {
                    Some(self.add_sp_e8())
                },
                Opcode::And(r) => {
                    Some(self.and_a(r))
                },
                Opcode::AndD8 => {
                    Some(self.and_d8())
                },
                Opcode::AndHl => {
                    Some(self.and_hl())
                },
                Opcode::Call => {
                    Some(self.call())
                },
                Opcode::CallCc(condition) => {
                    Some(self.call_cc(condition))
                },
                Opcode::Cb => {
                    self.cb_opcode = true;
                    Some((1, 4))
                },
                Opcode::Ccf => {
                    Some(self.ccf())
                },
                Opcode::Cp(r) => {
                    Some(self.cp_a(r))
                },
                Opcode::CpD8 => {
                    Some(self.cp_d8())
                },
                Opcode::CpHl => {
                    Some(self.cp_hl())
                },
                Opcode::Cpl => {
                    Some(self.cpl())
                },
                Opcode::DecHl => {
                    Some(self.dec_hl())
                },
                Opcode::DecR(r) => {
                    Some(self.dec_r(r))
                },
                Opcode::DecSp => {
                    Some(self.dec_sp())
                },
                Opcode::Dec16(r) => {
                    Some(self.dec_16(r))
                },
                Opcode::Ei => {
                    Some(self.ei())
                },
                Opcode::IncHl => {
                    Some(self.inc_hl())
                },
                Opcode::IncR(r) => {
                    Some(self.inc_r(r))
                },
                Opcode::IncSp => {
                    Some(self.inc_sp())
                },
                Opcode::Inc16(r) => {
                    Some(self.inc_16(r))
                },
                Opcode::Jp => {
                    Some(self.jp())
                },
                Opcode::JpCc(condition) => {
                    Some(self.jp_cc(condition))
                },
                Opcode::JpHl => {
                    Some(self.jp_hl())
                },
                Opcode::Jr => {
                    Some(self.jr())
                },
                Opcode::JrCc(condition) => {
                    Some(self.jr_cc(condition))
                },
                Opcode::Ld(dest, src) => {
                    Some(self.ld(dest, src))
                },
                Opcode::LdA8A => {
                    Some(self.ld_a8_a())
                },
                Opcode::LdAA8 => {
                    Some(self.ld_a_a8())
                },
                Opcode::LdA16A => {
                    Some(self.ld_a16_a())
                },
                Opcode::LdAA16 => {
                    Some(self.ld_a_a16())
                },
                Opcode::LdAC => {
                    Some(self.ld_a_c())
                },
                Opcode::LdCA => {
                    Some(self.ld_c_a())
                },
                Opcode::LdD8(r) => {
                    Some(self.ld_d8(r))
                },
                Opcode::LdHlD8 => {
                    Some(self.ld_hl_d8())
                },
                Opcode::LdHlA(increment) => {
                    Some(self.ld_hl_a(increment))

                },
                Opcode::LdAHl(increment) => {
                    Some(self.ld_a_hl(increment))
                },
                Opcode::LdA16Sp => {
                    Some(self.ld_a16_sp())
                },
                Opcode::Ld16R(r16, r) => {
                    Some(self.ld_16_r(r16, r))
                },
                Opcode::LdR16(r, r16) => {
                    Some(self.ld_r_16(r, r16))
                },
                Opcode::LdR16D16(r) => {
                    Some(self.ld_r16_d16(r))
                },
                Opcode::LdSpD16 => {
                    Some(self.ld_sp_d16())
                },
                Opcode::LdSpE8 => {
                    Some(self.ld_sp_e8())
                },
                Opcode::LdSpHl => {
                    Some(self.ld_sp_hl())
                },
                Opcode::Or(r) => {
                    Some(self.or_a(r))
                },
                Opcode::OrD8 => {
                    Some(self.or_d8())
                },
                Opcode::OrHl => {
                    Some(self.or_hl())
                },
                Opcode::Pop(r) => {
                    Some(self.pop(r))
                },
                Opcode::Push(r) => {
                    Some(self.push(r))
                },
                Opcode::Ret => {
                    Some(self.ret())
                },
                Opcode::RetCc(condition) => {
                    Some(self.ret_cc(condition))
                },
                Opcode::RetI => {
                    Some(self.ret_i())
                },
                Opcode::Rst(v) => {
                    Some(self.rst(v.clone()))
                },
                Opcode::Sbc(r) => {
                    Some(self.sbc_a(r))
                },
                Opcode::SbcD8 => {
                    Some(self.sbc_d8())
                },
                Opcode::SbcHl => {
                    Some(self.sbc_hl())
                },
                Opcode::Scf => {
                    Some(self.scf())
                },
                Opcode::Sub(r) => {
                    Some(self.sub_a(r))
                },
                Opcode::SubD8 => {
                    Some(self.sub_d8())
                },
                Opcode::SubHl => {
                    Some(self.sub_hl())
                },
                Opcode::XOr(r) => {
                    Some(self.xor_a(r))
                },
                Opcode::XOrD8 => {
                    Some(self.xor_d8())
                },
                Opcode::XOrHl => {
                    Some(self.xor_hl())
                }
            }
        } else {
            None
        }
    }

    fn execute_cb_opcode(&mut self) -> ClockCycle {
        let o = &CB_OPCODE_TABLE[self.opcode];
        match o {
            CbOpcode::BitNSetHl(b) => {
                self.bit_n_set_hl(b.clone())
            },
            CbOpcode::BitNSetR8(r, b) => {
                self.bit_n_set_r8(r, b.clone())
            },
            CbOpcode::ResHl(b) => {
                self.res_hl(b.clone())
            },
            CbOpcode::ResR8(r, b) => {
                self.res_r8(r, b.clone())
            },
            CbOpcode::RlHl => {
                self.rl_hl()
            },
            CbOpcode::RlR8(r) => {
                self.rl_r8(r)
            },
            CbOpcode::RlcHl => {
                self.rlc_hl()
            },
            CbOpcode::RlcR8(r) => {
                self.rlc_r8(r)
            },
            CbOpcode::RrHl => {
                self.rr_hl()
            },
            CbOpcode::RrR8(r) => {
                self.rr_r8(r)
            },
            CbOpcode::RrcHl => {
                self.rrc_hl()
            },
            CbOpcode::RrcR8(r) => {
                self.rrc_r8(r)
            },
            CbOpcode::SetHl(b) => {
                self.set_hl(b.clone())
            },
            CbOpcode::SetR8(r, b) => {
                self.set_r8(r, b.clone())
            },
            CbOpcode::SlaHl => {
                self.sla_hl()
            },
            CbOpcode::SlaR8(r) => {
                self.sla_r8(r)
            },
            CbOpcode::SraHl => {
                self.sra_hl()
            },
            CbOpcode::SraR8(r) => {
                self.sra_r8(r)
            },
            CbOpcode::SrlHl => {
                self.srl_hl()
            },
            CbOpcode::SrlR8(r) => {
                self.srl_r8(r)
            },
            CbOpcode::SwapHl => {
                self.swap_hl()
            },
            CbOpcode::SwapR8(r) => {
                self.swap_r8(r)
            },
            CbOpcode::Unknown => {
                panic!("Unrecognized cb opcode");
            }
        }
    }
}