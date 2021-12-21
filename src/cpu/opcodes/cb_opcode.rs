use super::opcode::CpuRegister;

pub enum CbOpcode {
    BitNSetHl(u8),
    BitNSetR8(CpuRegister, u8),
    ResHl(u8),
    ResR8(CpuRegister, u8),
    RlHl,
    RlR8(CpuRegister),
    RlcHl,
    RlcR8(CpuRegister),
    RrHl,
    RrR8(CpuRegister),
    RrcHl,
    RrcR8(CpuRegister),
    SetHl(u8),
    SetR8(CpuRegister, u8),
    SlaHl,
    SlaR8(CpuRegister),
    SraHl,
    SraR8(CpuRegister),
    SrlHl,
    SrlR8(CpuRegister),
    SwapHl,
    SwapR8(CpuRegister),
    Unknown
}