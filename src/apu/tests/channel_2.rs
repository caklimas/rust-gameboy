use crate::{
    addresses::apu::{CHANNEL_2_FREQUENCY_HI_DATA, CHANNEL_2_FREQUENCY_LO_DATA},
    apu::square_channel::SquareChannel,
};

#[test]
fn get_frequency_test() {
    let mut channel_2 = SquareChannel::default();
    channel_2.write(CHANNEL_2_FREQUENCY_LO_DATA, 0b1111_1111);
    channel_2.write(CHANNEL_2_FREQUENCY_HI_DATA, 0b0000_0111);

    let frequency = channel_2.get_frequency();

    assert_eq!(0b0111_1111_1111, frequency);
}
