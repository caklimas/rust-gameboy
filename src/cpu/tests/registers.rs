use super::super::registers::Registers;
use super::super::opcodes::opcode::ArithmeticRegister;

#[test]
fn get_bc_test() {
    let mut registers: Registers = Default::default();
    registers.b = 1;
    registers.c = 1;

    let expected = (1 << 8) as u16 | (1 as u16);
    let actual = registers.get_bc();
    assert_eq!(expected, actual);
}

#[test]
fn set_bc_test() {
    let b = 1 as u16;
    let c = 1 as u16;
    let bc = (b << 8) | c;
    let mut registers: Registers = Default::default();
    registers.set_bc(bc);

    assert_eq!(b as u8, registers.b);
    assert_eq!(c as u8, registers.c);
}

#[test]
pub fn get_de_test() {
    let mut registers: Registers = Default::default();
    registers.d = 1;
    registers.e = 1;

    let expected = (1 << 8) as u16 | (1 as u16);
    let actual = registers.get_de();
    assert_eq!(expected, actual);
}

#[test]
pub fn set_de_test() {
    let d = 1 as u16;
    let e = 1 as u16;
    let de = (d << 8) | e;
    let mut registers: Registers = Default::default();
    registers.set_de(de);

    assert_eq!(d as u8, registers.d);
    assert_eq!(e as u8, registers.e);
}

#[test]
pub fn get_hl_test() {
    let mut registers: Registers = Default::default();
    registers.h = 1;
    registers.l = 1;

    let expected = (1 << 8) as u16 | (1 as u16);
    let actual = registers.get_hl();
    assert_eq!(expected, actual);
}

#[test]
pub fn set_hl_test() {
    let h = 1 as u16;
    let l = 1 as u16;
    let hl = (h << 8) | l;
    let mut registers: Registers = Default::default();
    registers.set_hl(hl);

    assert_eq!(h as u8, registers.h);
    assert_eq!(l as u8, registers.l);
}

#[test]
pub fn get_target_test() {
    let mut registers: Registers = Default::default();
    registers.a = 1;
    registers.b = 2;
    registers.c = 3;
    registers.d = 4;
    registers.e = 5;
    registers.h = 6;
    registers.l = 7;

    let target_a = registers.get_target(&ArithmeticRegister::A);
    let target_b = registers.get_target(&ArithmeticRegister::B);
    let target_c = registers.get_target(&ArithmeticRegister::C);
    let target_d = registers.get_target(&ArithmeticRegister::D);
    let target_e = registers.get_target(&ArithmeticRegister::E);
    let target_h = registers.get_target(&ArithmeticRegister::H);
    let target_l = registers.get_target(&ArithmeticRegister::L);

    assert_eq!(target_a, registers.a);
    assert_eq!(target_b, registers.b);
    assert_eq!(target_c, registers.c);
    assert_eq!(target_d, registers.d);
    assert_eq!(target_e, registers.e);
    assert_eq!(target_h, registers.h);
    assert_eq!(target_l, registers.l);
}