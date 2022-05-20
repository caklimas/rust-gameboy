use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct WaveRam {
    data: [u8; 16],
}

impl WaveRam {
    pub fn read(&self, index: usize) -> u8 {
        self.data[index]
    }

    pub fn write(&mut self, index: usize, value: u8) {
        println!("Write to wave ram: {}, {}", index, value);
        self.data[index] = value;
    }
}

impl Default for WaveRam {
    fn default() -> Self {
        Self {
            data: [
                0x00, 0xFF, 0x00, 0xFF, 0x00, 0xFF, 0x00, 0xFF, 0x00, 0xFF, 0x00, 0xFF, 0x00, 0xFF,
                0x00, 0xFF,
            ],
        }
    }
}
