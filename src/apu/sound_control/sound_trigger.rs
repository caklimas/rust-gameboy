use serde::{Deserialize, Serialize};

bitfield! {
    #[derive(Serialize, Deserialize, Default)]
    pub struct SoundTrigger(u8);
    impl Debug;

    pub all_sound_on_off, set_all_sound_on_off: 7;
}
