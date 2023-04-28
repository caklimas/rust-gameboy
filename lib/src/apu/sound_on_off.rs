use serde::{Deserialize, Serialize};

bitfield! {
    #[derive(Serialize, Deserialize, Default)]
    pub struct SoundOnOff(u8);
    impl Debug;

    pub sound_channel_3_on, _: 7;
}
