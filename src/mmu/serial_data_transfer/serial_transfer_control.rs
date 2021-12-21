use serde::{Deserialize, Serialize};

bitfield! {
    #[derive(Serialize, Deserialize, Default)]
    pub struct SerialTransferControl(u8);
    impl Debug;

    pub shift_clock, set_shift_clock: 0;
    pub clock_speed, set_clock_speed: 1;
    pub transfer_start, set_transfer_start: 7;
    pub get, set: 7, 0;
}
