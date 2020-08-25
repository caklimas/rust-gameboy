use super::super::super::super::addresses::video_ram::VIDEO_RAM_LOWER;
use super::super::super::Cpu;
use super::super::opcode::ArithmeticRegister;

#[test]
fn adc_a_test() {
    let mut cpu: Cpu = Default::default();
    let a = 0;
    let b = 1;
    cpu.registers.a = a;
    cpu.registers.b = b;
    cpu.registers.f.set_carry(true);

    cpu.adc_a(&ArithmeticRegister::B);

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
    cpu.registers.set_hl(VIDEO_RAM_LOWER);
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

    cpu.add_a(&ArithmeticRegister::B);

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
    cpu.registers.set_hl(VIDEO_RAM_LOWER);

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

    cpu.and_a(&ArithmeticRegister::B);

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
    cpu.registers.set_hl(VIDEO_RAM_LOWER);

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
fn cp_a_test() {
    let mut cpu: Cpu = Default::default();
    cpu.registers.a = 0;
    cpu.registers.b = 0;

    cpu.cp_a(&ArithmeticRegister::B);

    assert_eq!(true, cpu.registers.f.zero());
    assert_eq!(true, cpu.registers.f.subtraction());
    assert_eq!(false, cpu.registers.f.half_carry());

    cpu.registers.a = 0b1010_0100;
    cpu.registers.b = 0b0001_1110;

    cpu.cp_a(&ArithmeticRegister::B);

    assert_eq!(true, cpu.registers.f.half_carry());
    assert_eq!(false, cpu.registers.f.carry());

    cpu.registers.a = 0;
    cpu.registers.b = 1;

    cpu.cp_a(&ArithmeticRegister::B);

    assert_eq!(true, cpu.registers.f.carry());
}

#[test]
fn cp_d8_test() {
    let mut data = 0;
    let mut cpu: Cpu = Default::default();
    cpu.registers.set_hl(VIDEO_RAM_LOWER);
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
fn or_a_test() {
    let mut cpu: Cpu = Default::default();
    let a = 0;
    let b = 1;
    cpu.registers.a = a;
    cpu.registers.b = b;

    cpu.or_a(&ArithmeticRegister::B);

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
    cpu.registers.set_hl(VIDEO_RAM_LOWER);

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
fn sbc_a_test() {
    let mut cpu: Cpu = Default::default();
    let a = 2;
    let b = 1;
    cpu.registers.a = a;
    cpu.registers.b = b;
    cpu.registers.f.set_carry(true);

    cpu.sbc_a(&ArithmeticRegister::B);

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
    cpu.registers.set_hl(VIDEO_RAM_LOWER);
    cpu.registers.f.set_carry(true);

    cpu.sbc_hl();

    assert_eq!(a - data - 1, cpu.registers.a);
}

#[test]
fn sub_a_test() {
    let mut cpu: Cpu = Default::default();
    let a = 2;
    let b = 1;
    cpu.registers.a = a;
    cpu.registers.b = b;

    cpu.sub_a(&ArithmeticRegister::B);

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
    cpu.registers.set_hl(VIDEO_RAM_LOWER);

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

    cpu.xor_a(&ArithmeticRegister::B);

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
    cpu.registers.set_hl(VIDEO_RAM_LOWER);

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