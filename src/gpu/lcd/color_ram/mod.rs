use crate::mmu::memory_sizes::COLOR_RAM;
use serde::{Deserialize, Serialize};
use serde_big_array::BigArray;

#[derive(Serialize, Deserialize)]
pub struct ColorRam {
    #[serde(with = "BigArray")]
    pub data: [u8; COLOR_RAM],
}

impl Default for ColorRam {
    /*
       All background colors are initialized as white by the boot ROM,
       however it is a good idea to initialize all colors yourself, e.g. if implementing a soft-reset mechanic.
    */
    fn default() -> Self {
        Self {
            data: [255; COLOR_RAM],
        }
    }
}
