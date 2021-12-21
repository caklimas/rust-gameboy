use serde::{Deserialize, Serialize};

bitfield! {
    #[derive(Serialize, Deserialize, Default)]
    pub struct PolynomialCounter(u8);
    impl Debug;

    pub shift_clock_frequency, _: 7, 4;
    pub counter_step_width, _: 3;
    pub frequency_dividing_ratio, _: 2, 0;
}
