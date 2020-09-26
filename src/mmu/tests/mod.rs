pub mod boot_rom;
pub mod high_ram;
pub mod work_ram;

use super::Mmu;

#[test]
fn finish_running_boot_rom_test() {
    let mut mmu: Mmu = Default::default();
    mmu.running_boot_rom = true;

    mmu.finish_running_boot_rom();

    assert_eq!(false, mmu.running_boot_rom);
}