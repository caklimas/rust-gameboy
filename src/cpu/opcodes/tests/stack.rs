use super::super::super::Cpu;
use super::super::opcode::CpuRegister16;
use crate::addresses::gpu::video_ram::VIDEO_RAM_LOWER;

#[test]
fn ld_a16_sp_test() {
    let data = 0xBEEF;
    let pc_address = VIDEO_RAM_LOWER;
    let address = VIDEO_RAM_LOWER + 5;
    let mut cpu: Cpu = Default::default();
    cpu.mmu.write_word(pc_address, address);
    cpu.program_counter = pc_address;
    cpu.stack_pointer = data;

    cpu.ld_a16_sp();

    assert_eq!(data, cpu.mmu.read_word(address));
}

#[test]
fn ld_r16_d16_test() {
    let register = &CpuRegister16::BC;
    let data = 0xBEEF;
    let mut cpu: Cpu = Default::default();
    cpu.program_counter = VIDEO_RAM_LOWER;
    cpu.mmu.write_word(VIDEO_RAM_LOWER, data);

    cpu.ld_r16_d16(register);

    assert_eq!(data, cpu.registers.get_target_16(register));
}

#[test]
fn ld_sp_d16_test() {
    let data = 0xBEEF;
    let mut cpu: Cpu = Default::default();
    cpu.program_counter = VIDEO_RAM_LOWER;
    cpu.mmu.write_word(VIDEO_RAM_LOWER, data);

    cpu.ld_sp_d16();

    assert_eq!(data, cpu.stack_pointer);
}

#[test]
fn ld_sp_e8_test() {
    let mut data = 5;
    let address = VIDEO_RAM_LOWER;
    let mut cpu: Cpu = Default::default();
    cpu.program_counter = address;
    cpu.mmu.write_byte(cpu.program_counter, data);

    cpu.ld_hl_sp_e8();

    assert_eq!(data as u16, cpu.registers.get_target_16(&CpuRegister16::HL));
    assert_eq!(false, cpu.registers.f.carry());
    assert_eq!(false, cpu.registers.f.half_carry());
    assert_eq!(false, cpu.registers.f.subtraction());
    assert_eq!(false, cpu.registers.f.zero());

    cpu.stack_pointer = 0xFFFF - 1;

    cpu.program_counter = address;
    cpu.ld_hl_sp_e8();

    assert_eq!(true, cpu.registers.f.carry());

    data = 62;
    cpu.stack_pointer = 34;
    cpu.mmu.write_byte(cpu.program_counter, data);

    cpu.ld_hl_sp_e8();

    assert_eq!(true, cpu.registers.f.half_carry());
}

#[test]
fn ld_sp_hl_test() {
    let register = &CpuRegister16::HL;
    let data = 5;
    let mut cpu: Cpu = Default::default();
    cpu.registers.set_target_16(register, data);

    cpu.ld_sp_hl();

    assert_eq!(data, cpu.stack_pointer);
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
    assert_eq!(
        cpu.mmu.read_word(VIDEO_RAM_LOWER),
        cpu.registers.get_target_16(&CpuRegister16::AF)
    );
}

#[test]
fn pop_stack_test() {
    let data = 0xBEEF;
    let mut cpu: Cpu = Default::default();
    cpu.mmu.write_word(VIDEO_RAM_LOWER, data);
    cpu.stack_pointer = VIDEO_RAM_LOWER;

    let value = cpu.pop_stack();

    assert_eq!(VIDEO_RAM_LOWER + 2, cpu.stack_pointer);
    assert_eq!(data, value);
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
    assert_eq!(
        cpu.registers.get_target_16(&CpuRegister16::BC),
        cpu.mmu.read_word(address - 2)
    );
}

#[test]
fn push_stack_test() {
    let data = 0xBEEF;
    let mut cpu: Cpu = Default::default();
    cpu.stack_pointer = VIDEO_RAM_LOWER + 2;

    cpu.push_stack(data);

    assert_eq!(VIDEO_RAM_LOWER, cpu.stack_pointer);
    assert_eq!(data, cpu.mmu.read_word(VIDEO_RAM_LOWER));
}
