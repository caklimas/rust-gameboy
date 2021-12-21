use crate::gpu::video_oam::VideoOam;
use crate::mmu::memory_sizes::VIDEO_OAM;

#[test]
fn read_test() {
    let address = 5;
    let data = 5;
    let mut video_ram: VideoOam = Default::default();
    video_ram.write(address, data);

    let actual = video_ram.read(address);
    assert_eq!(data, actual);

    // Testing masking
    let address = (VIDEO_OAM as u16) + address;
    let actual = video_ram.read(address);
    assert_eq!(data, actual);
}

#[test]
fn write_test() {
    let address = 5;
    let data = 5;
    let mut video_ram: VideoOam = Default::default();
    video_ram.write(address, data);

    let actual = video_ram.read(address);
    assert_eq!(data, actual);

    // Testing masking
    let data = data + 1;
    let address = (VIDEO_OAM as u16) + address;
    video_ram.write(address, data);
    let actual = video_ram.read(address);
    assert_eq!(data, actual);
}
