use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum BankingMode {
    Ram,
    Rtc,
}
