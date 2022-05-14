use crate::addresses::gpu::video_ram::VIDEO_RAM_LOWER;
use crate::gpu::video_ram::VideoRam;

#[test]
fn read_test() {
    let address = VIDEO_RAM_LOWER + 5;
    let data = 5;
    let mut video_ram: VideoRam = Default::default();
    video_ram.write(address, data);

    let actual = video_ram.read(address);
    assert_eq!(data, actual);
}

#[test]
fn write_test() {
    let address = VIDEO_RAM_LOWER + 5;
    let data = 5;
    let mut video_ram: VideoRam = Default::default();
    video_ram.write(address, data);

    let actual = video_ram.read(address);
    assert_eq!(data, actual);
}
