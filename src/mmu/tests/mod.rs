pub mod boot_rom;
pub mod high_ram;
pub mod work_ram;

use super::Mmu;

#[test]
fn finish_running_boot_rom_test() {
    let mut mmu: Mmu = Default::default();
    mmu.run_boot_rom = true;
    mmu.boot_rom_finished = true;

    mmu.finish_running_boot_rom();

    assert_eq!(false, mmu.run_boot_rom);
}

#[test]
#[should_panic]
fn finish_running_boot_rom_panic_test() {
    let mut mmu: Mmu = Default::default();
    mmu.run_boot_rom = true;
    mmu.boot_rom_finished = false;

    mmu.finish_running_boot_rom();
}