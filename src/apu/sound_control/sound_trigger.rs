use serde::{Deserialize, Serialize};

bitfield! {
    #[derive(Serialize, Deserialize, Default)]
    pub struct SoundTrigger(u8);
    impl Debug;

    pub all_sound_on_off, set_all_sound_on_off: 7;
    pub sound_4_on, _: 3;
    pub sound_3_on, _: 2;
    pub sound_2_on, _: 1;
    pub sound_1_on, _: 0;
}
