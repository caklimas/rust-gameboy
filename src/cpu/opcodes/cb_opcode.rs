use super::opcode::CpuRegister;

pub enum CbOpcode {
    RlHl,
    RlR8(CpuRegister),
    RlcHl,
    RlcR8(CpuRegister),
    RrHl,
    RrR8(CpuRegister),
    RrcHl,
    RrcR8(CpuRegister),
    SlaHl,
    SlaR8(CpuRegister),
    SraHl,
    SraR8(CpuRegister),
    Unknown
}