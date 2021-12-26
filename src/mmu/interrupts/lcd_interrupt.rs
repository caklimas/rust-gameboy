#[derive(Default)]
pub struct LcdInterruptResult {
    pub lcd_stat: bool,
    pub vertical_blank: bool,
}
