use serde::{Deserialize, Serialize};

bitfield! {
    #[derive(Serialize, Deserialize, Default)]
    pub struct SoundOutputTerminalSelection(u8);
    impl Debug;

    pub sound_4_to_s02, _: 7;
    pub sound_3_to_s02, _: 6;
    pub sound_2_to_s02, _: 5;
    pub sound_1_to_s02, _: 4;
    pub sound_4_to_s01, _: 3;
    pub sound_3_to_s01, _: 2;
    pub sound_2_to_s01, _: 1;
    pub sound_1_to_s01, _: 0;
}
