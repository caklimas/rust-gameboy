use serde::{Deserialize, Serialize};

bitfield! {
    ///Envelope is a way to adjust the volume of a channel periodically.
    #[derive(Serialize, Deserialize, Default)]
    pub struct VolumeEnvelope(u8);
    impl Debug;

    pub initial_volume, _: 7, 4;
    pub direction, _: 3;
    pub initial_envelope_period, _: 2, 0;
}
