use crate::apu::sound_length_wave_pattern::SoundLengthWavePattern;

#[test]
fn ratio_test() {
    let wave_pattern = SoundLengthWavePattern(0b0011_1111);

    assert_eq!(0.125, wave_pattern.get_ratio());

    let wave_pattern = SoundLengthWavePattern(0b0111_1111);

    assert_eq!(0.25, wave_pattern.get_ratio());

    let wave_pattern = SoundLengthWavePattern(0b1011_1111);

    assert_eq!(0.50, wave_pattern.get_ratio());

    let wave_pattern = SoundLengthWavePattern(0b1111_1111);

    assert_eq!(0.75, wave_pattern.get_ratio());
}