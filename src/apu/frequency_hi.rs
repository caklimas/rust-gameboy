use serde::{Deserialize, Serialize};

bitfield! {
    #[derive(Serialize, Deserialize, Default)]
    pub struct VolumeEnvelope(u8);
    impl Debug;

    pub initial, set_initial: 7,
    pub counter_consecutive_selection: 6;
    pub frequency_higher_bits: 2, 0;
}
