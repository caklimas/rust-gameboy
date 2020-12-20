use serde::{Serialize, Deserialize};

bitfield!{
    #[derive(Serialize, Deserialize)]
    pub struct Controls(u8);
    impl Debug;
    
    pub input_right_button_a, set_input_right_button_a: 0;
    pub input_left_button_b, set_input_left_button_b: 1;
    pub input_up_select, set_input_up_select: 2;
    pub input_down_start, set_input_down_start: 3;
    pub select_direction_keys, set_select_direction_keys: 4;
    pub select_button_keys, set_select_button_keys: 5;
}

impl Controls {
    pub fn read_byte(&self) -> u8 {
        self.0
    }

    pub fn write_byte(&mut self, data: u8) {
        
    }
}

impl Default for Controls {
    fn default() -> Self {
        Controls(0b0000_1111)
    }
}