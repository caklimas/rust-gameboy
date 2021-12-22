use serde::{Deserialize, Serialize};

bitfield! {
    #[derive(Serialize, Deserialize, Default)]
    pub struct FrequencyHi(u8);
    impl Debug;

    pub initialize, _: 7;
    pub length_enabled, _: 6;
    pub frequency_higher_bits, _: 2, 0;
}
