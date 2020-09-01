pub type ClockCycle = (u16, u16);

pub enum Opcode {
    Adc(CpuRegister, ClockCycle),
    AdcD8(ClockCycle),
    AdcHl(ClockCycle),
    Add(CpuRegister, ClockCycle),
    AddD8(ClockCycle),
    AddHl(ClockCycle),
    And(CpuRegister, ClockCycle),
    AndD8(ClockCycle),
    AndHl(ClockCycle),
    Ccf(ClockCycle),
    Cp(CpuRegister, ClockCycle),
    CpD8(ClockCycle),
    CpHl(ClockCycle),
    Cpl(ClockCycle),
    DecHl(ClockCycle),
    DecR(CpuRegister, ClockCycle),
    DecSp(ClockCycle),
    Dec16(CpuRegister16, ClockCycle),
    IncHl(ClockCycle),
    IncR(CpuRegister, ClockCycle),
    IncSp(ClockCycle),
    Inc16(CpuRegister16, ClockCycle),
    Ld(CpuRegister, CpuRegister, ClockCycle),
    LdA8A(ClockCycle),
    LdAA8(ClockCycle),
    LdA16A(ClockCycle),
    LdAA16(ClockCycle),
    LdAC(ClockCycle),
    LdCA(ClockCycle),
    LdD8(CpuRegister, ClockCycle),
    LdHlD8(ClockCycle),
    LdHlA(bool, ClockCycle),
    LdAHl(bool, ClockCycle),
    LdA16Sp(ClockCycle),
    Ld16R(CpuRegister16, CpuRegister, ClockCycle),
    LdR16(CpuRegister, CpuRegister16, ClockCycle),
    LdR16D16(CpuRegister16, ClockCycle),
    LdSpD16(ClockCycle),
    LdSpE8(ClockCycle),
    LdSpHl(ClockCycle),
    Or(CpuRegister, ClockCycle),
    OrD8(ClockCycle),
    OrHl(ClockCycle),
    Pop(CpuRegister16, ClockCycle),
    Push(CpuRegister16, ClockCycle),
    Sbc(CpuRegister, ClockCycle),
    SbcD8(ClockCycle),
    SbcHl(ClockCycle),
    Scf(ClockCycle),
    Sub(CpuRegister, ClockCycle),
    SubD8(ClockCycle),
    SubHl(ClockCycle),
    XOr(CpuRegister, ClockCycle),
    XOrD8(ClockCycle),
    XOrHl(ClockCycle)
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
    AF,
    BC,
    DE,
    HL
}