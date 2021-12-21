use serde::{Deserialize, Serialize};

bitfield! {
    #[derive(Serialize, Deserialize, Default)]
    pub struct FlagsRegister(u8);
    impl Debug;

    pub carry, set_carry: 4;
    pub half_carry, set_half_carry: 5;
    pub subtraction, set_subtraction: 6;
    pub zero, set_zero: 7;
    pub get, set: 7, 0;
}
