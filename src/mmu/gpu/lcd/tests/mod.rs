use crate::addresses::gpu::lcd::*;
use super::bg_palette_data::BgPaletteData;
use super::Lcd;

#[test]
pub fn read_bg_palette_test() {
    let data = 0b1100_1001;
    let mut lcd: Lcd = Default::default();
    lcd.bg_palette_data = BgPaletteData::from_u8(data);

    let result = lcd.read(LCD_BG_PALETTE_DATA);

    assert_eq!(data, result);
}

#[test]
pub fn read_control_test() {
    let data = 5;
    let mut lcd: Lcd = Default::default();
    lcd.control.set(data);

    let result = lcd.read(LCD_CONTROL);

    assert_eq!(data, result);
}

#[test]
pub fn read_scroll_y_test() {
    let data = 5;
    let mut lcd: Lcd = Default::default();
    lcd.scroll_y = data;

    let result = lcd.read(LCD_SCROLL_Y);

    assert_eq!(data, result);
}

#[test]
pub fn write_bg_palette_test() {
    let data = 0b1100_1001;
    let mut lcd: Lcd = Default::default();

    lcd.write(LCD_BG_PALETTE_DATA, data);

    assert_eq!(lcd.bg_palette_data.into_u8(), data);
}

#[test]
pub fn write_scroll_y_test() {
    let data = 5;
    let mut lcd: Lcd = Default::default();

    lcd.write(LCD_SCROLL_Y, data);

    assert_eq!(data, lcd.scroll_y);
}

#[test]
pub fn write_control_test() {
    let data = 5;
    let mut lcd: Lcd = Default::default();

    lcd.write(LCD_CONTROL, data);

    assert_eq!(data, lcd.control.get());
}