use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize, Default)]
pub enum LcdMode {
    HorizontalBlank = 0, // remaining 204 cycles
    VerticalBlank = 1,
    #[default]
    SearchingOam = 2, // cycles 0-80
    Drawing = 3, // cycles 81 - 252
}
