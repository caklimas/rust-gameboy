use super::super::super::Cpu;
use super::super::opcode::{CpuRegister, CpuRegister16};
use super::super::super::super::addresses::video_ram::VIDEO_RAM_LOWER;

#[test]
fn rl_hl_test() {
    let data = 0b1000_1100;
    let register = &CpuRegister16::HL;
    let mut cpu: Cpu = Default::default();
    cpu.mmu.write_byte(VIDEO_RAM_LOWER, data);
    cpu.registers.set_target_16(register, VIDEO_RAM_LOWER);

    cpu.rl_hl();

    assert_eq!(0b0001_1000, cpu.mmu.read_byte(VIDEO_RAM_LOWER));
}

#[test]
fn rl_r8_test() {
    let register = &CpuRegister::A;
    let mut cpu: Cpu = Default::default();
    cpu.registers.a = 0b1000_1100;

    cpu.rl_r8(register);

    assert_eq!(0b0001_1000, cpu.registers.a);
}

#[test]
fn rl_8_test() {
    let mut cpu: Cpu = Default::default();
    let value = 0b1000_1100;

    let new_value = cpu.rl_8(value);

    assert_eq!(true, cpu.registers.f.carry());
    assert_eq!(false, cpu.registers.f.half_carry());
    assert_eq!(false, cpu.registers.f.subtraction());
    assert_eq!(false, cpu.registers.f.zero());
    assert_eq!(0b0001_1000, new_value);

    cpu.registers.f.set_carry(true);
    let value = 0b0000_1100;
    let new_value = cpu.rl_8(value);
    
    assert_eq!(false, cpu.registers.f.carry());
    assert_eq!(0b0001_1001, new_value);

    cpu.registers.f.set_carry(false);
    let value = 0;
    cpu.rl_8(value);
    assert_eq!(true, cpu.registers.f.zero());
}

#[test]
fn rlc_hl_test() {
    let data = 0b1000_1100;
    let register = &CpuRegister16::HL;
    let mut cpu: Cpu = Default::default();
    cpu.mmu.write_byte(VIDEO_RAM_LOWER, data);
    cpu.registers.set_target_16(register, VIDEO_RAM_LOWER);

    cpu.rlc_hl();

    assert_eq!(0b0001_1001, cpu.mmu.read_byte(VIDEO_RAM_LOWER));
}

#[test]
fn rlc_r8_test() {
    let register = &CpuRegister::A;
    let mut cpu: Cpu = Default::default();
    cpu.registers.a = 0b1000_1100;

    cpu.rlc_r8(register);

    assert_eq!(0b0001_1001, cpu.registers.a);
}

#[test]
fn rlc_8_test() {
    let mut cpu: Cpu = Default::default();
    let value = 0b1000_1100;

    let new_value = cpu.rlc_8(value);

    assert_eq!(true, cpu.registers.f.carry());
    assert_eq!(false, cpu.registers.f.half_carry());
    assert_eq!(false, cpu.registers.f.subtraction());
    assert_eq!(false, cpu.registers.f.zero());
    assert_eq!(0b0001_1001, new_value);

    let value = 0b0000_1100;
    let new_value = cpu.rlc_8(value);
    
    assert_eq!(false, cpu.registers.f.carry());
    assert_eq!(0b0001_1000, new_value);

    let value = 0;
    cpu.rlc_8(value);
    assert_eq!(true, cpu.registers.f.zero());
}

#[test]
fn rr_hl_test() {
    let data = 0b1000_1100;
    let register = &CpuRegister16::HL;
    let mut cpu: Cpu = Default::default();
    cpu.mmu.write_byte(VIDEO_RAM_LOWER, data);
    cpu.registers.set_target_16(register, VIDEO_RAM_LOWER);

    cpu.rr_hl();

    assert_eq!(0b0100_0110, cpu.mmu.read_byte(VIDEO_RAM_LOWER));
}

#[test]
fn rr_r8_test() {
    let register = &CpuRegister::A;
    let mut cpu: Cpu = Default::default();
    cpu.registers.a = 0b1000_1100;

    cpu.rr_r8(register);

    assert_eq!(0b0100_0110, cpu.registers.a);
}

#[test]
fn rr_8_test() {
    let mut cpu: Cpu = Default::default();
    let value = 0b0000_1101;

    let new_value = cpu.rr_8(value);

    assert_eq!(true, cpu.registers.f.carry());
    assert_eq!(false, cpu.registers.f.half_carry());
    assert_eq!(false, cpu.registers.f.subtraction());
    assert_eq!(false, cpu.registers.f.zero());
    assert_eq!(0b0000_0110, new_value);

    cpu.registers.f.set_carry(true);
    let value = 0b0000_1100;
    let new_value = cpu.rr_8(value);
    
    assert_eq!(false, cpu.registers.f.carry());
    assert_eq!(0b1000_0110, new_value);

    cpu.registers.f.set_carry(false);
    let value = 0;
    cpu.rr_8(value);
    assert_eq!(true, cpu.registers.f.zero());
}

#[test]
fn rrc_hl_test() {
    let data = 0b1000_1100;
    let register = &CpuRegister16::HL;
    let mut cpu: Cpu = Default::default();
    cpu.mmu.write_byte(VIDEO_RAM_LOWER, data);
    cpu.registers.set_target_16(register, VIDEO_RAM_LOWER);

    cpu.rrc_hl();

    assert_eq!(0b0100_0110, cpu.mmu.read_byte(VIDEO_RAM_LOWER));
}

#[test]
fn rrc_r8_test() {
    let register = &CpuRegister::A;
    let mut cpu: Cpu = Default::default();
    cpu.registers.a = 0b1000_1100;

    cpu.rrc_r8(register);

    assert_eq!(0b0100_0110, cpu.registers.a);
}

#[test]
fn rrc_8_test() {
    let mut cpu: Cpu = Default::default();
    let value = 0b0000_1101;

    let new_value = cpu.rrc_8(value);

    assert_eq!(true, cpu.registers.f.carry());
    assert_eq!(false, cpu.registers.f.half_carry());
    assert_eq!(false, cpu.registers.f.subtraction());
    assert_eq!(false, cpu.registers.f.zero());
    assert_eq!(0b1000_0110, new_value);

    let value = 0b0000_1100;
    let new_value = cpu.rrc_8(value);
    
    assert_eq!(false, cpu.registers.f.carry());
    assert_eq!(0b0000_0110, new_value);

    let value = 0;
    cpu.rrc_8(value);
    assert_eq!(true, cpu.registers.f.zero());
}

#[test]
fn sla_hl_test() {
    let data = 0b1000_1100;
    let register = &CpuRegister16::HL;
    let mut cpu: Cpu = Default::default();
    cpu.mmu.write_byte(VIDEO_RAM_LOWER, data);
    cpu.registers.set_target_16(register, VIDEO_RAM_LOWER);

    cpu.sla_hl();

    assert_eq!(0b0001_1000, cpu.mmu.read_byte(VIDEO_RAM_LOWER));
}

#[test]
fn sla_r8_test() {
    let register = &CpuRegister::A;
    let mut cpu: Cpu = Default::default();
    cpu.registers.a = 0b1000_1100;

    cpu.sla_r8(register);

    assert_eq!(0b0001_1000, cpu.registers.a);
}

#[test]
fn sla_8_test() {
    let mut cpu: Cpu = Default::default();
    let value = 0b1000_0001;

    let result = cpu.sla_8(value);

    assert_eq!(true, cpu.registers.f.carry());
    assert_eq!(false, cpu.registers.f.half_carry());
    assert_eq!(false, cpu.registers.f.subtraction());
    assert_eq!(false, cpu.registers.f.zero());
    assert_eq!(0b0000_0010, result);

    let value = 0;
    cpu.sla_8(value);

    assert_eq!(true, cpu.registers.f.zero());
}

#[test]
fn sra_8_test() {
    let mut cpu: Cpu = Default::default();
    let value = 0b1000_0001;

    let result = cpu.sra_8(value);

    assert_eq!(true, cpu.registers.f.carry());
    assert_eq!(false, cpu.registers.f.half_carry());
    assert_eq!(false, cpu.registers.f.subtraction());
    assert_eq!(false, cpu.registers.f.zero());
    assert_eq!(0b0100_0000, result);

    let value = 0;
    cpu.sra_8(value);

    assert_eq!(true, cpu.registers.f.zero());
}