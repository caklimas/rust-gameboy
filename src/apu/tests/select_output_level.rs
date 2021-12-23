use crate::apu::wave_channel::select_output_level::SelectOutputLevel;

#[test]
fn get_output_level_test() {
    let select_output_level = SelectOutputLevel(0b0000_0000);
    let output_level = select_output_level.get_output_level();

    assert_eq!(0.0, output_level);

    let select_output_level = SelectOutputLevel(0b0010_0000);
    let output_level = select_output_level.get_output_level();

    assert_eq!(1.0, output_level);

    let select_output_level = SelectOutputLevel(0b0100_0000);
    let output_level = select_output_level.get_output_level();

    assert_eq!(0.5, output_level);

    let select_output_level = SelectOutputLevel(0b0110_0000);
    let output_level = select_output_level.get_output_level();

    assert_eq!(0.25, output_level);
}
