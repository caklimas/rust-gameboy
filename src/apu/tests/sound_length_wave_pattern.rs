use crate::apu::sound_length_wave_pattern::SoundLengthWavePattern;

#[test]
fn sound_length_test() {
    let wave_pattern = SoundLengthWavePattern(0b1100_1111);

    assert_eq!(64 - 15, wave_pattern.get_sound_length());
}
