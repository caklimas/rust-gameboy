pub mod flags_register;
pub mod index_registers;
pub mod opcodes;
pub mod registers;

#[cfg(test)]
mod tests;

use crate::cartridge::Cartridge;
use crate::constants::cpu::PROGRAM_START;
use crate::mmu::interrupts::Interrupt;
use crate::mmu::Mmu;
use opcodes::{
    cb_opcode::CbOpcode,
    cb_opcode_table::CB_OPCODE_TABLE,
    opcode::{CpuRegister16, Opcode},
    opcode_table::OPCODE_TABLE,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default)]
pub struct Cpu {
    pub master_clock_cycles: u32,
    pub mmu: Mmu,
    cb_opcode: bool,
    halted: bool,
    index_registers: index_registers::IndexRegisters,
    interrupt_master_enable: bool,
    interrupt_page_address: u8,
    memory_refresh: u8,
    opcode: usize,
    program_counter: u16,
    registers: registers::Registers,
    stack_pointer: u16,
    stopped: bool,
}

impl Cpu {
    pub fn new(cartridge: Cartridge, run_boot_rom: bool) -> Self {
        let mut cpu = Cpu {
            mmu: Mmu::new(cartridge, run_boot_rom),
            ..Default::default()
        };

        cpu.program_start(run_boot_rom);
        cpu
    }

    pub fn clock(&mut self) -> (u16, bool) {
        let mut cycles = 0;
        let mut audio_buffer_full = false;
        if let Some(c) = self.handle_interrupts() {
            audio_buffer_full = self.mmu.clock(c);
            cycles = c;
        } else if let Some(c) = self.execute() {
            audio_buffer_full = self.mmu.clock(c);
            cycles = c;
        }

        self.master_clock_cycles += cycles as u32;
        (cycles, audio_buffer_full)
    }

    pub fn frame_complete(&self) -> bool {
        self.mmu.ram.gpu.lcd.frame_complete
    }

    pub fn get_screen(&self) -> &[u8] {
        self.mmu.ram.gpu.lcd.screen.get_pixels()
    }

    pub fn get_audio_buffer(&self) -> Vec<f32> {
        self.mmu.ram.apu.get_audio_buffer()
    }

    pub fn read_byte(&mut self) -> u8 {
        let data = self.mmu.read_byte(self.program_counter);
        self.program_counter += 1;
        data
    }

    pub fn read_word(&mut self) -> u16 {
        let data = self.mmu.read_word(self.program_counter);
        self.program_counter += 2;
        data
    }

    fn execute(&mut self) -> Option<u16> {
        if self.halted {
            return Some(self.nop());
        }

        if self.program_counter == PROGRAM_START {
            self.mmu.finish_running_boot_rom();
        }

        self.opcode = self.read_byte() as usize;

        let mut clock_cycle = self.execute_opcode();
        if self.cb_opcode {
            self.cb_opcode = false;
            self.opcode = self.read_byte() as usize;
            clock_cycle = Some(self.execute_cb_opcode());
        }

        clock_cycle
    }

    fn execute_opcode(&mut self) -> Option<u16> {
        if let Some(ref o) = OPCODE_TABLE[self.opcode] {
            match o {
                Opcode::Adc(r) => Some(self.adc_a(r)),
                Opcode::AdcD8 => Some(self.adc_d8()),
                Opcode::AdcHl => Some(self.adc_hl()),
                Opcode::Add(r) => Some(self.add_a(r)),
                Opcode::AddD8 => Some(self.add_d8()),
                Opcode::AddAHl => Some(self.add_a_hl()),
                Opcode::AddHl16(r) => Some(self.add_hl_16(r)),
                Opcode::AddHlSp => Some(self.add_hl_sp()),
                Opcode::AddSpE8 => Some(self.add_sp_e8()),
                Opcode::And(r) => Some(self.and_a(r)),
                Opcode::AndD8 => Some(self.and_d8()),
                Opcode::AndHl => Some(self.and_hl()),
                Opcode::Call => Some(self.call()),
                Opcode::CallCc(condition) => Some(self.call_cc(condition)),
                Opcode::Cb => Some(self.prefix_cb()),
                Opcode::Ccf => Some(self.ccf()),
                Opcode::Cp(r) => Some(self.cp_a(r)),
                Opcode::CpD8 => Some(self.cp_d8()),
                Opcode::CpHl => Some(self.cp_hl()),
                Opcode::Cpl => Some(self.cpl()),
                Opcode::Daa => Some(self.daa()),
                Opcode::DecHl => Some(self.dec_hl()),
                Opcode::DecR(r) => Some(self.dec_r(r)),
                Opcode::DecSp => Some(self.dec_sp()),
                Opcode::Dec16(r) => Some(self.dec_16(r)),
                Opcode::Di => Some(self.di()),
                Opcode::Ei => Some(self.ei()),
                Opcode::Halt => Some(self.halt()),
                Opcode::IncHl => Some(self.inc_hl()),
                Opcode::IncR(r) => Some(self.inc_r(r)),
                Opcode::IncSp => Some(self.inc_sp()),
                Opcode::Inc16(r) => Some(self.inc_16(r)),
                Opcode::Jp => Some(self.jp()),
                Opcode::JpCc(condition) => Some(self.jp_cc(condition)),
                Opcode::JpHl => Some(self.jp_hl()),
                Opcode::Jr => Some(self.jr()),
                Opcode::JrCc(condition) => Some(self.jr_cc(condition)),
                Opcode::Ld(dest, src) => Some(self.ld(dest, src)),
                Opcode::LdA8A => Some(self.ld_a8_a()),
                Opcode::LdAA8 => Some(self.ld_a_a8()),
                Opcode::LdA16A => Some(self.ld_a16_a()),
                Opcode::LdAA16 => Some(self.ld_a_a16()),
                Opcode::LdAC => Some(self.ld_a_c()),
                Opcode::LdCA => Some(self.ld_c_a()),
                Opcode::LdD8(r) => Some(self.ld_d8(r)),
                Opcode::LdHlD8 => Some(self.ld_hl_d8()),
                Opcode::LdHlA(increment) => Some(self.ld_hl_a(increment)),
                Opcode::LdAHl(increment) => Some(self.ld_a_hl(increment)),
                Opcode::LdA16Sp => Some(self.ld_a16_sp()),
                Opcode::Ld16R(r16, r) => Some(self.ld_16_r(r16, r)),
                Opcode::LdR16(r, r16) => Some(self.ld_r_16(r, r16)),
                Opcode::LdR16D16(r) => Some(self.ld_r16_d16(r)),
                Opcode::LdSpD16 => Some(self.ld_sp_d16()),
                Opcode::LdHlSpE8 => Some(self.ld_hl_sp_e8()),
                Opcode::LdSpHl => Some(self.ld_sp_hl()),
                Opcode::Nop => Some(self.nop()),
                Opcode::Or(r) => Some(self.or_a(r)),
                Opcode::OrD8 => Some(self.or_d8()),
                Opcode::OrHl => Some(self.or_hl()),
                Opcode::Pop(r) => Some(self.pop(r)),
                Opcode::Push(r) => Some(self.push(r)),
                Opcode::Ret => Some(self.ret()),
                Opcode::RetCc(condition) => Some(self.ret_cc(condition)),
                Opcode::RetI => Some(self.ret_i()),
                Opcode::Rla => Some(self.rl_a()),
                Opcode::Rlca => Some(self.rlc_a()),
                Opcode::Rra => Some(self.rr_a()),
                Opcode::Rrca => Some(self.rrc_a()),
                Opcode::Rst(v) => Some(self.rst(*v)),
                Opcode::Sbc(r) => Some(self.sbc_a(r)),
                Opcode::SbcD8 => Some(self.sbc_d8()),
                Opcode::SbcHl => Some(self.sbc_hl()),
                Opcode::Scf => Some(self.scf()),
                Opcode::Stop => Some(self.stop()),
                Opcode::Sub(r) => Some(self.sub_a(r)),
                Opcode::SubD8 => Some(self.sub_d8()),
                Opcode::SubHl => Some(self.sub_hl()),
                Opcode::XOr(r) => Some(self.xor_a(r)),
                Opcode::XOrD8 => Some(self.xor_d8()),
                Opcode::XOrHl => Some(self.xor_hl()),
            }
        } else {
            None
        }
    }

    fn execute_cb_opcode(&mut self) -> u16 {
        let o = &CB_OPCODE_TABLE[self.opcode];
        match o {
            CbOpcode::BitNSetHl(b) => self.bit_n_set_hl(*b),
            CbOpcode::BitNSetR8(r, b) => self.bit_n_set_r8(r, *b),
            CbOpcode::ResHl(b) => self.res_hl(*b),
            CbOpcode::ResR8(r, b) => self.res_r8(r, *b),
            CbOpcode::RlHl => self.rl_hl(),
            CbOpcode::RlR8(r) => self.rl_r8(r),
            CbOpcode::RlcHl => self.rlc_hl(),
            CbOpcode::RlcR8(r) => self.rlc_r8(r),
            CbOpcode::RrHl => self.rr_hl(),
            CbOpcode::RrR8(r) => self.rr_r8(r),
            CbOpcode::RrcHl => self.rrc_hl(),
            CbOpcode::RrcR8(r) => self.rrc_r8(r),
            CbOpcode::SetHl(b) => self.set_hl(*b),
            CbOpcode::SetR8(r, b) => self.set_r8(r, *b),
            CbOpcode::SlaHl => self.sla_hl(),
            CbOpcode::SlaR8(r) => self.sla_r8(r),
            CbOpcode::SraHl => self.sra_hl(),
            CbOpcode::SraR8(r) => self.sra_r8(r),
            CbOpcode::SrlHl => self.srl_hl(),
            CbOpcode::SrlR8(r) => self.srl_r8(r),
            CbOpcode::SwapHl => self.swap_hl(),
            CbOpcode::SwapR8(r) => self.swap_r8(r),
            CbOpcode::Unknown => {
                panic!("Unrecognized cb opcode");
            }
        }
    }

    fn handle_interrupts(&mut self) -> Option<u16> {
        if !self.interrupt_master_enable && !self.halted {
            return None;
        }

        let interrupt =
            Interrupt(self.mmu.ram.interrupt_enable.get() & self.mmu.ram.interrupt_flag.get());
        if interrupt.get() == 0 {
            return None;
        }

        self.halted = false;
        if !self.interrupt_master_enable {
            return None;
        }

        self.interrupt_master_enable = false;
        self.mmu.ram.interrupt_flag.set(0);
        self.handle_interrupt(interrupt)
    }

    fn handle_interrupt(&mut self, interrupt: Interrupt) -> Option<u16> {
        if interrupt.v_blank() {
            self.mmu.ram.interrupt_flag.set_v_blank(false);
            Some(self.rst(0x0040))
        } else if interrupt.lcd_stat() {
            self.mmu.ram.interrupt_flag.set_lcd_stat(false);
            Some(self.rst(0x0048))
        } else if interrupt.timer() {
            self.mmu.ram.interrupt_flag.set_timer(false);
            Some(self.rst(0x0050))
        } else if interrupt.serial() {
            self.mmu.ram.interrupt_flag.set_serial(false);
            Some(self.rst(0x0058))
        } else if interrupt.joypad() {
            self.mmu.ram.interrupt_flag.set_joypad(false);
            Some(self.rst(0x0060))
        } else {
            None
        }
    }

    fn program_start(&mut self, run_boot_rom: bool) {
        if run_boot_rom {
            return;
        }

        self.program_counter = PROGRAM_START;
        self.stack_pointer = 0xFFFE;
        self.registers.set_target_16(&CpuRegister16::AF, 0x01B0);
        self.registers.set_target_16(&CpuRegister16::BC, 0x0013);
        self.registers.set_target_16(&CpuRegister16::DE, 0x00D8);
        self.registers.set_target_16(&CpuRegister16::HL, 0x014D);
    }
}
