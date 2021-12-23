use serde::{Deserialize, Serialize};

pub const ENVELOPE_PERIOD_MAX: u8 = 8;
pub const ENVELOPE_VOLUME_MIN: u8 = 0;
pub const ENVELOPE_VOLUME_MAX: u8 = 15;

bitfield! {
    ///Envelope is a way to adjust the volume of a channel periodically.
    #[derive(Serialize, Deserialize, Default)]
    pub struct VolumeEnvelope(u8);
    impl Debug;

    pub initial_volume, _: 7, 4;
    pub direction, _: 3;
    pub initial_envelope_period, _: 2, 0;
}
