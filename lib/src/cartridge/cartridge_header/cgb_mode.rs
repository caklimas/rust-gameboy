use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum CgbMode {
    CgbMonochrome,
    CgbOnly,
    NonCgb,
}

impl CgbMode {
    pub fn new(value: &u8) -> Self {
        match value {
            0x80 => CgbMode::CgbMonochrome,
            0xC0 => CgbMode::CgbOnly,
            _ => CgbMode::NonCgb,
        }
    }
}
