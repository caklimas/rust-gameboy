use crate::controls::*;

#[test]
fn read_byte_test() {
    let mut controls: Controls = Default::default();
    let mut result = controls.read_byte();

    assert_eq!(0b0011_1111, result);

    controls.select_button_keys = false;
    controls.button_keys = 0;

    result = controls.read_byte();
    assert_eq!(0b0001_1111, result);

    controls.select_button_keys = true;
    controls.select_direction_keys = false;

    result = controls.read_byte();
    assert_eq!(0b0010_0000, result);
}

#[test]
fn write_byte_test() {
    let mut controls: Controls = Default::default();
    controls.write_byte(0);

    assert_eq!(true, controls.select_button_keys);
    assert_eq!(true, controls.select_direction_keys);

    controls.write_byte(SELECT_BUTTON_BIT);

    assert_eq!(false, controls.select_button_keys);
    assert_eq!(true, controls.select_direction_keys);

    controls.write_byte(SELECT_DIRECTION_BIT);

    assert_eq!(true, controls.select_button_keys);
    assert_eq!(false, controls.select_direction_keys);
}
