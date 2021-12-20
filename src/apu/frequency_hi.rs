use serde::{Deserialize, Serialize};

bitfield! {
    #[derive(Serialize, Deserialize, Default)]
    pub struct FrequencyHi(u8);
    impl Debug;

    pub initial, _: 7;
    pub counter_consecutive_selection, _: 6;
    pub frequency_higher_bits, _: 2, 0;
}
