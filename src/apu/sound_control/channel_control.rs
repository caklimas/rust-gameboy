use serde::{Deserialize, Serialize};

bitfield! {
    #[derive(Serialize, Deserialize, Default)]
    pub struct ChannelControl(u8);
    impl Debug;

    pub output_vin_to_s02_terminal, _: 7;
    pub s02_output_level, _: 6, 4;
    pub output_vin_to_s01_terminal, _: 3;
    pub s01_output_level, _: 2, 0;
}
