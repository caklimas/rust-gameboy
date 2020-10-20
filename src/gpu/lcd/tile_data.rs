pub struct TileData {
    pub data: u16,
    pub use_unsigned: bool
}

impl TileData {
    pub fn new(data: u16, use_unsigned: bool) -> Self {
        TileData {
            data: data,
            use_unsigned
        }
    }
}