use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub struct RomConfig {
    pub run_boot_rom: bool,
}

#[wasm_bindgen]
impl RomConfig {
    #[wasm_bindgen(constructor)]
    pub fn new(run_boot_rom: bool) -> Self {
        Self { run_boot_rom }
    }
}
