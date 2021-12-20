use serde::{Serialize, Deserialize};

bitfield!{
    #[derive(Serialize, Deserialize, Default)]
    pub struct VolumeEnvelope(u8);
    impl Debug;

    pub initial_volume, set_initial_volume: 7, 4;
    pub direction, set_direction: 3;
    pub number_of_envelope_sweep: 2, 0;
}