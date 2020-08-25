use super::super::Cpu;

pub type Clock = (u16, u16);

pub enum Opcode {
    Adc(fn(&mut Cpu, &ArithmeticRegister), ArithmeticRegister, Clock),
    AdcD8(Clock),
    AdcHL(Clock),
    Add(fn(&mut Cpu, &ArithmeticRegister), ArithmeticRegister, Clock),
    AddD8(Clock),
    AddHL(Clock),
    And(fn(&mut Cpu, &ArithmeticRegister), ArithmeticRegister, Clock),
    AndD8(Clock),
    AndHL(Clock),
    Cp(fn(&mut Cpu, &ArithmeticRegister), ArithmeticRegister, Clock),
    CpD8(Clock),
    CpHL(Clock),
    Or(fn(&mut Cpu, &ArithmeticRegister), ArithmeticRegister, Clock),
    OrD8(Clock),
    OrHL(Clock),
    Sbc(fn(&mut Cpu, &ArithmeticRegister), ArithmeticRegister, Clock),
    SbcD8(Clock),
    SbcHL(Clock),
    Sub(fn(&mut Cpu, &ArithmeticRegister), ArithmeticRegister, Clock),
    SubD8(Clock),
    SubHL(Clock),
    XOr(fn(&mut Cpu, &ArithmeticRegister), ArithmeticRegister, Clock),
    XOrD8(Clock),
    XOrHL(Clock)
}

pub enum ArithmeticRegister {
    A,
    B,
    C,
    D,
    E,
    H,
    L
}