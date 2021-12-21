use serde::{Deserialize, Serialize};

bitfield! {
    #[derive(Serialize, Deserialize, Default)]
    pub struct CounterConsecutiveSelection(u8);
    impl Debug;

    pub initial, _: 7;
    pub selection, _: 6;
}
