pub type Clock = (u16, u16);

pub enum Opcode {
    Adc(CpuRegister, Clock),
    AdcD8(Clock),
    AdcHL(Clock),
    Add(CpuRegister, Clock),
    AddD8(Clock),
    AddHL(Clock),
    And(CpuRegister, Clock),
    AndD8(Clock),
    AndHL(Clock),
    Cp(CpuRegister, Clock),
    CpD8(Clock),
    CpHL(Clock),
    Ld(CpuRegister, CpuRegister, Clock),
    LdD8(CpuRegister, Clock),
    LdHlD8(Clock),
    LdHlA(bool, Clock),
    LdAHl(bool, Clock),
    Ld16R(CpuRegister16, CpuRegister, Clock),
    LdR16(CpuRegister, CpuRegister16, Clock),
    Or(CpuRegister, Clock),
    OrD8(Clock),
    OrHL(Clock),
    Sbc(CpuRegister, Clock),
    SbcD8(Clock),
    SbcHL(Clock),
    Sub(CpuRegister, Clock),
    SubD8(Clock),
    SubHL(Clock),
    XOr(CpuRegister, Clock),
    XOrD8(Clock),
    XOrHL(Clock)
}

pub enum CpuRegister {
    A,
    B,
    C,
    D,
    E,
    H,
    L
}

pub enum CpuRegister16 {
    BC,
    DE,
    HL
}