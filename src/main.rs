#[macro_use]
extern crate bitfield;

#[macro_use]
extern crate serde_big_array;

mod addresses;
mod cpu;
mod mmu;

fn main() {
    let mut cpu: cpu::Cpu = Default::default();
    cpu.clock();
}