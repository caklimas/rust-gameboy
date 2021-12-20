use serde::{Deserialize, Serialize};

bitfield! {
    #[derive(Serialize, Deserialize, Default)]
    pub struct VolumeEnvelope(u8);
    impl Debug;

    pub initial_volume, _: 7, 4;
    pub direction, _: 3;
    pub number_of_envelope_sweep, _: 2, 0;
}
