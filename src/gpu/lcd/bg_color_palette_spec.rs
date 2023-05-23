use serde::{Deserialize, Serialize};

bitfield! {
    #[derive(Serialize, Deserialize, Default)]
    pub struct BgColorPaletteSpec(u8);
    impl Debug;

    pub address, set_address: 5, 0;
    pub auto_increment, _: 7;
    pub _, set: 7, 0;
}
