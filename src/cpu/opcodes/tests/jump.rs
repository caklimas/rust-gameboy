use super::super::super::Cpu;
use super::super::opcode::{Condition, CpuRegister16};
use super::super::super::super::addresses::video_ram::VIDEO_RAM_LOWER;

#[test]
fn call_test() {
    let data = VIDEO_RAM_LOWER + 1;
    let new_data = 0xFEED;
    let mut cpu: Cpu = Default::default();
    cpu.stack_pointer = VIDEO_RAM_LOWER;
    cpu.program_counter = data;
    cpu.mmu.write_word(data + 1, new_data);

    cpu.call();

    assert_eq!(data, cpu.mmu.read_word(VIDEO_RAM_LOWER));
    assert_eq!(new_data, cpu.program_counter);
}

#[test]
fn call_cc_test() {
    let data = VIDEO_RAM_LOWER + 1;
    let new_data = 0xFEED;
    let mut cpu: Cpu = Default::default();
    cpu.stack_pointer = VIDEO_RAM_LOWER;
    cpu.program_counter = data;
    cpu.mmu.write_word(data + 1, new_data);

    // Condition is false
    cpu.call_cc(&Condition::Z);

    assert_eq!(data, cpu.program_counter);

    // Condition is true
    cpu.call_cc(&Condition::NZ);

    assert_eq!(data, cpu.mmu.read_word(VIDEO_RAM_LOWER));
    assert_eq!(new_data, cpu.program_counter);
}

#[test]
fn jp_test() {
    let data = 0xBEEF;
    let mut cpu: Cpu = Default::default();
    cpu.program_counter = VIDEO_RAM_LOWER - 1;
    cpu.mmu.write_word(VIDEO_RAM_LOWER, data);

    cpu.jp();

    assert_eq!(data, cpu.program_counter);
}

#[test]
fn jp_cc_test() {
    let data = 0xBEEF;
    let mut cpu: Cpu = Default::default();
    cpu.program_counter = VIDEO_RAM_LOWER - 1;
    cpu.mmu.write_word(VIDEO_RAM_LOWER, data);

    cpu.jp_cc(&Condition::Z);

    assert_ne!(data, cpu.program_counter);

    cpu.jp_cc(&Condition::NZ);

    assert_eq!(data, cpu.program_counter);
}

#[test]
fn jp_hl_test() {
    let data = 0xBEEF;
    let mut cpu: Cpu = Default::default();
    cpu.registers.set_target_16(&CpuRegister16::HL, VIDEO_RAM_LOWER);
    cpu.mmu.write_word(VIDEO_RAM_LOWER, data);

    cpu.jp_hl();

    assert_eq!(data, cpu.program_counter);
}

#[test]
fn jr_test() {
    let data = 5;
    let pc = VIDEO_RAM_LOWER - 1;
    let mut cpu: Cpu = Default::default();
    cpu.program_counter = pc;
    cpu.mmu.write_byte(VIDEO_RAM_LOWER, data);

    cpu.jr();

    assert_eq!(pc + (data as u16), cpu.program_counter);
}

#[test]
fn jr_cc_test() {
    let data = 5;
    let pc = VIDEO_RAM_LOWER - 1;
    let mut cpu: Cpu = Default::default();
    cpu.program_counter = pc;
    cpu.mmu.write_byte(VIDEO_RAM_LOWER, data);

    cpu.jr_cc(&Condition::Z);

    assert_ne!(pc + (data as u16), cpu.program_counter);

    cpu.jr_cc(&Condition::NZ);

    assert_eq!(pc + (data as u16), cpu.program_counter);
}

#[test]
fn ret_test() {
    let data = 0xBEEF;
    let original_stack_value = VIDEO_RAM_LOWER - 2;
    let mut cpu: Cpu = Default::default();
    cpu.mmu.write_word(VIDEO_RAM_LOWER, data);
    cpu.stack_pointer = original_stack_value;

    cpu.ret();

    assert_eq!(VIDEO_RAM_LOWER, cpu.stack_pointer);
    assert_eq!(data, cpu.program_counter);
}

#[test]
fn ret_cc_test() {
    let data = 0xBEEF;
    let original_stack_value = VIDEO_RAM_LOWER - 2;
    let mut cpu: Cpu = Default::default();
    cpu.mmu.write_word(VIDEO_RAM_LOWER, data);
    cpu.stack_pointer = original_stack_value;

    let clock = cpu.ret_cc(&Condition::Z);

    assert_eq!(original_stack_value, cpu.stack_pointer);
    assert_eq!((1, 8), clock);

    cpu.registers.f.set_zero(true);
    let clock = cpu.ret_cc(&Condition::Z);

    assert_eq!(VIDEO_RAM_LOWER, cpu.stack_pointer);
    assert_eq!(data, cpu.program_counter);
    assert_eq!((1, 20), clock);
}

#[test]
fn ret_i_test() {
    let data = 0xBEEF;
    let original_stack_value = VIDEO_RAM_LOWER - 2;
    let mut cpu: Cpu = Default::default();
    cpu.mmu.write_word(VIDEO_RAM_LOWER, data);
    cpu.stack_pointer = original_stack_value;
    cpu.interrupt_master_enable = false;

    cpu.ret_i();

    assert_eq!(VIDEO_RAM_LOWER, cpu.stack_pointer);
    assert_eq!(data, cpu.program_counter);
    assert_eq!(true, cpu.interrupt_master_enable);
}

#[test]
fn rst_test() {
    let data = 0xBEEF;
    let reset_value = 0xFEED;
    let mut cpu: Cpu = Default::default();
    cpu.stack_pointer = VIDEO_RAM_LOWER;
    cpu.program_counter = data;

    cpu.rst(reset_value);

    assert_eq!(data, cpu.mmu.read_word(VIDEO_RAM_LOWER));
    assert_eq!(reset_value, cpu.program_counter);
}

#[test]
fn is_condition_met_test() {
    let mut cpu: Cpu = Default::default();
    assert_eq!(false, cpu.is_condition_met(&Condition::Z));
    assert_eq!(true, cpu.is_condition_met(&Condition::NZ));
    assert_eq!(false, cpu.is_condition_met(&Condition::C));
    assert_eq!(true, cpu.is_condition_met(&Condition::NC));

    cpu.registers.f.set_zero(true);
    cpu.registers.f.set_carry(true);

    assert_eq!(true, cpu.is_condition_met(&Condition::Z));
    assert_eq!(false, cpu.is_condition_met(&Condition::NZ));
    assert_eq!(true, cpu.is_condition_met(&Condition::C));
    assert_eq!(false, cpu.is_condition_met(&Condition::NC));
}