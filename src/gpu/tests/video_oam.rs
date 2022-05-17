use crate::addresses::gpu::sprite::SPRITE_ATTRIBUTE_TABLE_LOWER;
use crate::gpu::video_oam::VideoOam;

#[test]
fn read_test() {
    let address = SPRITE_ATTRIBUTE_TABLE_LOWER + 5;
    let data = 5;
    let mut video_ram: VideoOam = Default::default();
    video_ram.write(address, data);

    let actual = video_ram.read(address);
    assert_eq!(data, actual);
}

#[test]
fn write_test() {
    let address = SPRITE_ATTRIBUTE_TABLE_LOWER + 5;
    let data = 5;
    let mut video_ram: VideoOam = Default::default();
    video_ram.write(address, data);

    let actual = video_ram.read(address);
    assert_eq!(data, actual);
}
