use super::super::super::Cpu;
use super::super::opcode::{CpuRegister, CpuRegister16};
use super::super::super::super::addresses::{
    high_ram::HIGH_RAM_LOWER,
    ld_opcode::LD_ADDRESS_LOWER,
    video_ram::VIDEO_RAM_LOWER
};

#[test]
fn ld_test() {
    let mut cpu: Cpu = Default::default();
    let b = 5;
    let c = 0;

    cpu.registers.b = b;
    cpu.registers.c = c;

    cpu.ld(&CpuRegister::C, &CpuRegister::B);

    assert_eq!(b, cpu.registers.c);
}

#[test]
fn ld_a8_a_test() {
    let data = 5;
    let mut cpu: Cpu = Default::default();
    cpu.mmu.write_byte(VIDEO_RAM_LOWER, (HIGH_RAM_LOWER - LD_ADDRESS_LOWER) as u8);
    cpu.program_counter = VIDEO_RAM_LOWER - 1;
    cpu.registers.a = data;

    cpu.ld_a8_a();

    assert_eq!(data, cpu.mmu.read_byte(HIGH_RAM_LOWER));
}

#[test]
fn ld_a_a8_test() {
    let data = 5;
    let mut cpu: Cpu = Default::default();
    cpu.mmu.write_byte(HIGH_RAM_LOWER, data);
    cpu.mmu.write_byte(VIDEO_RAM_LOWER, (HIGH_RAM_LOWER - LD_ADDRESS_LOWER) as u8);
    cpu.program_counter = VIDEO_RAM_LOWER - 1;

    cpu.ld_a_a8();

    assert_eq!(data, cpu.registers.a);
}

#[test]
fn ld_a16_a_test() {
    let data = 5;
    let mut cpu: Cpu = Default::default();
    cpu.mmu.write_word(VIDEO_RAM_LOWER, HIGH_RAM_LOWER);
    cpu.program_counter = VIDEO_RAM_LOWER - 1;
    cpu.registers.a = data;

    cpu.ld_a16_a();

    assert_eq!(data, cpu.mmu.read_byte(HIGH_RAM_LOWER));
}

#[test]
fn ld_a_a16_test() {
    let data = 5;
    let mut cpu: Cpu = Default::default();
    cpu.mmu.write_word(VIDEO_RAM_LOWER, HIGH_RAM_LOWER);
    cpu.mmu.write_byte(HIGH_RAM_LOWER, data);
    cpu.program_counter = VIDEO_RAM_LOWER - 1;
    
    cpu.ld_a_a16();

    assert_eq!(data, cpu.registers.a);
}

#[test]
fn ld_a_c_test() {
    let data = 5;
    let mut cpu: Cpu = Default::default();
    cpu.registers.c = (HIGH_RAM_LOWER - LD_ADDRESS_LOWER) as u8;
    cpu.mmu.write_byte(HIGH_RAM_LOWER, data);

    cpu.ld_a_c();

    assert_eq!(data, cpu.registers.a);
}

#[test]
fn ld_c_a_test() {
    let data = 5;
    let mut cpu: Cpu = Default::default();
    cpu.registers.a = data;
    cpu.registers.c = (HIGH_RAM_LOWER - LD_ADDRESS_LOWER) as u8;

    cpu.ld_c_a();

    assert_eq!(data, cpu.mmu.read_byte(HIGH_RAM_LOWER));
}

#[test]
fn ld_d8_test() {
    let data = 5;
    let mut cpu: Cpu = Default::default();
    cpu.program_counter = VIDEO_RAM_LOWER - 1;
    cpu.mmu.write_byte(VIDEO_RAM_LOWER, data);

    cpu.ld_d8(&CpuRegister::B);

    assert_eq!(data, cpu.registers.b);
}

#[test]
fn ld_hl_d8_test() {
    let data = 5;
    let mut cpu: Cpu = Default::default();
    cpu.program_counter = VIDEO_RAM_LOWER - 1;
    cpu.mmu.write_byte(VIDEO_RAM_LOWER, data);
    cpu.registers.set_target_16(&CpuRegister16::HL, VIDEO_RAM_LOWER + 1);

    cpu.ld_hl_d8();

    assert_eq!(data, cpu.mmu.read_byte(VIDEO_RAM_LOWER + 1));
}

#[test]
fn ld_hl_a_test() {
    let data = 5;
    let mut cpu: Cpu = Default::default();
    cpu.registers.a = data;
    cpu.registers.set_target_16(&CpuRegister16::HL, VIDEO_RAM_LOWER);

    cpu.ld_hl_a(&true);

    assert_eq!(data, cpu.mmu.read_byte(VIDEO_RAM_LOWER));
    assert_eq!(VIDEO_RAM_LOWER + 1, cpu.registers.get_target_16(&CpuRegister16::HL));

    cpu.ld_hl_a(&false);

    assert_eq!(VIDEO_RAM_LOWER, cpu.registers.get_target_16(&CpuRegister16::HL));
}

#[test]
fn ld_a_hl_test() {
    let data = 5;
    let mut cpu: Cpu = Default::default();
    cpu.registers.set_target_16(&CpuRegister16::HL, VIDEO_RAM_LOWER);
    cpu.mmu.write_byte(VIDEO_RAM_LOWER, data);

    cpu.ld_a_hl(&true);

    assert_eq!(data, cpu.registers.a);
    assert_eq!(VIDEO_RAM_LOWER + 1, cpu.registers.get_target_16(&CpuRegister16::HL));

    cpu.ld_hl_a(&false);

    assert_eq!(VIDEO_RAM_LOWER, cpu.registers.get_target_16(&CpuRegister16::HL));
}

#[test]
fn ld_16_r_test() {
    let data = 5;
    let mut cpu: Cpu = Default::default();
    cpu.registers.a = data;
    cpu.registers.set_target_16(&CpuRegister16::BC, VIDEO_RAM_LOWER);

    cpu.ld_16_r(&CpuRegister16::BC, &CpuRegister::A);

    assert_eq!(data, cpu.mmu.read_byte(VIDEO_RAM_LOWER));
}

#[test]
fn ld_r_16_test() {
    let data = 5;
    let mut cpu: Cpu = Default::default();
    cpu.mmu.write_byte(VIDEO_RAM_LOWER, data);
    cpu.registers.set_target_16(&CpuRegister16::BC, VIDEO_RAM_LOWER);

    cpu.ld_r_16(&CpuRegister::A, &CpuRegister16::BC);

    assert_eq!(data, cpu.registers.a);
}