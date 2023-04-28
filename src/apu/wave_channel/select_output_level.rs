use serde::{Deserialize, Serialize};

bitfield! {
    #[derive(Serialize, Deserialize, Default)]
    pub struct SelectOutputLevel(u8);
    impl Debug;

    pub select_output_level, _: 6, 5;
}
