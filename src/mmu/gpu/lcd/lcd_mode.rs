use serde::{Serialize, Deserialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum LcdMode {
    HorizontalBlank = 0,
    VerticalBlank = 1,
    SearchingOam = 2,
    TransferringDriver = 3
}

impl Default for LcdMode {
    fn default() -> Self {
        LcdMode::HorizontalBlank
    }
}