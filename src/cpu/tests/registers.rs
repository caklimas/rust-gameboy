use super::super::registers::Registers;
use super::super::opcodes::opcode::{CpuRegister, CpuRegister16};

#[test]
fn get_target_test() {
    let mut registers: Registers = Default::default();
    registers.a = 1;
    registers.b = 2;
    registers.c = 3;
    registers.d = 4;
    registers.e = 5;
    registers.h = 6;
    registers.l = 7;

    let target_a = registers.get_target(&CpuRegister::A);
    let target_b = registers.get_target(&CpuRegister::B);
    let target_c = registers.get_target(&CpuRegister::C);
    let target_d = registers.get_target(&CpuRegister::D);
    let target_e = registers.get_target(&CpuRegister::E);
    let target_h = registers.get_target(&CpuRegister::H);
    let target_l = registers.get_target(&CpuRegister::L);

    assert_eq!(target_a, registers.a);
    assert_eq!(target_b, registers.b);
    assert_eq!(target_c, registers.c);
    assert_eq!(target_d, registers.d);
    assert_eq!(target_e, registers.e);
    assert_eq!(target_h, registers.h);
    assert_eq!(target_l, registers.l);
}

#[test]
fn set_target_test() {
    let mut registers: Registers = Default::default();
    let a = 1;
    let b = 2;
    let c = 3;
    let d = 4;
    let e = 5;
    let h = 6;
    let l = 7;

    registers.set_target(&CpuRegister::A, a);
    registers.set_target(&CpuRegister::B, b);
    registers.set_target(&CpuRegister::C, c);
    registers.set_target(&CpuRegister::D, d);
    registers.set_target(&CpuRegister::E, e);
    registers.set_target(&CpuRegister::H, h);
    registers.set_target(&CpuRegister::L, l);

    assert_eq!(a, registers.a);
    assert_eq!(b, registers.b);
    assert_eq!(c, registers.c);
    assert_eq!(d, registers.d);
    assert_eq!(e, registers.e);
    assert_eq!(h, registers.h);
    assert_eq!(l, registers.l);
}

#[test]
fn target_16_test() {
    let mut registers: Registers = Default::default();
    let af = 1;
    let bc = 2;
    let de = 3;
    let hl = 4;
    registers.set_target_16(&CpuRegister16::AF, af);
    registers.set_target_16(&CpuRegister16::BC, bc);
    registers.set_target_16(&CpuRegister16::DE, de);
    registers.set_target_16(&CpuRegister16::HL, hl);

    assert_eq!(af & 0xF0, registers.get_target_16(&CpuRegister16::AF));
    assert_eq!(bc, registers.get_target_16(&CpuRegister16::BC));
    assert_eq!(de, registers.get_target_16(&CpuRegister16::DE));
    assert_eq!(hl, registers.get_target_16(&CpuRegister16::HL));
}