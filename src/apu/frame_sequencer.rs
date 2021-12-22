use serde::{Deserialize, Serialize};

const FRAME_SEQUENCE_COUNTDOWN_TICKS: u16 = 8192;

#[derive(Serialize, Deserialize, Default)]
pub struct FrameSequencer {
    pub countdown: u16,
    pub step: u8,
}
