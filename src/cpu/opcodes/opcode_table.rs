use super::opcode::{CpuRegister, CpuRegister16, Opcode};

pub const OPCODE_TABLE: [Option<Opcode>; 256] = [
             /* 0                                                            1                                                           	 2                                                               3                                                               4 														         5 														         6 																         7 														         8 													       9 												         A 														         B 													       C 												         D 													       E                                                               F */
   /* 0 */   None,                                                           None,                                                           Some(Opcode::Ld16R(CpuRegister16::BC, CpuRegister::A, (1, 8))), None,                                                           None,                                                           None,                                                           Some(Opcode::LdD8(CpuRegister::B, (2, 8))),                             None,                                                           None,                                                     None,                                                     Some(Opcode::LdR16(CpuRegister::A, CpuRegister16::BC, (1, 8))), None,                                                     None,                                                     None,                                                     Some(Opcode::LdD8(CpuRegister::C, (2, 8))),                     None,
   /* 1 */   None,                                                           None,                                                           Some(Opcode::Ld16R(CpuRegister16::DE, CpuRegister::A, (1, 8))), None,                                                           None,                                                           None,                                                           Some(Opcode::LdD8(CpuRegister::D, (2, 8))),                             None,                                                           None,                                                     None,                                                     Some(Opcode::LdR16(CpuRegister::A, CpuRegister16::DE, (1, 8))), None,                                                     None,                                                     None,                                                     Some(Opcode::LdD8(CpuRegister::E, (2, 8))),                     None,
   /* 2 */   None,                                                           None,                                                           Some(Opcode::LdHlA(true, (1, 8))),                              None,                                                           None,                                                           None,                                                           Some(Opcode::LdD8(CpuRegister::H, (2, 8))),                             None,                                                           None,                                                     None,                                                     Some(Opcode::LdAHl(true, (1, 8))),                              None,                                                     None,                                                     None,                                                     Some(Opcode::LdD8(CpuRegister::L, (2, 8))),                     None,
   /* 3 */   None,                                                           None,                                                           Some(Opcode::LdHlA(false, (1, 8))),                             None,                                                           None,                                                           None,                                                           Some(Opcode::LdHlD8((2, 12))),                                          None,                                                           None,                                                     None,                                                     Some(Opcode::LdAHl(false, (1, 8))),                             None,                                                     None,                                                     None,                                                     Some(Opcode::LdD8(CpuRegister::A, (2, 8))),                     None,
   /* 4 */   Some(Opcode::Ld(CpuRegister::B, CpuRegister::B, (1, 4))),       Some(Opcode::Ld(CpuRegister::B, CpuRegister::C, (1, 4))),       Some(Opcode::Ld(CpuRegister::B, CpuRegister::D, (1, 4))),       Some(Opcode::Ld(CpuRegister::B, CpuRegister::E, (1, 4))),       Some(Opcode::Ld(CpuRegister::B, CpuRegister::H, (1, 4))),       Some(Opcode::Ld(CpuRegister::B, CpuRegister::L, (1, 4))),       Some(Opcode::LdR16(CpuRegister::B, CpuRegister16::HL, (1, 8))),         Some(Opcode::Ld(CpuRegister::B, CpuRegister::A, (1, 4))),       Some(Opcode::Ld(CpuRegister::C, CpuRegister::B, (1, 4))), Some(Opcode::Ld(CpuRegister::C, CpuRegister::C, (1, 4))), Some(Opcode::Ld(CpuRegister::C, CpuRegister::D, (1, 4))),       Some(Opcode::Ld(CpuRegister::C, CpuRegister::E, (1, 4))), Some(Opcode::Ld(CpuRegister::C, CpuRegister::H, (1, 4))), Some(Opcode::Ld(CpuRegister::C, CpuRegister::L, (1, 4))), Some(Opcode::LdR16(CpuRegister::C, CpuRegister16::HL, (1, 8))), Some(Opcode::Ld(CpuRegister::C, CpuRegister::A, (1, 4))),
   /* 5 */   Some(Opcode::Ld(CpuRegister::D, CpuRegister::B, (1, 4))),       Some(Opcode::Ld(CpuRegister::D, CpuRegister::C, (1, 4))),       Some(Opcode::Ld(CpuRegister::D, CpuRegister::D, (1, 4))),       Some(Opcode::Ld(CpuRegister::D, CpuRegister::E, (1, 4))),       Some(Opcode::Ld(CpuRegister::D, CpuRegister::H, (1, 4))),       Some(Opcode::Ld(CpuRegister::D, CpuRegister::L, (1, 4))),       Some(Opcode::LdR16(CpuRegister::D, CpuRegister16::HL, (1, 8))),         Some(Opcode::Ld(CpuRegister::D, CpuRegister::A, (1, 4))),       Some(Opcode::Ld(CpuRegister::E, CpuRegister::B, (1, 4))), Some(Opcode::Ld(CpuRegister::E, CpuRegister::C, (1, 4))), Some(Opcode::Ld(CpuRegister::E, CpuRegister::D, (1, 4))),       Some(Opcode::Ld(CpuRegister::E, CpuRegister::E, (1, 4))), Some(Opcode::Ld(CpuRegister::E, CpuRegister::H, (1, 4))), Some(Opcode::Ld(CpuRegister::E, CpuRegister::L, (1, 4))), Some(Opcode::LdR16(CpuRegister::E, CpuRegister16::HL, (1, 8))), Some(Opcode::Ld(CpuRegister::E, CpuRegister::A, (1, 4))),
   /* 6 */   Some(Opcode::Ld(CpuRegister::H, CpuRegister::B, (1, 4))),       Some(Opcode::Ld(CpuRegister::H, CpuRegister::C, (1, 4))),       Some(Opcode::Ld(CpuRegister::H, CpuRegister::D, (1, 4))),       Some(Opcode::Ld(CpuRegister::H, CpuRegister::E, (1, 4))),       Some(Opcode::Ld(CpuRegister::H, CpuRegister::H, (1, 4))),       Some(Opcode::Ld(CpuRegister::H, CpuRegister::L, (1, 4))),       Some(Opcode::LdR16(CpuRegister::H, CpuRegister16::HL, (1, 8))),         Some(Opcode::Ld(CpuRegister::H, CpuRegister::A, (1, 4))),       Some(Opcode::Ld(CpuRegister::L, CpuRegister::B, (1, 4))), Some(Opcode::Ld(CpuRegister::L, CpuRegister::C, (1, 4))), Some(Opcode::Ld(CpuRegister::L, CpuRegister::D, (1, 4))),       Some(Opcode::Ld(CpuRegister::L, CpuRegister::E, (1, 4))), Some(Opcode::Ld(CpuRegister::L, CpuRegister::H, (1, 4))), Some(Opcode::Ld(CpuRegister::L, CpuRegister::L, (1, 4))), Some(Opcode::LdR16(CpuRegister::L, CpuRegister16::HL, (1, 8))), Some(Opcode::Ld(CpuRegister::L, CpuRegister::A, (1, 4))),
   /* 7 */   Some(Opcode::Ld16R(CpuRegister16::HL, CpuRegister::B, (1, 8))), Some(Opcode::Ld16R(CpuRegister16::HL, CpuRegister::C, (1, 8))), Some(Opcode::Ld16R(CpuRegister16::HL, CpuRegister::D, (1, 8))), Some(Opcode::Ld16R(CpuRegister16::HL, CpuRegister::E, (1, 8))), Some(Opcode::Ld16R(CpuRegister16::HL, CpuRegister::H, (1, 8))), Some(Opcode::Ld16R(CpuRegister16::HL, CpuRegister::L, (1, 8))), None,                                                                   Some(Opcode::Ld16R(CpuRegister16::HL, CpuRegister::A, (1, 8))), Some(Opcode::Ld(CpuRegister::A, CpuRegister::B, (1, 4))), Some(Opcode::Ld(CpuRegister::A, CpuRegister::C, (1, 4))), Some(Opcode::Ld(CpuRegister::A, CpuRegister::D, (1, 4))),       Some(Opcode::Ld(CpuRegister::A, CpuRegister::E, (1, 4))), Some(Opcode::Ld(CpuRegister::A, CpuRegister::H, (1, 4))), Some(Opcode::Ld(CpuRegister::A, CpuRegister::L, (1, 4))), Some(Opcode::LdR16(CpuRegister::A, CpuRegister16::HL, (1, 8))), Some(Opcode::Ld(CpuRegister::A, CpuRegister::A, (1, 4))),
   /* 8 */   Some(Opcode::Add(CpuRegister::B, (1, 4))),                      Some(Opcode::Add(CpuRegister::C, (1, 4))),                      Some(Opcode::Add(CpuRegister::D, (1, 4))),                      Some(Opcode::Add(CpuRegister::E, (1, 4))),                      Some(Opcode::Add(CpuRegister::H, (1, 4))),                      Some(Opcode::Add(CpuRegister::L, (1, 4))),                      Some(Opcode::AddHL((1, 8))),                                            Some(Opcode::Add(CpuRegister::A, (1, 4))),                      Some(Opcode::Adc(CpuRegister::B, (1, 4))),                Some(Opcode::Adc(CpuRegister::C, (1, 4))),                Some(Opcode::Adc(CpuRegister::D, (1, 4))),                      Some(Opcode::Adc(CpuRegister::E, (1, 4))),                Some(Opcode::Adc(CpuRegister::H, (1, 4))),                Some(Opcode::Adc(CpuRegister::L, (1, 4))),                Some(Opcode::AdcHL((1, 8))),                                    Some(Opcode::Adc(CpuRegister::A, (1, 4))),
   /* 9 */   Some(Opcode::Sub(CpuRegister::B, (1, 4))),                      Some(Opcode::Sub(CpuRegister::C, (1, 4))),                      Some(Opcode::Sub(CpuRegister::D, (1, 4))),                      Some(Opcode::Sub(CpuRegister::E, (1, 4))),                      Some(Opcode::Sub(CpuRegister::H, (1, 4))),                      Some(Opcode::Sub(CpuRegister::L, (1, 4))),                      Some(Opcode::SubHL((1, 8))),                                            Some(Opcode::Sub(CpuRegister::A, (1, 4))),                      Some(Opcode::Sbc(CpuRegister::B, (1, 4))),                Some(Opcode::Sbc(CpuRegister::C, (1, 4))),                Some(Opcode::Sbc(CpuRegister::D, (1, 4))),                      Some(Opcode::Sbc(CpuRegister::E, (1, 4))),                Some(Opcode::Sbc(CpuRegister::H, (1, 4))),                Some(Opcode::Sbc(CpuRegister::L, (1, 4))),                Some(Opcode::SbcHL((1, 8))),                                    Some(Opcode::Sbc(CpuRegister::A, (1, 4))),
   /* A */   Some(Opcode::And(CpuRegister::B, (1, 4))),                      Some(Opcode::And(CpuRegister::C, (1, 4))),                      Some(Opcode::And(CpuRegister::D, (1, 4))),                      Some(Opcode::And(CpuRegister::E, (1, 4))),                      Some(Opcode::And(CpuRegister::H, (1, 4))),                      Some(Opcode::And(CpuRegister::L, (1, 4))),                      Some(Opcode::AndHL((1, 8))),                                            Some(Opcode::And(CpuRegister::A, (1, 4))),                      Some(Opcode::XOr(CpuRegister::B, (1, 4))),                Some(Opcode::XOr(CpuRegister::C, (1, 4))),                Some(Opcode::XOr(CpuRegister::D, (1, 4))),                      Some(Opcode::XOr(CpuRegister::E, (1, 4))),                Some(Opcode::XOr(CpuRegister::H, (1, 4))),                Some(Opcode::XOr(CpuRegister::L, (1, 4))),                Some(Opcode::XOrHL((1, 8))),                                    Some(Opcode::XOr(CpuRegister::A, (1, 4))),
   /* B */   Some(Opcode::Or(CpuRegister::B, (1, 4))),                       Some(Opcode::Or(CpuRegister::C, (1, 4))),                       Some(Opcode::Or(CpuRegister::D, (1, 4))),                       Some(Opcode::Or(CpuRegister::E, (1, 4))),                       Some(Opcode::Or(CpuRegister::H, (1, 4))),                       Some(Opcode::Or(CpuRegister::L, (1, 4))),                       Some(Opcode::OrHL((1, 8))),                                             Some(Opcode::Or(CpuRegister::A, (1, 4))),                       Some(Opcode::Cp(CpuRegister::B, (1, 4))),                 Some(Opcode::Cp(CpuRegister::C, (1, 4))),                 Some(Opcode::Cp(CpuRegister::D, (1, 4))),                       Some(Opcode::Cp(CpuRegister::E, (1, 4))),                 Some(Opcode::Cp(CpuRegister::H, (1, 4))),                 Some(Opcode::Cp(CpuRegister::L, (1, 4))),                 Some(Opcode::CpHL((1, 8))),                                     Some(Opcode::Cp(CpuRegister::A, (1, 4))),
   /* C */   None,                                                           None,                                                           None,                                                           None,                                                           None,                                                           None,                                                           Some(Opcode::AddD8((2, 8))),                                            None,                                                           None,                                                     None,                                                     None,                                                           None,                                                     None,                                                     None,                                                     Some(Opcode::AdcD8((2, 8))),                                    None,
   /* D */   None,                                                           None,                                                           None,                                                           None,                                                           None,                                                           None,                                                           Some(Opcode::SubD8((2, 8))),                                            None,                                                           None,                                                     None,                                                     None,                                                           None,                                                     None,                                                     None,                                                     Some(Opcode::SbcD8((2, 8))),                                    None,
   /* E */   None,                                                           None,                                                           None,                                                           None,                                                           None,                                                           None,                                                           Some(Opcode::AndD8((2, 8))),                                            None,                                                           None,                                                     None,                                                     None,                                                           None,                                                     None,                                                     None,                                                     Some(Opcode::XOrD8((2, 8))),                                    None,
   /* F */   None,                                                           None,                                                           None,                                                           None,                                                           None,                                                           None,                                                           Some(Opcode::OrD8((2, 8))),                                             None,                                                           None,                                                     None,                                                     None,                                                           None,                                                     None,                                                     None,                                                     Some(Opcode::CpD8((2, 8))),                                     None
];