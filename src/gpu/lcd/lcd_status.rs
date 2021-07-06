use serde::{Serialize, Deserialize};

bitfield! {
    #[derive(Serialize, Deserialize, Default)]
    pub struct LcdStatus(u8);
    impl Debug;

    pub line_coincidence, set_line_coincidence: 2;
    pub horizontal_blank_interrupt, _: 3;
    pub vertical_blank_interrupt, _: 4;
    pub oam_interrupt, _: 5;
    pub line_coincidence_interrupt, _: 6;
    pub get, set: 6, 0;
}