pub enum Opcode {
    Adc(CpuRegister),
    AdcD8,
    AdcHl,
    Add(CpuRegister),
    AddD8,
    AddAHl,
    AddHl16(CpuRegister16),
    AddHlSp,
    AddSpE8,
    And(CpuRegister),
    AndD8,
    AndHl,
    Call,
    CallCc(Condition),
    Cb,
    Ccf,
    Cp(CpuRegister),
    CpD8,
    CpHl,
    Cpl,
    Daa,
    DecHl,
    DecR(CpuRegister),
    DecSp,
    Dec16(CpuRegister16),
    Di,
    Ei,
    Halt,
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
    LdHlSpE8,
    LdSpHl,
    Nop,
    Or(CpuRegister),
    OrD8,
    OrHl,
    Pop(CpuRegister16),
    Push(CpuRegister16),
    Ret,
    RetCc(Condition),
    RetI,
    Rla,
    Rlca,
    Rra,
    Rrca,
    Rst(u16),
    Sbc(CpuRegister),
    SbcD8,
    SbcHl,
    Scf,
    Stop,
    Sub(CpuRegister),
    SubD8,
    SubHl,
    XOr(CpuRegister),
    XOrD8,
    XOrHl
}

#[derive(Debug)]
pub enum CpuRegister {
    A,
    B,
    C,
    D,
    E,
    H,
    L
}

#[derive(Debug)]
pub enum CpuRegister16 {
    AF,
    BC,
    DE,
    HL
}

#[derive(Debug)]
pub enum Condition {
    Z, // Execute if Z is set.
    NZ, // Execute if Z is not set.
    C, // Execute if C is set.
    NC, // Execute if C is not set.
}