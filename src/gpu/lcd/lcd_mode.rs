use serde::{Serialize, Deserialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum LcdMode {
    HorizontalBlank = 0, // remaining 204 cycles
    VerticalBlank = 1,
    SearchingOam = 2, // cycles 0-80
    Drawing = 3 // cycles 81 - 252
}

impl Default for LcdMode {
    fn default() -> Self {
        LcdMode::SearchingOam
    }
}