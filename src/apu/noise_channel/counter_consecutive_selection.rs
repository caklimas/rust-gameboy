use serde::{Deserialize, Serialize};

bitfield! {
    #[derive(Serialize, Deserialize, Default)]
    pub struct CounterConsecutiveSelection(u8);
    impl Debug;

    pub initialize, _: 7;
    pub length_enabled, _: 6;
}
