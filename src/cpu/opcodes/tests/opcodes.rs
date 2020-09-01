use super::super::super::super::addresses::video_ram::VIDEO_RAM_LOWER;
use super::super::super::Cpu;
use super::super::opcode::{CpuRegister, CpuRegister16};

#[test]
fn adc_a_test() {
    let mut cpu: Cpu = Default::default();
    let a = 0;
    let b = 1;
    cpu.registers.a = a;
    cpu.registers.b = b;
    cpu.registers.f.set_carry(true);

    cpu.adc_a(&CpuRegister::B);

    assert_eq!(a + b + 1, cpu.registers.a);
}

#[test]
fn adc_d8_test() {
    let data = 5;
    let mut cpu: Cpu = Default::default();
    cpu.program_counter = VIDEO_RAM_LOWER - 1;
    cpu.mmu.write_byte(VIDEO_RAM_LOWER, data);

    let a = 1;
    cpu.registers.a = a;
    cpu.registers.f.set_carry(true);

    cpu.adc_d8();

    assert_eq!(a + data + 1, cpu.registers.a);
}

#[test]
fn adc_hl_test() {
    let data = 5;
    let mut cpu: Cpu = Default::default();
    cpu.mmu.write_byte(VIDEO_RAM_LOWER, data);

    let a = 1;
    cpu.registers.a = a;
    cpu.registers.set_target_16(&CpuRegister16::HL, VIDEO_RAM_LOWER);
    cpu.registers.f.set_carry(true);

    cpu.adc_hl();

    assert_eq!(a + data + 1, cpu.registers.a);
}

#[test]
fn add_a_test() {
    let mut cpu: Cpu = Default::default();
    let a = 0;
    let b = 1;
    cpu.registers.a = a;
    cpu.registers.b = b;

    cpu.add_a(&CpuRegister::B);

    assert_eq!(a + b, cpu.registers.a);
}

#[test]
fn add_d8_test() {
    let data = 5;
    let mut cpu: Cpu = Default::default();
    cpu.program_counter = VIDEO_RAM_LOWER - 1;
    cpu.mmu.write_byte(VIDEO_RAM_LOWER, data);

    let a = 1;
    cpu.registers.a = a;

    cpu.add_d8();

    assert_eq!(a + data, cpu.registers.a);
}

#[test]
fn add_hl_test() {
    let data = 5;
    let mut cpu: Cpu = Default::default();
    cpu.mmu.write_byte(VIDEO_RAM_LOWER, data);

    let a = 1;
    cpu.registers.a = a;
    cpu.registers.set_target_16(&CpuRegister16::HL, VIDEO_RAM_LOWER);

    cpu.add_hl();

    assert_eq!(a + data, cpu.registers.a);
}

#[test]
fn add_test() {
    let mut cpu: Cpu = Default::default();
    cpu.registers.a = 0;
    cpu.registers.b = 0;

    cpu.add(cpu.registers.b, false);

    assert_eq!(true, cpu.registers.f.zero());
    assert_eq!(false, cpu.registers.f.subtraction());
    assert_eq!(false, cpu.registers.f.half_carry());

    cpu.registers.a = 0b1000_1111;
    cpu.registers.b = 1;

    cpu.add(cpu.registers.b, false);

    assert_eq!(true, cpu.registers.f.half_carry());
    assert_eq!(false, cpu.registers.f.carry());

    cpu.registers.a = 0b1111_1111;
    cpu.registers.b = 1;
    
    cpu.add(cpu.registers.b, false);

    assert_eq!(true, cpu.registers.f.carry());
}

#[test]
fn and_a_test() {
    let mut cpu: Cpu = Default::default();
    let a = 0;
    let b = 1;
    cpu.registers.a = a;
    cpu.registers.b = b;

    cpu.and_a(&CpuRegister::B);

    assert_eq!(a & b, cpu.registers.a);
}

#[test]
fn and_d8_test() {
    let data = 4;
    let mut cpu: Cpu = Default::default();
    cpu.program_counter = VIDEO_RAM_LOWER - 1;
    cpu.mmu.write_byte(VIDEO_RAM_LOWER, data);

    let a = 5;
    cpu.registers.a = a;

    cpu.and_d8();

    assert_eq!(a & data, cpu.registers.a);
}

#[test]
fn and_hl_test() {
    let data = 5;
    let mut cpu: Cpu = Default::default();
    cpu.mmu.write_byte(VIDEO_RAM_LOWER, data);

    let a = 1;
    cpu.registers.a = a;
    cpu.registers.set_target_16(&CpuRegister16::HL, VIDEO_RAM_LOWER);

    cpu.and_hl();

    assert_eq!(a & data, cpu.registers.a);
}

#[test]
fn and_test() {
    let mut cpu: Cpu = Default::default();
    let a = 5;
    let target = 4;
    cpu.registers.a = a;

    let result = cpu.and(target);

    assert_eq!(a & target, result);
    assert_eq!(false, cpu.registers.f.zero());
    assert_eq!(false, cpu.registers.f.subtraction());
    assert_eq!(true, cpu.registers.f.half_carry());
    assert_eq!(false, cpu.registers.f.carry());

    cpu.registers.a = 0;

    cpu.and(target);

    assert_eq!(true, cpu.registers.f.zero());
}

#[test]
fn ccf_test() {
    let mut cpu: Cpu = Default::default();
    let value = true;
    cpu.registers.f.set_carry(value);
    cpu.registers.f.set_half_carry(true);
    cpu.registers.f.set_subtraction(true);

    cpu.ccf();

    assert_eq!(!value, cpu.registers.f.carry());
    assert_eq!(false, cpu.registers.f.half_carry());
    assert_eq!(false, cpu.registers.f.subtraction());
}

#[test]
fn cp_a_test() {
    let mut cpu: Cpu = Default::default();
    cpu.registers.a = 0;
    cpu.registers.b = 0;

    cpu.cp_a(&CpuRegister::B);

    assert_eq!(true, cpu.registers.f.zero());
    assert_eq!(true, cpu.registers.f.subtraction());
    assert_eq!(false, cpu.registers.f.half_carry());

    cpu.registers.a = 0b1010_0100;
    cpu.registers.b = 0b0001_1110;

    cpu.cp_a(&CpuRegister::B);

    assert_eq!(true, cpu.registers.f.half_carry());
    assert_eq!(false, cpu.registers.f.carry());

    cpu.registers.a = 0;
    cpu.registers.b = 1;

    cpu.cp_a(&CpuRegister::B);

    assert_eq!(true, cpu.registers.f.carry());
}

#[test]
fn cp_d8_test() {
    let mut data = 0;
    let mut cpu: Cpu = Default::default();
    cpu.registers.set_target_16(&CpuRegister16::HL, VIDEO_RAM_LOWER);
    cpu.mmu.write_byte(VIDEO_RAM_LOWER, data);
    cpu.registers.a = 0;

    cpu.cp_d8();

    assert_eq!(true, cpu.registers.f.zero());
    assert_eq!(true, cpu.registers.f.subtraction());
    assert_eq!(false, cpu.registers.f.half_carry());

    cpu.registers.a = 0b1010_0100;
    data = 0b0001_1110;
    cpu.mmu.write_byte(VIDEO_RAM_LOWER, data);

    cpu.cp_hl();

    assert_eq!(true, cpu.registers.f.half_carry());
    assert_eq!(false, cpu.registers.f.carry());

    cpu.registers.a = 0;
    data = 1;
    cpu.mmu.write_byte(VIDEO_RAM_LOWER, data);

    cpu.cp_hl();

    assert_eq!(true, cpu.registers.f.carry());
}

#[test]
fn cp_hl_test() {
    let mut data = 0;
    let mut cpu: Cpu = Default::default();
    cpu.program_counter = VIDEO_RAM_LOWER - 1;
    cpu.mmu.write_byte(VIDEO_RAM_LOWER, data);
    cpu.registers.a = 0;

    cpu.cp_hl();

    assert_eq!(true, cpu.registers.f.zero());
    assert_eq!(true, cpu.registers.f.subtraction());
    assert_eq!(false, cpu.registers.f.half_carry());

    cpu.registers.a = 0b1010_0100;
    data = 0b0001_1110;
    cpu.mmu.write_byte(VIDEO_RAM_LOWER, data);

    cpu.cp_d8();

    assert_eq!(true, cpu.registers.f.half_carry());
    assert_eq!(false, cpu.registers.f.carry());

    cpu.registers.a = 0;
    data = 1;
    cpu.mmu.write_byte(VIDEO_RAM_LOWER, data);

    cpu.cp_d8();

    assert_eq!(true, cpu.registers.f.carry());
}

#[test]
fn cpl_test() {
    let mut cpu: Cpu = Default::default();
    let value = 5;
    cpu.registers.a = value;
    cpu.registers.f.set_half_carry(false);
    cpu.registers.f.set_subtraction(false);

    cpu.cpl();

    assert_eq!(!value, cpu.registers.a);
    assert_eq!(true, cpu.registers.f.half_carry());
    assert_eq!(true, cpu.registers.f.subtraction());
}

#[test]
fn dec_hl_test() {
    let mut cpu: Cpu = Default::default();
    let target = 5;
    cpu.registers.set_target_16(&CpuRegister16::HL, VIDEO_RAM_LOWER);
    cpu.mmu.write_byte(VIDEO_RAM_LOWER, target);

    cpu.dec_hl();

    assert_eq!(target .wrapping_sub(1), cpu.mmu.read_byte(VIDEO_RAM_LOWER));
}

#[test]
fn dec_r_test() {
    let mut cpu: Cpu = Default::default();
    let target = 5;
    cpu.registers.a = target;

    cpu.dec_r(&CpuRegister::A);

    assert_eq!(target .wrapping_sub(1), cpu.registers.a);
}

#[test]
fn dec_sp_test() {
    let mut cpu: Cpu = Default::default();
    let data = 5;
    cpu.stack_pointer = VIDEO_RAM_LOWER;
    cpu.mmu.write_byte(VIDEO_RAM_LOWER, data);

    cpu.dec_sp();

    assert_eq!(data .wrapping_sub(1), cpu.mmu.read_byte(VIDEO_RAM_LOWER));
}

#[test]
fn dec_16_test() {
    let mut cpu: Cpu = Default::default();
    let data = 5;
    cpu.registers.set_target_16(&CpuRegister16::BC, VIDEO_RAM_LOWER);
    cpu.mmu.write_byte(VIDEO_RAM_LOWER, data);

    cpu.dec_16(&CpuRegister16::BC);

    assert_eq!(data .wrapping_sub(1), cpu.mmu.read_byte(VIDEO_RAM_LOWER));
}

#[test]
fn dec_8_test() {
    let mut cpu: Cpu = Default::default();
    let mut target = 5;

    let result = cpu.dec_8(target);

    assert_eq!(target .wrapping_sub(1), result);
    assert_eq!(false, cpu.registers.f.half_carry());
    assert_eq!(true, cpu.registers.f.subtraction());
    assert_eq!(false, cpu.registers.f.zero());

    target = 1;
    cpu.dec_8(target);
    assert_eq!(true, cpu.registers.f.zero());

    target = 0;
    cpu.dec_8(target);
    assert_eq!(true, cpu.registers.f.half_carry());
}

#[test]
fn inc_hl_test() {
    let mut cpu: Cpu = Default::default();
    let target = 5;
    cpu.registers.set_target_16(&CpuRegister16::HL, VIDEO_RAM_LOWER);
    cpu.mmu.write_byte(VIDEO_RAM_LOWER, target);

    cpu.inc_hl();

    assert_eq!(target + 1, cpu.mmu.read_byte(VIDEO_RAM_LOWER));
}

#[test]
fn inc_r_test() {
    let mut cpu: Cpu = Default::default();
    let target = 5;
    cpu.registers.a = target;

    cpu.inc_r(&CpuRegister::A);

    assert_eq!(target + 1, cpu.registers.a);
}

#[test]
fn inc_sp_test() {
    let mut cpu: Cpu = Default::default();
    let data = 5;
    cpu.stack_pointer = VIDEO_RAM_LOWER;
    cpu.mmu.write_byte(VIDEO_RAM_LOWER, data);

    cpu.inc_sp();

    assert_eq!(data + 1, cpu.mmu.read_byte(VIDEO_RAM_LOWER));
}

#[test]
fn inc_16_test() {
    let mut cpu: Cpu = Default::default();
    let data = 5;
    cpu.registers.set_target_16(&CpuRegister16::BC, VIDEO_RAM_LOWER);
    cpu.mmu.write_byte(VIDEO_RAM_LOWER, data);

    cpu.inc_16(&CpuRegister16::BC);

    assert_eq!(data + 1, cpu.mmu.read_byte(VIDEO_RAM_LOWER));
}

#[test]
fn inc_8_test() {
    let mut cpu: Cpu = Default::default();
    let mut target = 5;

    let result = cpu.inc_8(target);

    assert_eq!(target + 1, result);
    assert_eq!(false, cpu.registers.f.half_carry());
    assert_eq!(false, cpu.registers.f.subtraction());
    assert_eq!(false, cpu.registers.f.zero());

    target = 0xFF;
    cpu.inc_8(target);
    assert_eq!(true, cpu.registers.f.zero());

    target = 0b1000_1111;
    cpu.inc_8(target);
    assert_eq!(true, cpu.registers.f.half_carry());
}

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
fn ld_a16_sp_test() {
    let data = 0xBEEF;
    let pc_address = VIDEO_RAM_LOWER;
    let address = VIDEO_RAM_LOWER + 5;
    let mut cpu: Cpu = Default::default();
    cpu.mmu.write_word(pc_address, address);
    cpu.program_counter = pc_address - 1;
    cpu.stack_pointer = data;

    cpu.ld_a16_sp();

    assert_eq!(data, cpu.mmu.read_word(address));
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

#[test]
fn ld_r16_d16_test() {
    let register = &CpuRegister16::BC;
    let data = 0xBEEF;
    let mut cpu: Cpu = Default::default();
    cpu.program_counter = VIDEO_RAM_LOWER - 1;
    cpu.mmu.write_word(VIDEO_RAM_LOWER, data);

    cpu.ld_r16_d16(register);

    assert_eq!(data, cpu.registers.get_target_16(register));
}

#[test]
fn ld_sp_d16_test() {
    let data = 0xBEEF;
    let mut cpu: Cpu = Default::default();
    cpu.program_counter = VIDEO_RAM_LOWER - 1;
    cpu.mmu.write_word(VIDEO_RAM_LOWER, data);

    cpu.ld_sp_d16();

    assert_eq!(data, cpu.stack_pointer);
}

#[test]
fn ld_sp_e8_test() {
    let mut data = 5;
    let address = VIDEO_RAM_LOWER;
    let mut cpu: Cpu = Default::default();
    cpu.program_counter = address - 1;
    cpu.mmu.write_byte(address, data);

    cpu.ld_sp_e8();

    assert_eq!(data as u16, cpu.registers.get_target_16(&CpuRegister16::HL));
    assert_eq!(false, cpu.registers.f.carry());
    assert_eq!(false, cpu.registers.f.half_carry());
    assert_eq!(false, cpu.registers.f.subtraction());
    assert_eq!(false, cpu.registers.f.zero());

    cpu.stack_pointer = 0xFFFF - 1;

    cpu.ld_sp_e8();

    assert_eq!(true, cpu.registers.f.carry());

    data = 62;
    cpu.stack_pointer = 34;
    cpu.mmu.write_byte(address, data);

    cpu.ld_sp_e8();

    assert_eq!(true, cpu.registers.f.half_carry());
}

#[test]
fn ld_sp_hl_test() {
    let register = &CpuRegister16::HL;
    let data = 5;
    let address = VIDEO_RAM_LOWER;
    let mut cpu: Cpu = Default::default();
    cpu.registers.set_target_16(register, address);
    cpu.stack_pointer = address + 1;
    cpu.mmu.write_byte(address, data);

    cpu.ld_sp_hl();

    assert_eq!(data, cpu.mmu.read_byte(address + 1));
}

#[test]
fn or_a_test() {
    let mut cpu: Cpu = Default::default();
    let a = 0;
    let b = 1;
    cpu.registers.a = a;
    cpu.registers.b = b;

    cpu.or_a(&CpuRegister::B);

    assert_eq!(a | b, cpu.registers.a);
}

#[test]
fn or_d8_test() {
    let data = 4;
    let mut cpu: Cpu = Default::default();
    cpu.program_counter = VIDEO_RAM_LOWER - 1;
    cpu.mmu.write_byte(VIDEO_RAM_LOWER, data);

    let a = 5;
    cpu.registers.a = a;

    cpu.or_d8();

    assert_eq!(a | data, cpu.registers.a);
}

#[test]
fn or_hl_test() {
    let data = 5;
    let mut cpu: Cpu = Default::default();
    cpu.mmu.write_byte(VIDEO_RAM_LOWER, data);

    let a = 1;
    cpu.registers.a = a;
    cpu.registers.set_target_16(&CpuRegister16::HL, VIDEO_RAM_LOWER);

    cpu.or_hl();

    assert_eq!(a | data, cpu.registers.a);
}

#[test]
fn or_test() {
    let mut cpu: Cpu = Default::default();
    let a = 5;
    let mut target = 4;
    cpu.registers.a = a;

    let result = cpu.or(target);

    assert_eq!(a | target, result);
    assert_eq!(false, cpu.registers.f.zero());
    assert_eq!(false, cpu.registers.f.subtraction());
    assert_eq!(false, cpu.registers.f.half_carry());
    assert_eq!(false, cpu.registers.f.carry());

    cpu.registers.a = 0;
    target = 0;

    cpu.or(target);

    assert_eq!(true, cpu.registers.f.zero());
}

#[test]
fn pop_test() {
    let data = 5;
    let address = VIDEO_RAM_LOWER + 2;
    let mut cpu: Cpu = Default::default();
    cpu.stack_pointer = VIDEO_RAM_LOWER;
    cpu.mmu.write_word(address, data);

    cpu.pop(&CpuRegister16::AF);

    assert_eq!(address, cpu.stack_pointer);
    assert_eq!(cpu.mmu.read_word(address), cpu.registers.get_target_16(&CpuRegister16::AF));
}

#[test]
fn push_test() {
    let data = 5;
    let address = VIDEO_RAM_LOWER + 2;
    let mut cpu: Cpu = Default::default();
    cpu.stack_pointer = address;
    cpu.registers.set_target_16(&CpuRegister16::BC, data);

    cpu.push(&CpuRegister16::BC);

    assert_eq!(VIDEO_RAM_LOWER, cpu.stack_pointer);
    assert_eq!(cpu.registers.get_target_16(&CpuRegister16::BC), cpu.mmu.read_word(address));
}

#[test]
fn sbc_a_test() {
    let mut cpu: Cpu = Default::default();
    let a = 2;
    let b = 1;
    cpu.registers.a = a;
    cpu.registers.b = b;
    cpu.registers.f.set_carry(true);

    cpu.sbc_a(&CpuRegister::B);

    assert_eq!(a - b - 1, cpu.registers.a);
}

#[test]
fn sbc_d8_test() {
    let data = 4;
    let mut cpu: Cpu = Default::default();
    cpu.program_counter = VIDEO_RAM_LOWER - 1;
    cpu.mmu.write_byte(VIDEO_RAM_LOWER, data);

    let a = 5;
    cpu.registers.a = a;
    cpu.registers.f.set_carry(true);

    cpu.sbc_d8();

    assert_eq!(a - data - 1, cpu.registers.a);
}

#[test]
fn sbc_hl_test() {
    let data = 1;
    let mut cpu: Cpu = Default::default();
    cpu.mmu.write_byte(VIDEO_RAM_LOWER, data);

    let a = 2;
    cpu.registers.a = a;
    cpu.registers.set_target_16(&CpuRegister16::HL, VIDEO_RAM_LOWER);
    cpu.registers.f.set_carry(true);

    cpu.sbc_hl();

    assert_eq!(a - data - 1, cpu.registers.a);
}

#[test]
fn scf_test() {
    let mut cpu: Cpu = Default::default();
    cpu.registers.f.set_carry(false);
    cpu.registers.f.set_half_carry(true);
    cpu.registers.f.set_subtraction(true);
    cpu.registers.f.set_zero(true);

    cpu.scf();

    assert_eq!(true, cpu.registers.f.carry());
    assert_eq!(false, cpu.registers.f.half_carry());
    assert_eq!(false, cpu.registers.f.subtraction());
    assert_eq!(true, cpu.registers.f.zero());
}

#[test]
fn sub_a_test() {
    let mut cpu: Cpu = Default::default();
    let a = 2;
    let b = 1;
    cpu.registers.a = a;
    cpu.registers.b = b;

    cpu.sub_a(&CpuRegister::B);

    assert_eq!(a - b, cpu.registers.a);
}

#[test]
fn sub_d8_test() {
    let data = 4;
    let mut cpu: Cpu = Default::default();
    cpu.program_counter = VIDEO_RAM_LOWER - 1;
    cpu.mmu.write_byte(VIDEO_RAM_LOWER, data);

    let a = 5;
    cpu.registers.a = a;

    cpu.sub_d8();

    assert_eq!(a - data, cpu.registers.a);
}

#[test]
fn sub_hl_test() {
    let data = 5;
    let mut cpu: Cpu = Default::default();
    cpu.mmu.write_byte(VIDEO_RAM_LOWER, data);

    let a = data + 2;
    cpu.registers.a = a;
    cpu.registers.set_target_16(&CpuRegister16::HL, VIDEO_RAM_LOWER);

    cpu.sub_hl();

    assert_eq!(a - data, cpu.registers.a);
}

#[test]
fn sub_test() {
    let mut cpu: Cpu = Default::default();
    cpu.registers.a = 0;
    cpu.registers.b = 0;

    cpu.sub(cpu.registers.b, false);

    assert_eq!(true, cpu.registers.f.zero());
    assert_eq!(true, cpu.registers.f.subtraction());
    assert_eq!(false, cpu.registers.f.half_carry());

    cpu.registers.a = 0b1010_0100;
    cpu.registers.b = 0b0001_1110;

    cpu.sub(cpu.registers.b, false);

    assert_eq!(true, cpu.registers.f.half_carry());
    assert_eq!(false, cpu.registers.f.carry());

    cpu.registers.a = 0;
    cpu.registers.b = 1;

    cpu.sub(cpu.registers.b, false);

    assert_eq!(true, cpu.registers.f.carry());
}

#[test]
fn xor_a_test() {
    let mut cpu: Cpu = Default::default();
    let a = 0;
    let b = 1;
    cpu.registers.a = a;
    cpu.registers.b = b;

    cpu.xor_a(&CpuRegister::B);

    assert_eq!(a ^ b, cpu.registers.a);
}

#[test]
fn xor_d8_test() {
    let data = 4;
    let mut cpu: Cpu = Default::default();
    cpu.program_counter = VIDEO_RAM_LOWER - 1;
    cpu.mmu.write_byte(VIDEO_RAM_LOWER, data);

    let a = 5;
    cpu.registers.a = a;

    cpu.xor_d8();

    assert_eq!(a ^ data, cpu.registers.a);
}

#[test]
fn xor_hl_test() {
    let data = 5;
    let mut cpu: Cpu = Default::default();
    cpu.mmu.write_byte(VIDEO_RAM_LOWER, data);

    let a = 1;
    cpu.registers.a = a;
    cpu.registers.set_target_16(&CpuRegister16::HL, VIDEO_RAM_LOWER);

    cpu.xor_hl();

    assert_eq!(a ^ data, cpu.registers.a);
}

#[test]
fn xor_test() {
    let mut cpu: Cpu = Default::default();
    let a = 5;
    let target = 4;
    cpu.registers.a = a;

    let result = cpu.xor(target);

    assert_eq!(a ^ target, result);
    assert_eq!(false, cpu.registers.f.zero());
    assert_eq!(false, cpu.registers.f.subtraction());
    assert_eq!(false, cpu.registers.f.half_carry());
    assert_eq!(false, cpu.registers.f.carry());

    cpu.registers.a = target;

    cpu.xor(target);

    assert_eq!(true, cpu.registers.f.zero());
}