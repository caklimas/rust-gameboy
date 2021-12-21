use crate::addresses::gpu::video_ram::VIDEO_RAM_LOWER;
use crate::cpu::opcodes::opcode::{CpuRegister, CpuRegister16};
use crate::cpu::Cpu;

#[test]
fn bit_n_set_hl_test() {
    let address = VIDEO_RAM_LOWER;
    let data = 0b0010_0000;
    let mut cpu: Cpu = Default::default();
    cpu.registers.set_target_16(&CpuRegister16::HL, address);
    cpu.mmu.write_byte(address, data);

    cpu.bit_n_set_hl(6);

    assert_eq!(true, cpu.registers.f.zero());
}

#[test]
fn bit_n_set_r8_test() {
    let value = 0b0010_0000;
    let mut cpu: Cpu = Default::default();
    cpu.registers.a = value;

    cpu.bit_n_set_r8(&CpuRegister::A, 6);

    assert_eq!(true, cpu.registers.f.zero());
}

#[test]
fn bit_n_set_test() {
    let value = 0b0100_0000;
    let mut cpu: Cpu = Default::default();
    cpu.registers.f.set_half_carry(false);

    cpu.bit_n_set(6, value);

    assert_eq!(true, cpu.registers.f.half_carry());
    assert_eq!(false, cpu.registers.f.subtraction());
    assert_eq!(false, cpu.registers.f.zero());

    let value = 0b0000_0010;
    cpu.bit_n_set(6, value);

    assert_eq!(true, cpu.registers.f.zero());
}

#[test]
fn res_hl_test() {
    let value = 0b1110_0000;
    let address = VIDEO_RAM_LOWER;
    let register = &CpuRegister16::HL;
    let mut cpu: Cpu = Default::default();
    cpu.registers.set_target_16(register, address);
    cpu.mmu.write_byte(address, value);

    cpu.res_hl(5);

    assert_eq!(0b1100_0000, cpu.mmu.read_byte(address));
}

#[test]
fn res_r8_test() {
    let value = 0b1110_0000;
    let mut cpu: Cpu = Default::default();
    cpu.registers.a = value;

    cpu.res_r8(&CpuRegister::A, 5);

    assert_eq!(0b1100_0000, cpu.registers.a);
}

#[test]
fn res_8_test() {
    let value = 0b1110_0000;
    let cpu: Cpu = Default::default();

    let result = cpu.res_8(value, 5);

    assert_eq!(0b1100_0000, result);
}

#[test]
fn rl_a_test() {
    let mut cpu: Cpu = Default::default();
    cpu.registers.a = 0b1000_1100;
    cpu.registers.f.set_zero(true);

    cpu.rl_a();

    assert_eq!(0b0001_1000, cpu.registers.a);
    assert_eq!(false, cpu.registers.f.zero());
}

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
fn rlc_a_test() {
    let mut cpu: Cpu = Default::default();
    cpu.registers.a = 0b1000_1100;
    cpu.registers.f.set_zero(true);

    cpu.rlc_a();

    assert_eq!(0b0001_1001, cpu.registers.a);
    assert_eq!(false, cpu.registers.f.zero());
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
fn rr_a_test() {
    let mut cpu: Cpu = Default::default();
    cpu.registers.a = 0b1000_1100;
    cpu.registers.f.set_zero(true);

    cpu.rr_a();

    assert_eq!(0b0100_0110, cpu.registers.a);
    assert_eq!(false, cpu.registers.f.zero());
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
fn rrc_a_test() {
    let mut cpu: Cpu = Default::default();
    cpu.registers.a = 0b1000_1100;
    cpu.registers.f.set_zero(true);

    cpu.rrc_a();

    assert_eq!(0b0100_0110, cpu.registers.a);
    assert_eq!(false, cpu.registers.f.zero());
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
fn set_hl_test() {
    let value = 0b1100_0000;
    let address = VIDEO_RAM_LOWER;
    let register = &CpuRegister16::HL;
    let mut cpu: Cpu = Default::default();
    cpu.registers.set_target_16(register, address);
    cpu.mmu.write_byte(address, value);

    cpu.set_hl(5);

    assert_eq!(0b1110_0000, cpu.mmu.read_byte(address));
}

#[test]
fn set_r8_test() {
    let value = 0b1100_0000;
    let mut cpu: Cpu = Default::default();
    cpu.registers.a = value;

    cpu.set_r8(&CpuRegister::A, 5);

    assert_eq!(0b1110_0000, cpu.registers.a);
}

#[test]
fn set_8_test() {
    let value = 0b1100_0000;
    let cpu: Cpu = Default::default();

    let result = cpu.set_8(value, 5);

    assert_eq!(0b1110_0000, result);
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
fn sra_hl_test() {
    let data = 0b1000_0001;
    let register = &CpuRegister16::HL;
    let mut cpu: Cpu = Default::default();
    cpu.mmu.write_byte(VIDEO_RAM_LOWER, data);
    cpu.registers.set_target_16(register, VIDEO_RAM_LOWER);

    cpu.sra_hl();

    assert_eq!(0b1100_0000, cpu.mmu.read_byte(VIDEO_RAM_LOWER));
}

#[test]
fn sra_r8_test() {
    let register = &CpuRegister::A;
    let mut cpu: Cpu = Default::default();
    cpu.registers.a = 0b1000_0001;

    cpu.sra_r8(register);

    assert_eq!(0b1100_0000, cpu.registers.a);
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
    assert_eq!(0b1100_0000, result);

    let value = 0;
    cpu.sra_8(value);

    assert_eq!(true, cpu.registers.f.zero());
}

#[test]
fn srl_hl_test() {
    let data = 0b1000_0001;
    let register = &CpuRegister16::HL;
    let mut cpu: Cpu = Default::default();
    cpu.mmu.write_byte(VIDEO_RAM_LOWER, data);
    cpu.registers.set_target_16(register, VIDEO_RAM_LOWER);

    cpu.srl_hl();

    assert_eq!(0b0100_0000, cpu.mmu.read_byte(VIDEO_RAM_LOWER));
}

#[test]
fn srl_r8_test() {
    let register = &CpuRegister::A;
    let mut cpu: Cpu = Default::default();
    cpu.registers.a = 0b1000_0001;

    cpu.srl_r8(register);

    assert_eq!(0b0100_0000, cpu.registers.a);
}

#[test]
fn srl_8_test() {
    let mut cpu: Cpu = Default::default();
    let value = 0b1000_0001;

    let result = cpu.srl_8(value);

    assert_eq!(true, cpu.registers.f.carry());
    assert_eq!(false, cpu.registers.f.half_carry());
    assert_eq!(false, cpu.registers.f.subtraction());
    assert_eq!(false, cpu.registers.f.zero());
    assert_eq!(0b0100_0000, result);

    let value = 0;
    cpu.srl_8(value);

    assert_eq!(true, cpu.registers.f.zero());
}

#[test]
fn swap_hl_test() {
    let address = VIDEO_RAM_LOWER;
    let data = 0b0100_1000;
    let register = &CpuRegister16::HL;
    let mut cpu: Cpu = Default::default();
    cpu.registers.set_target_16(register, address);
    cpu.mmu.write_byte(address, data);

    cpu.swap_hl();

    assert_eq!(0b1000_0100, cpu.mmu.read_byte(address));
}

#[test]
fn swap_r8_test() {
    let mut cpu: Cpu = Default::default();
    cpu.registers.a = 0b0100_1000;
    cpu.registers.f.set_carry(true);
    cpu.registers.f.set_half_carry(true);
    cpu.registers.f.set_subtraction(true);

    cpu.swap_r8(&CpuRegister::A);

    assert_eq!(0b1000_0100, cpu.registers.a);
    assert_eq!(false, cpu.registers.f.carry());
    assert_eq!(false, cpu.registers.f.half_carry());
    assert_eq!(false, cpu.registers.f.subtraction());
    assert_eq!(false, cpu.registers.f.zero());

    cpu.registers.a = 0;
    cpu.swap_r8(&CpuRegister::A);

    assert_eq!(true, cpu.registers.f.zero());
}

#[test]
fn swap_test() {
    let mut cpu: Cpu = Default::default();
    let value = 0b0100_1000;

    let result = cpu.swap(value);

    assert_eq!(false, cpu.registers.f.zero());
    assert_eq!(0b1000_0100, result);

    let value = 0;
    cpu.swap(value);

    assert_eq!(true, cpu.registers.f.zero());
}
