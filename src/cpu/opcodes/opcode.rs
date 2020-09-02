pub enum Opcode {
    Adc(CpuRegister),
    AdcD8,
    AdcHl,
    Add(CpuRegister),
    AddD8,
    AddAHl,
    AddHl16(CpuRegister16),
    AddHl16Sp,
    And(CpuRegister),
    AndD8,
    AndHl,
    Call,
    CallCc(Condition),
    Ccf,
    Cp(CpuRegister),
    CpD8,
    CpHl,
    Cpl,
    DecHl,
    DecR(CpuRegister),
    DecSp,
    Dec16(CpuRegister16),
    Ei,
    IncHl,
    IncR(CpuRegister),
    IncSp,
    Inc16(CpuRegister16),
    Jp,
    JpCc(Condition),
    JpHl,
    Jr,
    JrCc(Condition),
    Ld(CpuRegister, CpuRegister),
    LdA8A,
    LdAA8,
    LdA16A,
    LdAA16,
    LdAC,
    LdCA,
    LdD8(CpuRegister),
    LdHlD8,
    LdHlA(bool),
    LdAHl(bool),
    LdA16Sp,
    Ld16R(CpuRegister16, CpuRegister),
    LdR16(CpuRegister, CpuRegister16),
    LdR16D16(CpuRegister16),
    LdSpD16,
    LdSpE8,
    LdSpHl,
    Or(CpuRegister),
    OrD8,
    OrHl,
    Pop(CpuRegister16),
    Push(CpuRegister16),
    Ret,
    RetCc(Condition),
    RetI,
    Rst(u16),
    Sbc(CpuRegister),
    SbcD8,
    SbcHl,
    Scf,
    Sub(CpuRegister),
    SubD8,
    SubHl,
    XOr(CpuRegister),
    XOrD8,
    XOrHl
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

pub enum Condition {
    Z, // Execute if Z is set.
    NZ, // Execute if Z is not set.
    C, // Execute if C is set.
    NC, // Execute if C is not set.
}