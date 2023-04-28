pub fn mix_audio(input: f32, src: f32, volume: u16) -> f32 {
    let max_volume = 1.0 / 128.0_f32;
    let max_audio = f32::MAX as f64;
    let min_audio = f32::MIN as f64;
    let src1 = src * volume as f32 * max_volume;

    let mut mixed = src1 as f64 + input as f64;
    if mixed > max_audio {
        mixed = max_audio;
    } else if mixed < min_audio {
        mixed = min_audio;
    }

    mixed as f32
}
