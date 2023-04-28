use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default)]
pub struct FrameSequencer {
    pub countdown: u16,
    pub step: u8,
}
