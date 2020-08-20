pub mod flags_register;
pub mod index_registers;
pub mod opcodes;
pub mod registers;

#[cfg(test)]
mod registers_tests;

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Cpu {
    index_registers: index_registers::IndexRegisters,
    interrupt_page_address: u8,
    memory_refresh: u8,
    program_counter: u16,
    registers: registers::Registers,
    stack_pointer: u16
}