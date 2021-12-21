use serde::{Deserialize, Serialize};

bitfield! {
    #[derive(Serialize, Deserialize, Default)]
    pub struct SpriteAttributes(u8);
    impl Debug;

    pub cgb_palette_number, _: 2, 0; // **CGB Mode Only**     (OBP0-7)
    pub tile_vram_bank, _: 3; // **CGB Mode Only**     (0=Bank 0, 1=Bank 1)
    pub palette_number, _: 4; // **Non CGB Mode Only** (0=OBP0, 1=OBP1)
    pub x_flip, _: 5; // (0=Normal, 1=Horizontally mirrored)
    pub y_flip, _: 6; // (0=Normal, 1=Vertically mirrored)
    pub obj_priority, _: 7; // (0=OBJ Above BG, 1=OBJ Behind BG color 1-3)
}
