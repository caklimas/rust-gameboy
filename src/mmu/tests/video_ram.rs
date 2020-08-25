use super::super::memory_sizes::KILOBYTES_8;
use super::super::video_ram::VideoRam;

#[test]
fn read_test() {
    let address = 5;
    let data = 5;
    let mut video_ram: VideoRam = Default::default();
    video_ram.write(address, data);

    let actual = video_ram.read(address);
    assert_eq!(data, actual);

    // Testing masking
    let address = KILOBYTES_8 + address;
    let actual = video_ram.read(address);
    assert_eq!(data, actual);
}

#[test]
fn write_test() {
    let address = 5;
    let data = 5;
    let mut video_ram: VideoRam = Default::default();
    video_ram.write(address, data);

    let actual = video_ram.read(address);
    assert_eq!(data, actual);

    // Testing masking
    let data = data + 1;
    let address = KILOBYTES_8 + address;
    video_ram.write(address, data);
    let actual = video_ram.read(address);
    assert_eq!(data, actual);
}