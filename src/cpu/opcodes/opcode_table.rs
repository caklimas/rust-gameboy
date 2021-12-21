use super::opcode::{Condition, CpuRegister, CpuRegister16, Opcode};

pub const OPCODE_TABLE: [Option<Opcode>; 256] = [
    /* 0                                                               1                                                           	   2                                                               3                                                               4 														                     5 														                      6 																                      7 														                    8 													                  9 												                      A 														                    B 													                  C 												                      D 													                 E                                                               F */
    /* 0 */
    Some(Opcode::Nop),
    Some(Opcode::LdR16D16(CpuRegister16::BC)),
    Some(Opcode::Ld16R(CpuRegister16::BC, CpuRegister::A)),
    Some(Opcode::Inc16(CpuRegister16::BC)),
    Some(Opcode::IncR(CpuRegister::B)),
    Some(Opcode::DecR(CpuRegister::B)),
    Some(Opcode::LdD8(CpuRegister::B)),
    Some(Opcode::Rlca),
    Some(Opcode::LdA16Sp),
    Some(Opcode::AddHl16(CpuRegister16::BC)),
    Some(Opcode::LdR16(CpuRegister::A, CpuRegister16::BC)),
    Some(Opcode::Dec16(CpuRegister16::BC)),
    Some(Opcode::IncR(CpuRegister::C)),
    Some(Opcode::DecR(CpuRegister::C)),
    Some(Opcode::LdD8(CpuRegister::C)),
    Some(Opcode::Rrca),
    /* 1 */ Some(Opcode::Stop),
    Some(Opcode::LdR16D16(CpuRegister16::DE)),
    Some(Opcode::Ld16R(CpuRegister16::DE, CpuRegister::A)),
    Some(Opcode::Inc16(CpuRegister16::DE)),
    Some(Opcode::IncR(CpuRegister::D)),
    Some(Opcode::DecR(CpuRegister::D)),
    Some(Opcode::LdD8(CpuRegister::D)),
    Some(Opcode::Rla),
    Some(Opcode::Jr),
    Some(Opcode::AddHl16(CpuRegister16::DE)),
    Some(Opcode::LdR16(CpuRegister::A, CpuRegister16::DE)),
    Some(Opcode::Dec16(CpuRegister16::DE)),
    Some(Opcode::IncR(CpuRegister::E)),
    Some(Opcode::DecR(CpuRegister::E)),
    Some(Opcode::LdD8(CpuRegister::E)),
    Some(Opcode::Rra),
    /* 2 */ Some(Opcode::JrCc(Condition::NZ)),
    Some(Opcode::LdR16D16(CpuRegister16::HL)),
    Some(Opcode::LdHlA(true)),
    Some(Opcode::Inc16(CpuRegister16::HL)),
    Some(Opcode::IncR(CpuRegister::H)),
    Some(Opcode::DecR(CpuRegister::H)),
    Some(Opcode::LdD8(CpuRegister::H)),
    Some(Opcode::Daa),
    Some(Opcode::JrCc(Condition::Z)),
    Some(Opcode::AddHl16(CpuRegister16::HL)),
    Some(Opcode::LdAHl(true)),
    Some(Opcode::Dec16(CpuRegister16::HL)),
    Some(Opcode::IncR(CpuRegister::L)),
    Some(Opcode::DecR(CpuRegister::L)),
    Some(Opcode::LdD8(CpuRegister::L)),
    Some(Opcode::Cpl),
    /* 3 */ Some(Opcode::JrCc(Condition::NC)),
    Some(Opcode::LdSpD16),
    Some(Opcode::LdHlA(false)),
    Some(Opcode::IncSp),
    Some(Opcode::IncHl),
    Some(Opcode::DecHl),
    Some(Opcode::LdHlD8),
    Some(Opcode::Scf),
    Some(Opcode::JrCc(Condition::C)),
    Some(Opcode::AddHlSp),
    Some(Opcode::LdAHl(false)),
    Some(Opcode::DecSp),
    Some(Opcode::IncR(CpuRegister::A)),
    Some(Opcode::DecR(CpuRegister::A)),
    Some(Opcode::LdD8(CpuRegister::A)),
    Some(Opcode::Ccf),
    /* 4 */ Some(Opcode::Ld(CpuRegister::B, CpuRegister::B)),
    Some(Opcode::Ld(CpuRegister::B, CpuRegister::C)),
    Some(Opcode::Ld(CpuRegister::B, CpuRegister::D)),
    Some(Opcode::Ld(CpuRegister::B, CpuRegister::E)),
    Some(Opcode::Ld(CpuRegister::B, CpuRegister::H)),
    Some(Opcode::Ld(CpuRegister::B, CpuRegister::L)),
    Some(Opcode::LdR16(CpuRegister::B, CpuRegister16::HL)),
    Some(Opcode::Ld(CpuRegister::B, CpuRegister::A)),
    Some(Opcode::Ld(CpuRegister::C, CpuRegister::B)),
    Some(Opcode::Ld(CpuRegister::C, CpuRegister::C)),
    Some(Opcode::Ld(CpuRegister::C, CpuRegister::D)),
    Some(Opcode::Ld(CpuRegister::C, CpuRegister::E)),
    Some(Opcode::Ld(CpuRegister::C, CpuRegister::H)),
    Some(Opcode::Ld(CpuRegister::C, CpuRegister::L)),
    Some(Opcode::LdR16(CpuRegister::C, CpuRegister16::HL)),
    Some(Opcode::Ld(CpuRegister::C, CpuRegister::A)),
    /* 5 */ Some(Opcode::Ld(CpuRegister::D, CpuRegister::B)),
    Some(Opcode::Ld(CpuRegister::D, CpuRegister::C)),
    Some(Opcode::Ld(CpuRegister::D, CpuRegister::D)),
    Some(Opcode::Ld(CpuRegister::D, CpuRegister::E)),
    Some(Opcode::Ld(CpuRegister::D, CpuRegister::H)),
    Some(Opcode::Ld(CpuRegister::D, CpuRegister::L)),
    Some(Opcode::LdR16(CpuRegister::D, CpuRegister16::HL)),
    Some(Opcode::Ld(CpuRegister::D, CpuRegister::A)),
    Some(Opcode::Ld(CpuRegister::E, CpuRegister::B)),
    Some(Opcode::Ld(CpuRegister::E, CpuRegister::C)),
    Some(Opcode::Ld(CpuRegister::E, CpuRegister::D)),
    Some(Opcode::Ld(CpuRegister::E, CpuRegister::E)),
    Some(Opcode::Ld(CpuRegister::E, CpuRegister::H)),
    Some(Opcode::Ld(CpuRegister::E, CpuRegister::L)),
    Some(Opcode::LdR16(CpuRegister::E, CpuRegister16::HL)),
    Some(Opcode::Ld(CpuRegister::E, CpuRegister::A)),
    /* 6 */ Some(Opcode::Ld(CpuRegister::H, CpuRegister::B)),
    Some(Opcode::Ld(CpuRegister::H, CpuRegister::C)),
    Some(Opcode::Ld(CpuRegister::H, CpuRegister::D)),
    Some(Opcode::Ld(CpuRegister::H, CpuRegister::E)),
    Some(Opcode::Ld(CpuRegister::H, CpuRegister::H)),
    Some(Opcode::Ld(CpuRegister::H, CpuRegister::L)),
    Some(Opcode::LdR16(CpuRegister::H, CpuRegister16::HL)),
    Some(Opcode::Ld(CpuRegister::H, CpuRegister::A)),
    Some(Opcode::Ld(CpuRegister::L, CpuRegister::B)),
    Some(Opcode::Ld(CpuRegister::L, CpuRegister::C)),
    Some(Opcode::Ld(CpuRegister::L, CpuRegister::D)),
    Some(Opcode::Ld(CpuRegister::L, CpuRegister::E)),
    Some(Opcode::Ld(CpuRegister::L, CpuRegister::H)),
    Some(Opcode::Ld(CpuRegister::L, CpuRegister::L)),
    Some(Opcode::LdR16(CpuRegister::L, CpuRegister16::HL)),
    Some(Opcode::Ld(CpuRegister::L, CpuRegister::A)),
    /* 7 */ Some(Opcode::Ld16R(CpuRegister16::HL, CpuRegister::B)),
    Some(Opcode::Ld16R(CpuRegister16::HL, CpuRegister::C)),
    Some(Opcode::Ld16R(CpuRegister16::HL, CpuRegister::D)),
    Some(Opcode::Ld16R(CpuRegister16::HL, CpuRegister::E)),
    Some(Opcode::Ld16R(CpuRegister16::HL, CpuRegister::H)),
    Some(Opcode::Ld16R(CpuRegister16::HL, CpuRegister::L)),
    Some(Opcode::Halt),
    Some(Opcode::Ld16R(CpuRegister16::HL, CpuRegister::A)),
    Some(Opcode::Ld(CpuRegister::A, CpuRegister::B)),
    Some(Opcode::Ld(CpuRegister::A, CpuRegister::C)),
    Some(Opcode::Ld(CpuRegister::A, CpuRegister::D)),
    Some(Opcode::Ld(CpuRegister::A, CpuRegister::E)),
    Some(Opcode::Ld(CpuRegister::A, CpuRegister::H)),
    Some(Opcode::Ld(CpuRegister::A, CpuRegister::L)),
    Some(Opcode::LdR16(CpuRegister::A, CpuRegister16::HL)),
    Some(Opcode::Ld(CpuRegister::A, CpuRegister::A)),
    /* 8 */ Some(Opcode::Add(CpuRegister::B)),
    Some(Opcode::Add(CpuRegister::C)),
    Some(Opcode::Add(CpuRegister::D)),
    Some(Opcode::Add(CpuRegister::E)),
    Some(Opcode::Add(CpuRegister::H)),
    Some(Opcode::Add(CpuRegister::L)),
    Some(Opcode::AddAHl),
    Some(Opcode::Add(CpuRegister::A)),
    Some(Opcode::Adc(CpuRegister::B)),
    Some(Opcode::Adc(CpuRegister::C)),
    Some(Opcode::Adc(CpuRegister::D)),
    Some(Opcode::Adc(CpuRegister::E)),
    Some(Opcode::Adc(CpuRegister::H)),
    Some(Opcode::Adc(CpuRegister::L)),
    Some(Opcode::AdcHl),
    Some(Opcode::Adc(CpuRegister::A)),
    /* 9 */ Some(Opcode::Sub(CpuRegister::B)),
    Some(Opcode::Sub(CpuRegister::C)),
    Some(Opcode::Sub(CpuRegister::D)),
    Some(Opcode::Sub(CpuRegister::E)),
    Some(Opcode::Sub(CpuRegister::H)),
    Some(Opcode::Sub(CpuRegister::L)),
    Some(Opcode::SubHl),
    Some(Opcode::Sub(CpuRegister::A)),
    Some(Opcode::Sbc(CpuRegister::B)),
    Some(Opcode::Sbc(CpuRegister::C)),
    Some(Opcode::Sbc(CpuRegister::D)),
    Some(Opcode::Sbc(CpuRegister::E)),
    Some(Opcode::Sbc(CpuRegister::H)),
    Some(Opcode::Sbc(CpuRegister::L)),
    Some(Opcode::SbcHl),
    Some(Opcode::Sbc(CpuRegister::A)),
    /* A */ Some(Opcode::And(CpuRegister::B)),
    Some(Opcode::And(CpuRegister::C)),
    Some(Opcode::And(CpuRegister::D)),
    Some(Opcode::And(CpuRegister::E)),
    Some(Opcode::And(CpuRegister::H)),
    Some(Opcode::And(CpuRegister::L)),
    Some(Opcode::AndHl),
    Some(Opcode::And(CpuRegister::A)),
    Some(Opcode::XOr(CpuRegister::B)),
    Some(Opcode::XOr(CpuRegister::C)),
    Some(Opcode::XOr(CpuRegister::D)),
    Some(Opcode::XOr(CpuRegister::E)),
    Some(Opcode::XOr(CpuRegister::H)),
    Some(Opcode::XOr(CpuRegister::L)),
    Some(Opcode::XOrHl),
    Some(Opcode::XOr(CpuRegister::A)),
    /* B */ Some(Opcode::Or(CpuRegister::B)),
    Some(Opcode::Or(CpuRegister::C)),
    Some(Opcode::Or(CpuRegister::D)),
    Some(Opcode::Or(CpuRegister::E)),
    Some(Opcode::Or(CpuRegister::H)),
    Some(Opcode::Or(CpuRegister::L)),
    Some(Opcode::OrHl),
    Some(Opcode::Or(CpuRegister::A)),
    Some(Opcode::Cp(CpuRegister::B)),
    Some(Opcode::Cp(CpuRegister::C)),
    Some(Opcode::Cp(CpuRegister::D)),
    Some(Opcode::Cp(CpuRegister::E)),
    Some(Opcode::Cp(CpuRegister::H)),
    Some(Opcode::Cp(CpuRegister::L)),
    Some(Opcode::CpHl),
    Some(Opcode::Cp(CpuRegister::A)),
    /* C */ Some(Opcode::RetCc(Condition::NZ)),
    Some(Opcode::Pop(CpuRegister16::BC)),
    Some(Opcode::JpCc(Condition::NZ)),
    Some(Opcode::Jp),
    Some(Opcode::CallCc(Condition::NZ)),
    Some(Opcode::Push(CpuRegister16::BC)),
    Some(Opcode::AddD8),
    Some(Opcode::Rst(0x00)),
    Some(Opcode::RetCc(Condition::Z)),
    Some(Opcode::Ret),
    Some(Opcode::JpCc(Condition::Z)),
    Some(Opcode::Cb),
    Some(Opcode::CallCc(Condition::Z)),
    Some(Opcode::Call),
    Some(Opcode::AdcD8),
    Some(Opcode::Rst(0x08)),
    /* D */ Some(Opcode::RetCc(Condition::NC)),
    Some(Opcode::Pop(CpuRegister16::DE)),
    Some(Opcode::JpCc(Condition::NC)),
    None,
    Some(Opcode::CallCc(Condition::NC)),
    Some(Opcode::Push(CpuRegister16::DE)),
    Some(Opcode::SubD8),
    Some(Opcode::Rst(0x10)),
    Some(Opcode::RetCc(Condition::C)),
    Some(Opcode::RetI),
    Some(Opcode::JpCc(Condition::C)),
    None,
    Some(Opcode::CallCc(Condition::C)),
    None,
    Some(Opcode::SbcD8),
    Some(Opcode::Rst(0x18)),
    /* E */ Some(Opcode::LdA8A),
    Some(Opcode::Pop(CpuRegister16::HL)),
    Some(Opcode::LdCA),
    None,
    None,
    Some(Opcode::Push(CpuRegister16::HL)),
    Some(Opcode::AndD8),
    Some(Opcode::Rst(0x20)),
    Some(Opcode::AddSpE8),
    Some(Opcode::JpHl),
    Some(Opcode::LdA16A),
    None,
    None,
    None,
    Some(Opcode::XOrD8),
    Some(Opcode::Rst(0x28)),
    /* F */ Some(Opcode::LdAA8),
    Some(Opcode::Pop(CpuRegister16::AF)),
    Some(Opcode::LdAC),
    Some(Opcode::Di),
    None,
    Some(Opcode::Push(CpuRegister16::AF)),
    Some(Opcode::OrD8),
    Some(Opcode::Rst(0x30)),
    Some(Opcode::LdHlSpE8),
    Some(Opcode::LdSpHl),
    Some(Opcode::LdAA16),
    Some(Opcode::Ei),
    None,
    None,
    Some(Opcode::CpD8),
    Some(Opcode::Rst(0x38)),
];
