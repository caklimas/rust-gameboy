use crate::addresses::gpu::lcd::*;
use super::bg_palette_data::BgPaletteData;
use super::Lcd;
use super::obj_palette_data::ObjPaletteData;

#[test]
fn read_bg_palette_test() {
    let data = 0b1100_1001;
    let mut lcd: Lcd = Default::default();
    lcd.bg_palette_data = BgPaletteData::from_u8(data);

    let result = lcd.read(LCD_BG_PALETTE_DATA);

    assert_eq!(data, result);
}

#[test]
fn read_control_test() {
    let data = 5;
    let mut lcd: Lcd = Default::default();
    lcd.control.set(data);

    let result = lcd.read(LCD_CONTROL);

    assert_eq!(data, result);
}

#[test]
fn read_ly_test() {
    let data = 0b1100_1001;
    let mut lcd: Lcd = Default::default();
    lcd.line_number = data;

    let result = lcd.read(LCD_LY);

    assert_eq!(data, result);
}

#[test]
fn read_lyc_test() {
    let data = 0b1100_1001;
    let mut lcd: Lcd = Default::default();
    lcd.lyc = data;

    let result = lcd.read(LCD_LYC);

    assert_eq!(data, result);
}

#[test]
fn read_obj_palette_0_test() {
    let data = 0b1100_1000;
    let mut lcd: Lcd = Default::default();
    lcd.obj_palette_0_data = ObjPaletteData::from_u8(data);

    let result = lcd.read(LCD_OBJ_0_PALETTE_DATA);

    assert_eq!(data, result);
}

#[test]
fn read_obj_palette_1_test() {
    let data = 0b1100_1000;
    let mut lcd: Lcd = Default::default();
    lcd.obj_palette_1_data = ObjPaletteData::from_u8(data);

    let result = lcd.read(LCD_OBJ_1_PALETTE_DATA);

    assert_eq!(data, result);
}

#[test]
fn read_scroll_x_test() {
    let data = 5;
    let mut lcd: Lcd = Default::default();
    lcd.scroll_x = data;

    let result = lcd.read(LCD_SCROLL_X);

    assert_eq!(data, result);
}

#[test]
fn read_scroll_y_test() {
    let data = 5;
    let mut lcd: Lcd = Default::default();
    lcd.scroll_y = data;

    let result = lcd.read(LCD_SCROLL_Y);

    assert_eq!(data, result);
}

#[test]
fn read_status_test() {
    let data = 0b0111_1000;
    let mut lcd: Lcd = Default::default();
    lcd.status.set(data);

    let result = lcd.read(LCD_STATUS);

    assert_eq!(data, result);
    assert_eq!(false, lcd.status.line_coincidence());
}

#[test]
fn read_window_x_test() {
    let data = 5;
    let mut lcd: Lcd = Default::default();
    lcd.window_x = data;

    let result = lcd.read(LCD_WINDOW_X);

    assert_eq!(data, result);
}

#[test]
fn read_window_y_test() {
    let data = 5;
    let mut lcd: Lcd = Default::default();
    lcd.window_y = data;

    let result = lcd.read(LCD_WINDOW_Y);

    assert_eq!(data, result);
}

#[test]
fn write_bg_palette_test() {
    let data = 0b1100_1001;
    let mut lcd: Lcd = Default::default();

    lcd.write(LCD_BG_PALETTE_DATA, data);

    assert_eq!(lcd.bg_palette_data.into_u8(), data);
}

#[test]
fn write_control_test() {
    let data = 5;
    let mut lcd: Lcd = Default::default();

    lcd.write(LCD_CONTROL, data);

    assert_eq!(data, lcd.control.get());
}

#[test]
fn write_ly_test() {
    let data = 0b1100_1001;
    let mut lcd: Lcd = Default::default();

    lcd.write(LCD_LY, data);

    assert_ne!(data, lcd.line_number);
}

#[test]
fn write_lyc_test() {
    let data = 0b1100_1001;
    let mut lcd: Lcd = Default::default();

    lcd.write(LCD_LYC, data);

    assert_eq!(data, lcd.lyc);
}

#[test]
fn write_obj_palette_0_test() {
    let data = 0b1100_1000;
    let mut lcd: Lcd = Default::default();

    lcd.write(LCD_OBJ_0_PALETTE_DATA, data);

    assert_eq!(lcd.obj_palette_0_data.into_u8(), data);
}

#[test]
fn write_obj_palette_1_test() {
    let data = 0b1101_1000;
    let mut lcd: Lcd = Default::default();

    lcd.write(LCD_OBJ_1_PALETTE_DATA, data);

    assert_eq!(lcd.obj_palette_1_data.into_u8(), data);
}

#[test]
fn write_scroll_x_test() {
    let data = 5;
    let mut lcd: Lcd = Default::default();

    lcd.write(LCD_SCROLL_X, data);

    assert_eq!(data, lcd.scroll_x);
}

#[test]
fn write_scroll_y_test() {
    let data = 5;
    let mut lcd: Lcd = Default::default();

    lcd.write(LCD_SCROLL_Y, data);

    assert_eq!(data, lcd.scroll_y);
}

#[test]
fn write_status_test() {
    let data = 0b0111_1000;
    let mut lcd: Lcd = Default::default();
    lcd.lyc = 1;
    lcd.line_number = lcd.lyc + 2;

    lcd.write(LCD_STATUS, data);

    assert_eq!(data, lcd.status.get());
    assert_eq!(false, lcd.status.line_coincidence());

    lcd.lyc = 2;
    lcd.line_number = lcd.lyc;
    lcd.write(LCD_STATUS, data);

    assert_eq!(true, lcd.status.line_coincidence());
}

#[test]
fn write_window_x_test() {
    let data = 5;
    let mut lcd: Lcd = Default::default();

    lcd.write(LCD_WINDOW_X, data);

    assert_eq!(data, lcd.window_x);
}

#[test]
fn write_window_y_test() {
    let data = 5;
    let mut lcd: Lcd = Default::default();

    lcd.write(LCD_WINDOW_Y, data);

    assert_eq!(data, lcd.window_y);
}