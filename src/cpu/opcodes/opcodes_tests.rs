use super::super::Cpu;
use super::instruction::ArithmeticRegister;

#[test]
pub fn add_a_test() {
    let mut cpu: Cpu = Default::default();
    let a = 0;
    let b = 1;
    cpu.registers.a = a;
    cpu.registers.b = b;

    cpu.add_a(ArithmeticRegister::B);

    assert_eq!(a + b, cpu.registers.a);
}

#[test]
pub fn add_test() {
    let mut cpu: Cpu = Default::default();
    cpu.registers.a = 0;
    cpu.registers.b = 0;

    cpu.add(ArithmeticRegister::B);

    assert_eq!(true, cpu.registers.f.zero());
    assert_eq!(false, cpu.registers.f.subtraction());
    assert_eq!(false, cpu.registers.f.half_carry());

    cpu.registers.a = 0b1000_1111;
    cpu.registers.b = 1;
    cpu.add(ArithmeticRegister::B);

    assert_eq!(true, cpu.registers.f.half_carry());
    assert_eq!(false, cpu.registers.f.carry());

    cpu.registers.a = 0b1111_1111;
    cpu.registers.b = 1;
    cpu.add(ArithmeticRegister::B);

    assert_eq!(true, cpu.registers.f.carry());
}