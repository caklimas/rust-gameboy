pub struct LcdInterruptResult {
    pub lcd_stat: bool,
    pub vertical_blank: bool
}

impl LcdInterruptResult {
    pub fn new() -> Self {
        LcdInterruptResult {
            lcd_stat: false,
            vertical_blank: false
        }
    }
}