use super::cb_opcode::CbOpcode;
use super::opcode::{CpuRegister};

pub const CB_OPCODE_TABLE: [CbOpcode; 256] = [
             /* 0                              1                                2                                3                                4 	                             5                                6 	               7                                8                                9                                A                                B                                C                                D                                E                  F */
   /* 0 */   CbOpcode::RlcR8(CpuRegister::B),  CbOpcode::RlcR8(CpuRegister::C), CbOpcode::RlcR8(CpuRegister::D), CbOpcode::RlcR8(CpuRegister::E), CbOpcode::RlcR8(CpuRegister::H), CbOpcode::RlcR8(CpuRegister::L), CbOpcode::RlcHl,   CbOpcode::RlcR8(CpuRegister::A), CbOpcode::RrcR8(CpuRegister::B), CbOpcode::RrcR8(CpuRegister::C), CbOpcode::RrcR8(CpuRegister::D), CbOpcode::RrcR8(CpuRegister::E), CbOpcode::RrcR8(CpuRegister::H), CbOpcode::RrcR8(CpuRegister::L), CbOpcode::RrcHl,   CbOpcode::RrcR8(CpuRegister::A),
   /* 1 */   CbOpcode::RlR8(CpuRegister::B),   CbOpcode::RlR8(CpuRegister::C),  CbOpcode::RlR8(CpuRegister::D),  CbOpcode::RlR8(CpuRegister::E),  CbOpcode::RlR8(CpuRegister::H),  CbOpcode::RlR8(CpuRegister::L),  CbOpcode::RlHl,    CbOpcode::RlR8(CpuRegister::A),  CbOpcode::RrR8(CpuRegister::B),  CbOpcode::RrR8(CpuRegister::C),  CbOpcode::RrR8(CpuRegister::D),  CbOpcode::RrR8(CpuRegister::E),  CbOpcode::RrR8(CpuRegister::H),  CbOpcode::RrR8(CpuRegister::L),  CbOpcode::RrHl,    CbOpcode::RrR8(CpuRegister::A),
   /* 2 */   CbOpcode::SlaR8(CpuRegister::B),  CbOpcode::SlaR8(CpuRegister::C), CbOpcode::SlaR8(CpuRegister::D), CbOpcode::SlaR8(CpuRegister::E), CbOpcode::SlaR8(CpuRegister::H), CbOpcode::SlaR8(CpuRegister::L), CbOpcode::SlaHl,   CbOpcode::SlaR8(CpuRegister::A), CbOpcode::SraR8(CpuRegister::B), CbOpcode::SraR8(CpuRegister::C), CbOpcode::SraR8(CpuRegister::D), CbOpcode::SraR8(CpuRegister::E), CbOpcode::SraR8(CpuRegister::H), CbOpcode::SraR8(CpuRegister::L), CbOpcode::SraHl,   CbOpcode::SraR8(CpuRegister::A),
   /* 3 */   CbOpcode::Unknown,                CbOpcode::Unknown,               CbOpcode::Unknown,               CbOpcode::Unknown,               CbOpcode::Unknown,               CbOpcode::Unknown,               CbOpcode::Unknown, CbOpcode::Unknown,               CbOpcode::Unknown,               CbOpcode::Unknown,               CbOpcode::Unknown,               CbOpcode::Unknown,               CbOpcode::Unknown,               CbOpcode::Unknown,               CbOpcode::Unknown, CbOpcode::Unknown,
   /* 4 */   CbOpcode::Unknown,                CbOpcode::Unknown,               CbOpcode::Unknown,               CbOpcode::Unknown,               CbOpcode::Unknown,               CbOpcode::Unknown,               CbOpcode::Unknown, CbOpcode::Unknown,               CbOpcode::Unknown,               CbOpcode::Unknown,               CbOpcode::Unknown,               CbOpcode::Unknown,               CbOpcode::Unknown,               CbOpcode::Unknown,               CbOpcode::Unknown, CbOpcode::Unknown,
   /* 5 */   CbOpcode::Unknown,                CbOpcode::Unknown,               CbOpcode::Unknown,               CbOpcode::Unknown,               CbOpcode::Unknown,               CbOpcode::Unknown,               CbOpcode::Unknown, CbOpcode::Unknown,               CbOpcode::Unknown,               CbOpcode::Unknown,               CbOpcode::Unknown,               CbOpcode::Unknown,               CbOpcode::Unknown,               CbOpcode::Unknown,               CbOpcode::Unknown, CbOpcode::Unknown,
   /* 6 */   CbOpcode::Unknown,                CbOpcode::Unknown,               CbOpcode::Unknown,               CbOpcode::Unknown,               CbOpcode::Unknown,               CbOpcode::Unknown,               CbOpcode::Unknown, CbOpcode::Unknown,               CbOpcode::Unknown,               CbOpcode::Unknown,               CbOpcode::Unknown,               CbOpcode::Unknown,               CbOpcode::Unknown,               CbOpcode::Unknown,               CbOpcode::Unknown, CbOpcode::Unknown,
   /* 7 */   CbOpcode::Unknown,                CbOpcode::Unknown,               CbOpcode::Unknown,               CbOpcode::Unknown,               CbOpcode::Unknown,               CbOpcode::Unknown,               CbOpcode::Unknown, CbOpcode::Unknown,               CbOpcode::Unknown,               CbOpcode::Unknown,               CbOpcode::Unknown,               CbOpcode::Unknown,               CbOpcode::Unknown,               CbOpcode::Unknown,               CbOpcode::Unknown, CbOpcode::Unknown,
   /* 8 */   CbOpcode::Unknown,                CbOpcode::Unknown,               CbOpcode::Unknown,               CbOpcode::Unknown,               CbOpcode::Unknown,               CbOpcode::Unknown,               CbOpcode::Unknown, CbOpcode::Unknown,               CbOpcode::Unknown,               CbOpcode::Unknown,               CbOpcode::Unknown,               CbOpcode::Unknown,               CbOpcode::Unknown,               CbOpcode::Unknown,               CbOpcode::Unknown, CbOpcode::Unknown,
   /* 9 */   CbOpcode::Unknown,                CbOpcode::Unknown,               CbOpcode::Unknown,               CbOpcode::Unknown,               CbOpcode::Unknown,               CbOpcode::Unknown,               CbOpcode::Unknown, CbOpcode::Unknown,               CbOpcode::Unknown,               CbOpcode::Unknown,               CbOpcode::Unknown,               CbOpcode::Unknown,               CbOpcode::Unknown,               CbOpcode::Unknown,               CbOpcode::Unknown, CbOpcode::Unknown,
   /* A */   CbOpcode::Unknown,                CbOpcode::Unknown,               CbOpcode::Unknown,               CbOpcode::Unknown,               CbOpcode::Unknown,               CbOpcode::Unknown,               CbOpcode::Unknown, CbOpcode::Unknown,               CbOpcode::Unknown,               CbOpcode::Unknown,               CbOpcode::Unknown,               CbOpcode::Unknown,               CbOpcode::Unknown,               CbOpcode::Unknown,               CbOpcode::Unknown, CbOpcode::Unknown,
   /* B */   CbOpcode::Unknown,                CbOpcode::Unknown,               CbOpcode::Unknown,               CbOpcode::Unknown,               CbOpcode::Unknown,               CbOpcode::Unknown,               CbOpcode::Unknown, CbOpcode::Unknown,               CbOpcode::Unknown,               CbOpcode::Unknown,               CbOpcode::Unknown,               CbOpcode::Unknown,               CbOpcode::Unknown,               CbOpcode::Unknown,               CbOpcode::Unknown, CbOpcode::Unknown,
   /* C */   CbOpcode::Unknown,                CbOpcode::Unknown,               CbOpcode::Unknown,               CbOpcode::Unknown,               CbOpcode::Unknown,               CbOpcode::Unknown,               CbOpcode::Unknown, CbOpcode::Unknown,               CbOpcode::Unknown,               CbOpcode::Unknown,               CbOpcode::Unknown,               CbOpcode::Unknown,               CbOpcode::Unknown,               CbOpcode::Unknown,               CbOpcode::Unknown, CbOpcode::Unknown,
   /* D */   CbOpcode::Unknown,                CbOpcode::Unknown,               CbOpcode::Unknown,               CbOpcode::Unknown,               CbOpcode::Unknown,               CbOpcode::Unknown,               CbOpcode::Unknown, CbOpcode::Unknown,               CbOpcode::Unknown,               CbOpcode::Unknown,               CbOpcode::Unknown,               CbOpcode::Unknown,               CbOpcode::Unknown,               CbOpcode::Unknown,               CbOpcode::Unknown, CbOpcode::Unknown,
   /* E */   CbOpcode::Unknown,                CbOpcode::Unknown,               CbOpcode::Unknown,               CbOpcode::Unknown,               CbOpcode::Unknown,               CbOpcode::Unknown,               CbOpcode::Unknown, CbOpcode::Unknown,               CbOpcode::Unknown,               CbOpcode::Unknown,               CbOpcode::Unknown,               CbOpcode::Unknown,               CbOpcode::Unknown,               CbOpcode::Unknown,               CbOpcode::Unknown, CbOpcode::Unknown,
   /* F */   CbOpcode::Unknown,                CbOpcode::Unknown,               CbOpcode::Unknown,               CbOpcode::Unknown,               CbOpcode::Unknown,               CbOpcode::Unknown,               CbOpcode::Unknown, CbOpcode::Unknown,               CbOpcode::Unknown,               CbOpcode::Unknown,               CbOpcode::Unknown,               CbOpcode::Unknown,               CbOpcode::Unknown,               CbOpcode::Unknown,               CbOpcode::Unknown, CbOpcode::Unknown
];