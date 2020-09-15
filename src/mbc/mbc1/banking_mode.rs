#[derive(Debug, PartialEq)]
pub enum BankingMode {
    Rom,
    Ram
}

impl BankingMode {
    pub fn new(value: u8) -> Self {
        match value & 0x01 {
            0 => BankingMode::Rom,
            1 => BankingMode::Ram,
            _ => panic!("Invalid banking mode")
        } 
    }
}